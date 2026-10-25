//! # PostgreSQL Tenant Repository Adapter
//!
//! Secondary adapter implementing domain `TenantRepository` using SQLx.

use domain::{Tenant, TenantId, TenantName, TenantRepository, TenantSlug};
use shared::{AppError, AppResult};
#[cfg(feature = "sqlx")]
use sqlx::{PgPool, Row};
use std::sync::{Arc, Mutex};

/// Repository adapter for Tenant persistence.
#[derive(Clone)]
pub struct PgTenantRepository {
    #[cfg(feature = "sqlx")]
    pool: Option<PgPool>,
    fallback: Arc<Mutex<Vec<Tenant>>>,
}

impl PgTenantRepository {
    /// Creates a repository backed by a real SQLx connection pool.
    #[cfg(feature = "sqlx")]
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Some(pool),
            fallback: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Creates an in-memory repository instance (useful for unit testing and offline development).
    pub fn in_memory() -> Self {
        Self {
            #[cfg(feature = "sqlx")]
            pool: None,
            fallback: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl TenantRepository for PgTenantRepository {
    fn save<'a>(
        &'a self,
        tenant: &'a Tenant,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>> {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                sqlx::query(
                    r#"
                    INSERT INTO tenants (id, name, slug, created_at)
                    VALUES ($1, $2, $3, NOW())
                    ON CONFLICT (id) DO UPDATE SET
                        name = EXCLUDED.name,
                        slug = EXCLUDED.slug
                    "#,
                )
                .bind(tenant.id.as_str())
                .bind(tenant.name.as_str())
                .bind(tenant.slug.as_str())
                .execute(pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to save tenant: {}", e)))?;
            }

            let mut mem = self.fallback.lock().unwrap();
            if let Some(idx) = mem.iter().position(|t| t.id == tenant.id) {
                mem[idx] = tenant.clone();
            } else {
                mem.push(tenant.clone());
            }

            Ok(())
        })
    }

    fn find_by_id<'a>(
        &'a self,
        id: &'a TenantId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<Tenant>>> + Send + 'a>>
    {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let row = sqlx::query(
                    r#"
                    SELECT id, name, slug, created_at::text as created_at
                    FROM tenants
                    WHERE id = $1
                    "#,
                )
                .bind(id.as_str())
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to fetch tenant: {}", e)))?;

                if let Some(row) = row {
                    let id_str: String = row.get("id");
                    let name_str: String = row.get("name");
                    let slug_str: String = row.get("slug");
                    let created_at: String = row.get("created_at");

                    let name = TenantName::new(name_str)?;
                    let slug = TenantSlug::new(slug_str)?;
                    return Ok(Some(Tenant::reconstitute(
                        TenantId::new(id_str),
                        name,
                        slug,
                        created_at,
                    )));
                }
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem.iter().find(|t| &t.id == id).cloned())
        })
    }

    fn find_by_slug<'a>(
        &'a self,
        slug: &'a TenantSlug,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<Tenant>>> + Send + 'a>>
    {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let row = sqlx::query(
                    r#"
                    SELECT id, name, slug, created_at::text as created_at
                    FROM tenants
                    WHERE slug = $1
                    "#,
                )
                .bind(slug.as_str())
                .fetch_optional(pool)
                .await
                .map_err(|e| {
                    AppError::Internal(format!("Failed to fetch tenant by slug: {}", e))
                })?;

                if let Some(row) = row {
                    let id_str: String = row.get("id");
                    let name_str: String = row.get("name");
                    let slug_str: String = row.get("slug");
                    let created_at: String = row.get("created_at");

                    let name = TenantName::new(name_str)?;
                    let slug_vo = TenantSlug::new(slug_str)?;
                    return Ok(Some(Tenant::reconstitute(
                        TenantId::new(id_str),
                        name,
                        slug_vo,
                        created_at,
                    )));
                }
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem.iter().find(|t| &t.slug == slug).cloned())
        })
    }

    fn list_all<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Tenant>>> + Send + 'a>>
    {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let rows = sqlx::query(
                    r#"
                    SELECT id, name, slug, created_at::text as created_at
                    FROM tenants
                    ORDER BY created_at DESC
                    "#,
                )
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to list tenants: {}", e)))?;

                let mut list = Vec::with_capacity(rows.len());
                for row in rows {
                    let id_str: String = row.get("id");
                    let name_str: String = row.get("name");
                    let slug_str: String = row.get("slug");
                    let created_at: String = row.get("created_at");

                    let name = TenantName::new(name_str)?;
                    let slug = TenantSlug::new(slug_str)?;
                    list.push(Tenant::reconstitute(
                        TenantId::new(id_str),
                        name,
                        slug,
                        created_at,
                    ));
                }
                return Ok(list);
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem.clone())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pg_tenant_repository_in_memory() {
        let repo = PgTenantRepository::in_memory();
        let id = TenantId::new("tnt_test");
        let name = TenantName::new("Test Corp").unwrap();
        let slug = TenantSlug::new("test-corp").unwrap();

        let tenant = Tenant::new(id.clone(), name.clone(), slug.clone());
        repo.save(&tenant).await.unwrap();

        let found = repo.find_by_id(&id).await.unwrap().unwrap();
        assert_eq!(found.id, id);
        assert_eq!(found.name, name);
        assert_eq!(found.slug, slug);

        let list = repo.list_all().await.unwrap();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn test_pg_tenant_repository_real_postgres() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev".to_string()
        });

        if let Ok(pool) = sqlx::PgPool::connect(&db_url).await {
            let repo = PgTenantRepository::new(pool);
            let unique_slug_str = format!(
                "test-slug-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );
            let id = TenantId::new(format!("tnt_{}", unique_slug_str));
            let name = TenantName::new("Integration Test Corp").unwrap();
            let slug = TenantSlug::new(unique_slug_str).unwrap();

            let tenant = Tenant::new(id.clone(), name.clone(), slug.clone());
            repo.save(&tenant).await.expect("save should succeed");

            let found = repo
                .find_by_id(&id)
                .await
                .unwrap()
                .expect("should find by id");
            assert_eq!(found.id, id);
            assert_eq!(found.name, name);
            assert_eq!(found.slug, slug);

            let found_by_slug = repo
                .find_by_slug(&slug)
                .await
                .unwrap()
                .expect("should find by slug");
            assert_eq!(found_by_slug.id, id);

            let list = repo.list_all().await.unwrap();
            assert!(list.iter().any(|t| t.id == id));

            // Verify duplicate slug error on distinct ID (tests database UNIQUE constraint)
            let duplicate_id = TenantId::new(format!("tnt_dup_{}", id.as_str()));
            let duplicate_tenant = Tenant::new(duplicate_id, name, slug);
            let dup_res = repo.save(&duplicate_tenant).await;
            assert!(
                dup_res.is_err(),
                "Duplicate slug insert must fail due to database unique constraint"
            );
        }
    }
}
