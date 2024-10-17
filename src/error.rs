use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerateDataError {
    #[error("Invalid fake method: {0}")]
    InvalidFakeMethod(String),

    #[error("Collection not found: {0}")]
    CollectionNotFound(String),

    #[error("Failed to choose a random ID from the collection: {0}")]
    RandomIdSelectionFailed(String),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Data generation error: {0}")]
    DataGenerationError(#[from] GenerateDataError),
}
