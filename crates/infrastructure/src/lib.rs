//! # Infrastructure Crate
//!
//! External service integrations, background jobs, email sending, telemetry, and external I/O.

pub mod telemetry;

pub use telemetry::*;
