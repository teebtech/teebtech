//! # Shared Crate
//!
//! Common primitives, error handling traits, identifier types, and shared utilities.

pub mod error;
pub mod id;
pub mod result;

pub use error::AppError;
pub use id::EntityId;
pub use result::AppResult;
