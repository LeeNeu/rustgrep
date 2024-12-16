use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchParameterError {
    #[error("'{0}' argument is empty.\nPlease provide '{0}' argument.")]
    Empty(String),

    #[error("Failed to read piped input as PATHNAME argument.")]
    FalsePipeInput,
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("No match found for '{0}'.")]
    NotFound(String),
}
