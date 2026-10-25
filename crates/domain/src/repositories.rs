//! # Domain Repositories
//!
//! Abstract secondary port interfaces for persisting domain aggregates.

use crate::models::{Member, Tenant, User};
use crate::value_objects::{MemberEmail, MemberId, TenantId, TenantSlug, UserEmail, UserId};
use shared::AppResult;

/// Secondary repository port for Tenant aggregate persistence.
pub trait TenantRepository: Send + Sync {
    /// Persists a new or updated Tenant aggregate.
    fn save<'a>(
        &'a self,
        tenant: &'a Tenant,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>>;

    /// Retrieves a Tenant aggregate by unique identifier.
    fn find_by_id<'a>(
        &'a self,
        id: &'a TenantId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<Tenant>>> + Send + 'a>>;

    /// Retrieves a Tenant aggregate by unique subdomain/url slug.
    fn find_by_slug<'a>(
        &'a self,
        slug: &'a TenantSlug,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<Tenant>>> + Send + 'a>>;

    /// Lists all tenants currently registered in the platform.
    fn list_all<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Tenant>>> + Send + 'a>>;
}

/// Secondary repository port for Member aggregate persistence.
pub trait MemberRepository: Send + Sync {
    /// Persists a new or updated Member aggregate.
    fn save<'a>(
        &'a self,
        member: &'a Member,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>>;

    /// Retrieves a Member aggregate by its unique member id.
    fn find_by_id<'a>(
        &'a self,
        id: &'a MemberId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<Member>>> + Send + 'a>>;

    /// Retrieves a Member aggregate by tenant id and member email.
    fn find_by_tenant_and_email<'a>(
        &'a self,
        tenant_id: &'a TenantId,
        email: &'a MemberEmail,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<Member>>> + Send + 'a>>;

    /// Lists all members belonging to a specific tenant organization.
    fn list_by_tenant<'a>(
        &'a self,
        tenant_id: &'a TenantId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Member>>> + Send + 'a>>;

    /// Lists all memberships belonging to a member email address across all tenant organizations.
    fn list_by_email<'a>(
        &'a self,
        email: &'a MemberEmail,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Member>>> + Send + 'a>>;

    /// Deletes a member from a tenant.
    fn delete<'a>(
        &'a self,
        id: &'a MemberId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>>;
}

/// Secondary repository port for User aggregate persistence.
pub trait UserRepository: Send + Sync {
    /// Persists a new or updated User aggregate.
    fn save<'a>(
        &'a self,
        user: &'a User,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>>;

    /// Retrieves a User aggregate by unique user id.
    fn find_by_id<'a>(
        &'a self,
        id: &'a UserId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<User>>> + Send + 'a>>;

    /// Retrieves a User aggregate by email address.
    fn find_by_email<'a>(
        &'a self,
        email: &'a UserEmail,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<User>>> + Send + 'a>>;
}
