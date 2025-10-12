/// The errors that can occur when using the CLI.
#[derive(Debug)]
pub enum CliError {
    BuildError,
    FileError,
    QueryError,
    InputError,
}
