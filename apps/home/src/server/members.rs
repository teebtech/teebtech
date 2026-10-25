use application::MemberDto;
use leptos::prelude::*;

/// Server function to invite a new team member to a tenant organization.
#[server(InviteMemberServerFn, "/api")]
pub async fn invite_member(
    tenant_id: String,
    email: String,
    name: String,
    role: String,
) -> Result<MemberDto, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use application::{InviteMemberDto, InviteMemberUseCase};
        use config::ServerAppConfig;
        use database::DatabasePool;

        let config = ServerAppConfig::from_env();
        let pool = DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| DatabasePool::new(&config.database_url));
        let tenant_repo = pool.tenant_repository();
        let member_repo = pool.member_repository();

        let use_case = InviteMemberUseCase::new(&tenant_repo, &member_repo);
        let res = use_case
            .execute(InviteMemberDto {
                tenant_id,
                email,
                name,
                role,
            })
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(res)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (tenant_id, email, name, role);
        unreachable!()
    }
}

/// Server function to list all team members belonging to a tenant organization.
#[server(GetTenantMembersServerFn, "/api")]
pub async fn get_tenant_members(tenant_id: String) -> Result<Vec<MemberDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use application::ListTenantMembersUseCase;
        use config::ServerAppConfig;
        use database::DatabasePool;

        let config = ServerAppConfig::from_env();
        let pool = DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| DatabasePool::new(&config.database_url));
        let member_repo = pool.member_repository();

        let use_case = ListTenantMembersUseCase::new(&member_repo);
        let list = use_case
            .execute(&tenant_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(list)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = tenant_id;
        unreachable!()
    }
}
