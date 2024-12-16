use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchParameterError {
    #[error("'{0}' argument is empty.\nPlease provide '{0}' argument.")]
    Empty(String),
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("No match found for '{0}'.")]
    NotFound(String),
}
