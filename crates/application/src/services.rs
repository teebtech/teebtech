use crate::dtos::{CreateTenantDto, UserProfileDto};
use shared::{AppResult, EntityId};

/// Application service contract for Tenant workflows.
pub trait TenantService: Send + Sync {
    fn register_tenant(&self, dto: CreateTenantDto) -> AppResult<EntityId>;
}

/// Application service contract for User workflows.
pub trait UserService: Send + Sync {
    fn get_profile(&self, user_id: &EntityId) -> AppResult<UserProfileDto>;
}
