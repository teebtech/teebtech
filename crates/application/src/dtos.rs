//! # Application Data Transfer Objects (DTOs)
//!
//! Serializable data structures used across application boundaries and delivery protocols.

use domain::{Member, Tenant, User};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Request payload to register a new tenant organization.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateTenantDto {
    pub name: String,
    pub slug: String,
}

/// Tenant response DTO returned by use cases.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TenantDto {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub created_at: String,
}

impl From<Tenant> for TenantDto {
    fn from(tenant: Tenant) -> Self {
        Self {
            id: tenant.id.to_string(),
            name: tenant.name.to_string(),
            slug: tenant.slug.to_string(),
            created_at: tenant.created_at,
        }
    }
}

/// Request payload to invite a new team member to a tenant organization.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InviteMemberDto {
    pub tenant_id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

/// Member response DTO returned by use cases.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemberDto {
    pub id: String,
    pub tenant_id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: String,
}

impl From<Member> for MemberDto {
    fn from(member: Member) -> Self {
        Self {
            id: member.id.to_string(),
            tenant_id: member.tenant_id.to_string(),
            email: member.email.to_string(),
            name: member.name.to_string(),
            role: member.role.to_string(),
            created_at: member.created_at,
        }
    }
}

/// Request payload to register a new platform user.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterUserDto {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

/// Request payload to log in with email and password credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

/// Response returned upon successful authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuthenticatedUserDto {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub token: String,
    pub expires_in: u64,
}

/// Information describing the currently authenticated user identity.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CurrentUserDto {
    pub id: String,
    pub email: String,
    pub display_name: String,
}

impl From<User> for CurrentUserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email.to_string(),
            display_name: user.display_name.to_string(),
        }
    }
}

/// Tenant membership summary for authenticated user context.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserTenantMembershipDto {
    pub tenant_id: String,
    pub tenant_name: String,
    pub tenant_slug: String,
    pub role: String,
}

/// Client portal session context containing user identity and authorized tenant memberships.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientSessionDto {
    pub user: CurrentUserDto,
    pub memberships: Vec<UserTenantMembershipDto>,
}
