//! # Domain Errors
//!
//! Business domain invariant failures and validation errors.

use shared::AppError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("Invalid tenant name: {0}")]
    InvalidTenantName(String),

    #[error("Invalid tenant slug: {0}")]
    InvalidTenantSlug(String),

    #[error("Tenant slug '{0}' is already taken")]
    TenantSlugAlreadyTaken(String),

    #[error("Tenant not found: {0}")]
    TenantNotFound(String),

    #[error("Invalid member email: {0}")]
    InvalidMemberEmail(String),

    #[error("Invalid member name: {0}")]
    InvalidMemberName(String),

    #[error("Invalid member role: {0}")]
    InvalidMemberRole(String),

    #[error("Member '{0}' already exists in this tenant organization")]
    MemberAlreadyExistsInTenant(String),

    #[error("Member not found: {0}")]
    MemberNotFound(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Invalid display name: {0}")]
    InvalidDisplayName(String),

    #[error("User email '{0}' is already registered")]
    UserEmailAlreadyRegistered(String),

    #[error("User not found: {0}")]
    UserNotFound(String),
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::InvalidTenantName(msg) => AppError::Validation(msg),
            DomainError::InvalidTenantSlug(msg) => AppError::Validation(msg),
            DomainError::TenantSlugAlreadyTaken(msg) => {
                AppError::Conflict(format!("Tenant slug '{}' is already taken", msg))
            }
            DomainError::TenantNotFound(msg) => AppError::NotFound(msg),
            DomainError::InvalidMemberEmail(msg) => AppError::Validation(msg),
            DomainError::InvalidMemberName(msg) => AppError::Validation(msg),
            DomainError::InvalidMemberRole(msg) => AppError::Validation(msg),
            DomainError::MemberAlreadyExistsInTenant(msg) => AppError::Conflict(format!(
                "Member '{}' already exists in this tenant organization",
                msg
            )),
            DomainError::MemberNotFound(msg) => AppError::NotFound(msg),
            DomainError::InvalidCredentials => {
                AppError::Unauthorized("Invalid email or password".to_string())
            }
            DomainError::InvalidEmail(msg) => AppError::Validation(msg),
            DomainError::InvalidPassword(msg) => AppError::Validation(msg),
            DomainError::InvalidDisplayName(msg) => AppError::Validation(msg),
            DomainError::UserEmailAlreadyRegistered(msg) => {
                AppError::Conflict(format!("User email '{}' is already registered", msg))
            }
            DomainError::UserNotFound(msg) => AppError::NotFound(msg),
        }
    }
}
