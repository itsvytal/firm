use firm_core::graph::EntityGraph;
use std::{env, fs, path::PathBuf};

use crate::errors::CliError;
use crate::ui::{self};

pub const CURRENT_GRAPH_NAME: &str = "current.firm.graph";
pub const BACKUP_GRAPH_NAME: &str = "backup.firm.graph";

pub fn get_workspace_path(directory_path: &Option<PathBuf>) -> Result<PathBuf, CliError> {
    let path = match directory_path {
        Some(path) => path.clone(),
        None => match env::current_dir() {
            Ok(path) => path,
            Err(e) => {
                ui::error_with_details("Cannot access current working directory", &e.to_string());
                return Err(CliError::FileError);
            }
        },
    };

    ui::debug(&format!("Using workspace directory: '{}'", path.display()));
    Ok(path)
}

pub fn save_graph_with_backup(
    workspace_path: &PathBuf,
    graph: &EntityGraph,
) -> Result<(), CliError> {
    let current_graph_path = workspace_path.join(CURRENT_GRAPH_NAME);
    let backup_graph_path = workspace_path.join(BACKUP_GRAPH_NAME);

    // If current firm graph exists, back it up
    if current_graph_path.exists() {
        ui::debug("Backing up existing graph");

        if let Err(e) = fs::rename(&current_graph_path, &backup_graph_path) {
            ui::error_with_details("Failed to rename existing graph file", &e.to_string());
            return Err(CliError::FileError);
        }
    }

    // Write new graph to file
    ui::debug("Saving current graph");
    let serialized_graph = serde_json::to_string(&graph).map_err(|e| {
        ui::error_with_details("Failed to serialize graph", &e.to_string());
        CliError::FileError
    })?;

    if let Err(e) = fs::write(&current_graph_path, serialized_graph) {
        ui::error_with_details("Failed to write graph file", &e.to_string());
        return Err(CliError::FileError);
    }

    ui::info(&format!("Graph saved to {}", current_graph_path.display()));
    Ok(())
}

pub fn load_current_graph(workspace_path: &PathBuf) -> Result<EntityGraph, CliError> {
    let current_graph_path = workspace_path.join(CURRENT_GRAPH_NAME);

    if !current_graph_path.exists() {
        ui::error_with_details(
            "The graph file to load didn't exist",
            &current_graph_path.display().to_string(),
        );
        return Err(CliError::FileError);
    }

    // Load graph from file
    ui::debug("Loading current graph");
    let file_content = fs::read_to_string(&current_graph_path).map_err(|e| {
        ui::error_with_details("Failed to read graph file", &e.to_string());
        CliError::FileError
    })?;

    let graph: EntityGraph = serde_json::from_str(&file_content).map_err(|e| {
        ui::error_with_details("Failed to deserialize graph file", &e.to_string());
        CliError::FileError
    })?;

    ui::info(&format!(
        "Graph loaded from {}",
        current_graph_path.display()
    ));

    Ok(graph)
}
