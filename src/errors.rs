use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Task with ID {0} not found")]
    TaskNotFound(usize),

    #[error("Failed to read or write tasks")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse tasks")]
    Parse(#[from] serde_json::Error),
}
