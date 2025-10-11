mod commands;
mod errors;
mod files;
mod logging;
mod query;
mod ui;

use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::ExitCode};

use commands::build_and_save_graph;
use files::get_workspace_path;
use query::CliDirection;
use ui::OutputFormat;

#[derive(Parser, Debug)]
#[command(name = "firm")]
#[command(version, about = "Firm CLI: Business management in the terminal.")]
struct Cli {
    /// Path to firm workspace directory
    #[arg(short, long, global = true)]
    workspace: Option<PathBuf>,

    /// Use cached firm graph in the workspace
    #[arg(short, long, global = true)]
    cached: bool,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format
    #[arg(short, long, global = true, default_value_t = OutputFormat::default())]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Build workspace and entity graph
    Build,
    /// Get an entity by ID
    Get {
        /// Entity type (e.g. person, organization or project)
        entity_type: String,
        /// Entity ID (e.g. john_doe)
        entity_id: String,
    },
    /// List entities of type
    List {
        /// An entity type (e.g. "person") to list entities or "schema" to list schemas
        entity_type: String,
    },
    /// Gets entities related to a given entity
    Related {
        /// Entity type (e.g. person, organization or project)
        entity_type: String,
        /// Entity ID (e.g. john_doe)
        entity_id: String,
        /// Direction of relationships (incoming, outgoing, or both if not specified)
        #[arg(short, long)]
        direction: Option<CliDirection>,
    },
    /// Interactively adds a new entity to a file in the workspace
    Add {
        /// Target firm file
        to_file: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Set up logging
    if let Err(e) = logging::initialize(cli.verbose) {
        ui::error_with_details("Failed to initialize logging", &e.to_string());
        return ExitCode::FAILURE;
    }

    // Get the workspace
    let workspace_path = match get_workspace_path(&cli.workspace) {
        Ok(path) => path,
        Err(_) => return ExitCode::FAILURE,
    };

    // Build the graph unless we're using a cached version or the command requested is build
    if !cli.cached && cli.command != Commands::Build {
        match build_and_save_graph(&workspace_path) {
            Ok(_) => (),
            Err(_) => return ExitCode::FAILURE,
        }
    }

    // Handle CLI subcommands
    let result = match cli.command {
        Commands::Build => build_and_save_graph(&workspace_path),
        Commands::Get {
            entity_type,
            entity_id,
        } => commands::get_entity_by_id(&workspace_path, entity_type, entity_id, cli.format),
        Commands::List { entity_type } => {
            if entity_type == "schema" {
                commands::list_schemas(&workspace_path)
            } else {
                commands::list_entities_by_type(&workspace_path, entity_type, cli.format)
            }
        }
        Commands::Related {
            entity_type,
            entity_id,
            direction,
        } => commands::get_related_entities(
            &workspace_path,
            entity_type,
            entity_id,
            direction,
            cli.format,
        ),
        Commands::Add { to_file } => commands::add_entity(&workspace_path, to_file),
    };

    result.map_or(ExitCode::FAILURE, |_| ExitCode::SUCCESS)
}
