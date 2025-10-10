use std::path::PathBuf;

use firm_core::make_composite_entity_id;
use firm_lang::workspace::Workspace;

use crate::errors::CliError;
use crate::files::load_current_graph;
use crate::query::CliDirection;
use crate::ui::{self};

use super::{
    build_workspace, load_workspace_files,
};

pub fn get_entity_by_id(
    workspace_path: &PathBuf,
    entity_type: String,
    entity_id: String,
) -> Result<(), CliError> {
    ui::header("Getting entity by ID");
    let graph = load_current_graph(&workspace_path)?;

    let id = make_composite_entity_id(&entity_type, &entity_id);
    match graph.get_entity(&id) {
        Some(entity) => {
            ui::success(&format!(
                "Found '{}' entity with ID '{}'",
                entity_type, entity_id
            ));

            ui::json_output(entity);
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
) -> Result<(), CliError> {
    ui::header("Getting related entities");
    let graph = load_current_graph(&workspace_path)?;

    let id = make_composite_entity_id(&entity_type, &entity_id);
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

            ui::json_output(&entities);
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

pub fn list_schemas(
    workspace_path: &PathBuf,
) -> Result<(), CliError> {
    ui::header("Listing schemas");
    let mut workspace = Workspace::new();
    load_workspace_files(&workspace_path, &mut workspace).map_err(|_| CliError::BuildError)?;
    let build = build_workspace(workspace).map_err(|_| CliError::BuildError)?;

    ui::success(&format!(
        "Found {} schemas for this workspace",
        build.schemas.len()
    ));

    ui::json_output(&build.schemas);
    Ok(())
}

pub fn list_entities_by_type(
    workspace_path: &PathBuf,
    entity_type: String,
) -> Result<(), CliError> {
    ui::header("Listing entities by type");
    let graph = load_current_graph(&workspace_path)?;

    let entities = graph.list_by_type(&entity_type.as_str().into());
    ui::success(&format!(
        "Found {} entities with type '{}'",
        entities.len(),
        entity_type,
    ));

    ui::json_output(&entities);
    Ok(())
}
