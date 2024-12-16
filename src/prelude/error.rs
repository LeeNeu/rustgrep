use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchParameterError {
    #[error("'{0}' argument is empty. Please provide '{0}' argument")]
    Empty(String),
}
