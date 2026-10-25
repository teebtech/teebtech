//! # Internationalization (i18n) Module
//!
//! Provides LocaleContext, request-scoped server resolution, client hydration matching,
//! and complete multilingual dictionaries for English and Arabic.

pub mod context;
pub mod translations;

pub use context::*;
pub use translations::*;
