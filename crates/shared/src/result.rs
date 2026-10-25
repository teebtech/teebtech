use crate::error::AppError;

/// Standard Result alias for application workflows.
pub type AppResult<T> = Result<T, AppError>;
