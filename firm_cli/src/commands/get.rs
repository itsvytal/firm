use std::path::PathBuf;

use firm_core::compose_entity_id;
use firm_lang::workspace::Workspace;

use crate::errors::CliError;
use crate::files::load_current_graph;
use crate::query::CliDirection;
use crate::ui::{self, OutputFormat};

use super::{build_workspace, load_workspace_files};

pub fn get_entity_by_id(
    workspace_path: &PathBuf,
    entity_type: String,
    entity_id: String,
    output_format: OutputFormat,
) -> Result<(), CliError> {
    ui::header("Getting entity by ID");
    let graph = load_current_graph(&workspace_path)?;

    let id = compose_entity_id(&entity_type, &entity_id);
    match graph.get_entity(&id) {
        Some(entity) => {
            ui::success(&format!(
                "Found '{}' entity with ID '{}'",
                entity_type, entity_id
            ));

            match output_format {
                ui::OutputFormat::Pretty => ui::pretty_output_entity_single(entity),
                ui::OutputFormat::Json => ui::json_output(entity),
            }
        }
        None => {
            ui::error(&format!(
                "Couldn't find '{}' entity with ID '{}'",
                entity_type, entity_id
            ));

            return Err(CliError::QueryError);
        }
    }

    Ok(())
}

pub fn get_related_entities(
    workspace_path: &PathBuf,
    entity_type: String,
    entity_id: String,
    direction: Option<CliDirection>,
    output_format: OutputFormat,
) -> Result<(), CliError> {
    ui::header("Getting related entities");
    let graph = load_current_graph(&workspace_path)?;

    let id = compose_entity_id(&entity_type, &entity_id);
    match graph.get_related(&id, direction.clone().map(|d| d.into())) {
        Some(entities) => {
            let direction_text = match direction {
                Some(CliDirection::To) => "references to",
                Some(CliDirection::From) => "references from",
                None => "relationships for",
            };

            ui::success(&format!(
                "Found {} {} '{}' entity with ID '{}'",
                entities.len(),
                direction_text,
                entity_type,
                entity_id
            ));

            match output_format {
                OutputFormat::Pretty => ui::pretty_output_entity_list(&entities),
                OutputFormat::Json => ui::json_output(&entities),
            }

            Ok(())
        }
        None => {
            ui::error(&format!(
                "Couldn't find '{}' entity with ID '{}'",
                entity_type, entity_id
            ));

            Err(CliError::QueryError)
        }
    }
}

pub fn list_schemas(workspace_path: &PathBuf, output_format: OutputFormat) -> Result<(), CliError> {
    ui::header("Listing schemas");
    let mut workspace = Workspace::new();
    load_workspace_files(&workspace_path, &mut workspace).map_err(|_| CliError::BuildError)?;
    let build = build_workspace(workspace).map_err(|_| CliError::BuildError)?;

    ui::success(&format!(
        "Found {} schemas for this workspace",
        build.schemas.len()
    ));

    match output_format {
        OutputFormat::Pretty => ui::pretty_output_schema_list(&build.schemas.iter().collect()),
        OutputFormat::Json => ui::json_output(&build.schemas),
    }
    Ok(())
}

pub fn list_entities_by_type(
    workspace_path: &PathBuf,
    entity_type: String,
    output_format: OutputFormat,
) -> Result<(), CliError> {
    ui::header("Listing entities by type");
    let graph = load_current_graph(&workspace_path)?;

    let entities = graph.list_by_type(&entity_type.as_str().into());
    ui::success(&format!(
        "Found {} entities with type '{}'",
        entities.len(),
        entity_type,
    ));

    match output_format {
        OutputFormat::Pretty => ui::pretty_output_entity_list(&entities),
        OutputFormat::Json => ui::json_output(&entities),
    }

    Ok(())
}
