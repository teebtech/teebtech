//! # Application Crate
//!
//! Orchestration layer containing use cases, workflows, and DTOs.
//! Completely decoupled from specific database drivers and presentation frameworks.

pub mod dtos;
pub mod portal_dtos;
pub mod use_cases;

pub use dtos::*;
pub use portal_dtos::*;
pub use use_cases::*;
