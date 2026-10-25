//! # Domain Entities & Aggregates
//!
//! Pure aggregate roots and entities representing business domain invariants.

use crate::value_objects::{
    DisplayName, MemberEmail, MemberId, MemberName, MemberRole, PasswordHash, TenantId, TenantName,
    TenantSlug, UserEmail, UserId,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Tenant Aggregate Root.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tenant {
    pub id: TenantId,
    pub name: TenantName,
    pub slug: TenantSlug,
    pub created_at: String,
}

impl Tenant {
    pub fn new(id: TenantId, name: TenantName, slug: TenantSlug) -> Self {
        let created_at = "2026-09-01T00:00:00Z".to_string();
        Self {
            id,
            name,
            slug,
            created_at,
        }
    }

    pub fn reconstitute(
        id: TenantId,
        name: TenantName,
        slug: TenantSlug,
        created_at: String,
    ) -> Self {
        Self {
            id,
            name,
            slug,
            created_at,
        }
    }
}

/// Tenant Member Aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Member {
    pub id: MemberId,
    pub tenant_id: TenantId,
    pub email: MemberEmail,
    pub name: MemberName,
    pub role: MemberRole,
    pub created_at: String,
}

impl Member {
    pub fn new(
        id: MemberId,
        tenant_id: TenantId,
        email: MemberEmail,
        name: MemberName,
        role: MemberRole,
    ) -> Self {
        let created_at = "2026-09-01T00:00:00Z".to_string();
        Self {
            id,
            tenant_id,
            email,
            name,
            role,
            created_at,
        }
    }

    pub fn reconstitute(
        id: MemberId,
        tenant_id: TenantId,
        email: MemberEmail,
        name: MemberName,
        role: MemberRole,
        created_at: String,
    ) -> Self {
        Self {
            id,
            tenant_id,
            email,
            name,
            role,
            created_at,
        }
    }
}

/// User Aggregate Root (Authentication Identity).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct User {
    pub id: UserId,
    pub email: UserEmail,
    pub password_hash: PasswordHash,
    pub display_name: DisplayName,
    pub created_at: String,
}

impl User {
    pub fn new(
        id: UserId,
        email: UserEmail,
        password_hash: PasswordHash,
        display_name: DisplayName,
    ) -> Self {
        let created_at = "2026-09-01T00:00:00Z".to_string();
        Self {
            id,
            email,
            password_hash,
            display_name,
            created_at,
        }
    }

    pub fn reconstitute(
        id: UserId,
        email: UserEmail,
        password_hash: PasswordHash,
        display_name: DisplayName,
        created_at: String,
    ) -> Self {
        Self {
            id,
            email,
            password_hash,
            display_name,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let uid = UserId::new("usr_123");
        let email = UserEmail::new("admin@teebtech.com").unwrap();
        let hash = PasswordHash::new("$2b$12$e8Y...hash").unwrap();
        let name = DisplayName::new("Admin User").unwrap();

        let user = User::new(uid.clone(), email.clone(), hash.clone(), name.clone());
        assert_eq!(user.id, uid);
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, hash);
        assert_eq!(user.display_name, name);
    }
}
