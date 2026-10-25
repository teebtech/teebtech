//! # Authorization Crate
//!
//! Role-based access control (RBAC), fine-grained permissions, and authorization policies.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use shared::{AppResult, EntityId};

/// Core system roles.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Role {
    SuperAdmin,
    TenantAdmin,
    Member,
    Guest,
}

/// Granular system permission token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Permission(String);

impl Permission {
    pub fn new(p: impl Into<String>) -> Self {
        Self(p.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Authorization evaluator interface.
pub trait AuthorizationPolicy: Send + Sync {
    fn has_permission(&self, user_id: &EntityId, permission: &Permission) -> AppResult<bool>;
    fn has_role(&self, user_id: &EntityId, role: &Role) -> AppResult<bool>;
}
