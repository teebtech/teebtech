use application::TenantDto;
use leptos::prelude::*;

/// Server function to register a new tenant organization.
#[server(RegisterTenantServerFn, "/api")]
pub async fn register_tenant(name: String, slug: String) -> Result<TenantDto, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use application::{CreateTenantDto, RegisterTenantUseCase};
        use config::ServerAppConfig;
        use database::DatabasePool;

        let config = ServerAppConfig::from_env();
        let pool = DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| DatabasePool::new(&config.database_url));
        let repo = pool.tenant_repository();

        let use_case = RegisterTenantUseCase::new(&repo);
        let res = use_case
            .execute(CreateTenantDto { name, slug })
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(res)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (name, slug);
        unreachable!()
    }
}

/// Server function to list all registered tenant organizations.
#[server(GetTenantsServerFn, "/api")]
pub async fn get_tenants() -> Result<Vec<TenantDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use application::ListTenantsUseCase;
        use config::ServerAppConfig;
        use database::DatabasePool;

        let config = ServerAppConfig::from_env();
        let pool = DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| DatabasePool::new(&config.database_url));
        let repo = pool.tenant_repository();

        let use_case = ListTenantsUseCase::new(&repo);
        let list = use_case
            .execute()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(list)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
