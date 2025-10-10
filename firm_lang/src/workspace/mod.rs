mod build;
mod io;
mod workspace_errors;

use std::{collections::HashMap, path::PathBuf};

pub use build::WorkspaceBuild;
pub use workspace_errors::WorkspaceError;

use crate::parser::ParsedSource;

#[derive(Debug)]
pub struct Workspace {
    files: HashMap<PathBuf, WorkspaceFile>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn num_files(&self) -> usize {
        self.files.len()
    }
}

#[derive(Debug)]
pub struct WorkspaceFile {
    parsed: ParsedSource,
}

impl WorkspaceFile {
    pub fn new(parsed: ParsedSource) -> Self {
        Self { parsed }
    }
}
