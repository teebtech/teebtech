//! # Domain Value Objects
//!
//! Immutable domain value objects with encapsulated self-validation.

use crate::errors::DomainError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type-safe Tenant Identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TenantId(String);

impl TenantId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self(format!("tnt_{:x}", ts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for TenantId {
    fn from(s: T) -> Self {
        Self::new(s)
    }
}

/// Tenant Organization Display Name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TenantName(String);

impl TenantName {
    pub fn new(name: impl AsRef<str>) -> Result<Self, DomainError> {
        let trimmed = name.as_ref().trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidTenantName(
                "Tenant name cannot be empty".to_string(),
            ));
        }
        if trimmed.len() < 2 {
            return Err(DomainError::InvalidTenantName(
                "Tenant name must be at least 2 characters long".to_string(),
            ));
        }
        if trimmed.len() > 100 {
            return Err(DomainError::InvalidTenantName(
                "Tenant name cannot exceed 100 characters".to_string(),
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TenantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Tenant URL/Subdomain Slug.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TenantSlug(String);

impl TenantSlug {
    pub fn new(slug: impl AsRef<str>) -> Result<Self, DomainError> {
        let raw = slug.as_ref().trim().to_lowercase();
        if raw.len() < 2 {
            return Err(DomainError::InvalidTenantSlug(
                "Tenant slug must be at least 2 characters long".to_string(),
            ));
        }
        if raw.len() > 50 {
            return Err(DomainError::InvalidTenantSlug(
                "Tenant slug cannot exceed 50 characters".to_string(),
            ));
        }
        if raw.starts_with('-') || raw.ends_with('-') {
            return Err(DomainError::InvalidTenantSlug(
                "Tenant slug cannot start or end with a hyphen".to_string(),
            ));
        }

        let is_valid = raw
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        if !is_valid || raw.contains("--") {
            return Err(DomainError::InvalidTenantSlug(
                "Tenant slug must contain only lowercase alphanumeric characters and single hyphens".to_string(),
            ));
        }

        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TenantSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type-safe Member Identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemberId(String);

impl MemberId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self(format!("mbr_{:x}", ts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MemberId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for MemberId {
    fn from(s: T) -> Self {
        Self::new(s)
    }
}

/// Validated Member Email Value Object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemberEmail(String);

impl MemberEmail {
    pub fn new(email: impl AsRef<str>) -> Result<Self, DomainError> {
        let raw = email.as_ref().trim().to_lowercase();
        if raw.len() < 5 {
            return Err(DomainError::InvalidMemberEmail(
                "Email address must be at least 5 characters long".to_string(),
            ));
        }
        if raw.len() > 150 {
            return Err(DomainError::InvalidMemberEmail(
                "Email address cannot exceed 150 characters".to_string(),
            ));
        }
        if !raw.contains('@') || !raw.contains('.') {
            return Err(DomainError::InvalidMemberEmail(
                "Email address must contain '@' and domain extension".to_string(),
            ));
        }
        if raw.starts_with('@') || raw.ends_with('@') || raw.starts_with('.') || raw.ends_with('.')
        {
            return Err(DomainError::InvalidMemberEmail(
                "Email address format is invalid".to_string(),
            ));
        }

        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MemberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validated Member Name Value Object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemberName(String);

impl MemberName {
    pub fn new(name: impl AsRef<str>) -> Result<Self, DomainError> {
        let trimmed = name.as_ref().trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidMemberName(
                "Member name cannot be empty".to_string(),
            ));
        }
        if trimmed.len() < 2 {
            return Err(DomainError::InvalidMemberName(
                "Member name must be at least 2 characters long".to_string(),
            ));
        }
        if trimmed.len() > 100 {
            return Err(DomainError::InvalidMemberName(
                "Member name cannot exceed 100 characters".to_string(),
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Tenant Membership Role Value Object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MemberRole {
    Owner,
    Admin,
    Member,
    Viewer,
}

impl MemberRole {
    pub fn from_str_loose(s: &str) -> Result<Self, DomainError> {
        match s.trim().to_lowercase().as_str() {
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            "member" => Ok(Self::Member),
            "viewer" => Ok(Self::Viewer),
            other => Err(DomainError::InvalidMemberRole(format!(
                "Unknown role '{}'. Valid roles are Owner, Admin, Member, Viewer",
                other
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Owner => "Owner",
            Self::Admin => "Admin",
            Self::Member => "Member",
            Self::Viewer => "Viewer",
        }
    }
}

impl std::fmt::Display for MemberRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Type-safe User Identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserId(String);

impl UserId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self(format!("usr_{:x}", ts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for UserId {
    fn from(s: T) -> Self {
        Self::new(s)
    }
}

/// Validated User Email Value Object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(email: impl AsRef<str>) -> Result<Self, DomainError> {
        let raw = email.as_ref().trim().to_lowercase();
        if raw.len() < 5 {
            return Err(DomainError::InvalidEmail(
                "Email address must be at least 5 characters long".to_string(),
            ));
        }
        if raw.len() > 150 {
            return Err(DomainError::InvalidEmail(
                "Email address cannot exceed 150 characters".to_string(),
            ));
        }
        if !raw.contains('@') || !raw.contains('.') {
            return Err(DomainError::InvalidEmail(
                "Email address must contain '@' and domain extension".to_string(),
            ));
        }
        if raw.starts_with('@') || raw.ends_with('@') || raw.starts_with('.') || raw.ends_with('.')
        {
            return Err(DomainError::InvalidEmail(
                "Email address format is invalid".to_string(),
            ));
        }

        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UserEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Secure Password Hash Value Object (never plaintext).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: impl Into<String>) -> Result<Self, DomainError> {
        let h = hash.into();
        if h.trim().is_empty() {
            return Err(DomainError::InvalidPassword(
                "Password hash cannot be empty".to_string(),
            ));
        }
        Ok(Self(h))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Plaintext Password Validation Object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(plain: impl AsRef<str>) -> Result<Self, DomainError> {
        let p = plain.as_ref();
        if p.len() < 8 {
            return Err(DomainError::InvalidPassword(
                "Password must be at least 8 characters long".to_string(),
            ));
        }
        if p.len() > 128 {
            return Err(DomainError::InvalidPassword(
                "Password cannot exceed 128 characters".to_string(),
            ));
        }
        Ok(Self(p.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// User Display Name Value Object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(name: impl AsRef<str>) -> Result<Self, DomainError> {
        let trimmed = name.as_ref().trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidDisplayName(
                "Display name cannot be empty".to_string(),
            ));
        }
        if trimmed.len() < 2 {
            return Err(DomainError::InvalidDisplayName(
                "Display name must be at least 2 characters long".to_string(),
            ));
        }
        if trimmed.len() > 100 {
            return Err(DomainError::InvalidDisplayName(
                "Display name cannot exceed 100 characters".to_string(),
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DisplayName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_email_and_password_validation() {
        assert!(UserEmail::new("user@teebtech.com").is_ok());
        assert!(UserEmail::new("invalid").is_err());
        assert!(PlainPassword::new("password123").is_ok());
        assert!(PlainPassword::new("short").is_err());
        assert!(DisplayName::new("Alice").is_ok());
        assert!(DisplayName::new("").is_err());
    }
}
