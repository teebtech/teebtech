use thiserror::Error;

/// Universal error enum for the TeebTech platform.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Authentication failure: {0}")]
    Unauthorized(String),

    #[error("Permission denied: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal system error: {0}")]
    Internal(String),
}
