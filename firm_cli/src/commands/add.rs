use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use convert_case::{Case, Casing};
use firm_core::{Entity, EntitySchema, compose_entity_id};
use firm_lang::generate::generate_dsl;
use firm_lang::workspace::Workspace;
use inquire::{Confirm, Select, Text};

use super::{
    build_graph, build_workspace, field_prompt::prompt_for_field_value, load_workspace_files,
};
use crate::errors::CliError;
use crate::ui::{self, OutputFormat};

pub const GENERATED_DIR_NAME: &str = "generated";
pub const FIRM_EXTENSION: &str = "firm";

// Wrapper for EntitySchema that customizes Display for Inquire
struct InquireSchema<'a>(&'a EntitySchema);
impl<'a> std::fmt::Display for InquireSchema<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.entity_type)
    }
}

pub fn add_entity(
    workspace_path: &PathBuf,
    to_file: Option<PathBuf>,
    output_format: OutputFormat,
) -> Result<(), CliError> {
    ui::header("Adding new entity");
    let mut workspace = Workspace::new();
    load_workspace_files(&workspace_path, &mut workspace).map_err(|_| CliError::BuildError)?;
    let build = build_workspace(workspace).map_err(|_| CliError::BuildError)?;
    let graph = build_graph(&build)?;

    let mut sorted_schemas = build.schemas.clone();
    sorted_schemas.sort_by_key(|schema| schema.entity_type.to_string());
    let schema_options: Vec<_> = sorted_schemas.iter().map(InquireSchema).collect();
    let chosen_option = Select::new("Type:", schema_options)
        .prompt()
        .map_err(|_| CliError::InputError)?;

    let chosen_schema = chosen_option.0.clone();
    let chosen_type_str = format!("{}", &chosen_schema.entity_type);
    let mut chosen_id = Text::new("ID:")
        .prompt()
        .map_err(|_| CliError::InputError)?;

    // Make a unique ID for the entity based on the name
    chosen_id = chosen_id
        .chars()
        .filter(|&c| c == ' ' || c.is_alphabetic())
        .collect::<String>()
        .to_case(Case::Snake);
    let mut entity_id = chosen_id.clone();
    let mut id_counter = 1;
    while graph
        .get_entity(&compose_entity_id(&chosen_type_str, &entity_id))
        .is_some()
    {
        entity_id = format!("{}_{}", chosen_id, id_counter);
        id_counter += 1;
    }

    // Create the initial entity
    let mut entity = Entity::new(entity_id.into(), chosen_schema.entity_type);

    // Collect required fields
    let mut required_fields: Vec<_> = chosen_schema
        .fields
        .iter()
        .filter(|(_, f)| f.is_required())
        .collect();
    required_fields.sort_by_key(|(field_id, _)| field_id.as_str());

    let arc_graph = Arc::new(graph.clone());
    for (field_id, field) in required_fields {
        match prompt_for_field_value(
            field_id,
            field.expected_type(),
            field.is_required(),
            Arc::clone(&arc_graph),
        )? {
            Some(value) => {
                entity = entity.with_field(field_id.clone(), value);
            }
            None => {}
        }
    }

    let add_optional = Confirm::new("Add optional fields?")
        .with_default(false)
        .prompt()
        .map_err(|_| CliError::InputError)?;

    if add_optional {
        // Collect optional fields
        let mut optional_fields: Vec<_> = chosen_schema
            .fields
            .iter()
            .filter(|(_, f)| !f.is_required())
            .collect();
        optional_fields.sort_by_key(|(field_id, _)| field_id.as_str());

        for (field_id, field) in optional_fields {
            match prompt_for_field_value(
                field_id,
                field.expected_type(),
                field.is_required(),
                Arc::clone(&arc_graph),
            )? {
                Some(value) => {
                    entity = entity.with_field(field_id.clone(), value);
                }
                None => {}
            }
        }
    }

    let generated_dsl = generate_dsl(&[entity.clone()]);

    // Use target file if provided, otherwise create a sensible default path
    let generated_file_path = match to_file {
        Some(file_path) => workspace_path
            .join(file_path)
            .with_extension(FIRM_EXTENSION),
        None => workspace_path
            .join(GENERATED_DIR_NAME)
            .join(&chosen_type_str)
            .with_extension(FIRM_EXTENSION),
    };

    ui::info(&format!(
        "Writing generated DSL to file {}",
        generated_file_path.display()
    ));

    // Create parent directory if it doesn't exist
    if let Some(parent) = generated_file_path.parent() {
        fs::create_dir_all(parent).map_err(|_| CliError::FileError)?;
    }

    match File::options()
        .create(true)
        .append(true)
        .open(generated_file_path)
    {
        Ok(mut file) => match file.write_all(&generated_dsl.into_bytes()) {
            Ok(_) => {
                ui::success(&format!("Generated DSL for '{}'", &entity.id));

                match output_format {
                    OutputFormat::Pretty => ui::pretty_output_entity_single(&entity),
                    OutputFormat::Json => ui::json_output(&entity),
                }
                Ok(())
            }
            Err(e) => {
                ui::error_with_details("Couldn't write to file", &e.to_string());
                Err(CliError::FileError)
            }
        },
        Err(e) => {
            ui::error_with_details("Couldn't open file", &e.to_string());
            Err(CliError::FileError)
        }
    }
}
