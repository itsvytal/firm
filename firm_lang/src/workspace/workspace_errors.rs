use std::{fmt, io, path::PathBuf};

/// Defines the errors you might encounter using a workspace.
#[derive(Debug)]
pub enum WorkspaceError {
    IoError(io::Error),
    ParseError(PathBuf, String),
    ValidationError(PathBuf, String),
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkspaceError::IoError(error) => {
                write!(f, "There was a problem reading workspace files: {}", error)
            }
            WorkspaceError::ParseError(path_buf, error) => write!(
                f,
                "Workspace file at {} could not be parsed: {}",
                path_buf.display(),
                error
            ),
            WorkspaceError::ValidationError(path_buf, error) => write!(
                f,
                "Workspace file at {} was invalid: {}",
                path_buf.display(),
                error
            ),
        }
    }
}
