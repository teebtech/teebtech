//! # PostgreSQL Member Repository Adapter
//!
//! Secondary adapter implementing domain `MemberRepository` using SQLx.

use domain::{Member, MemberEmail, MemberId, MemberName, MemberRepository, MemberRole, TenantId};
use shared::{AppError, AppResult};
#[cfg(feature = "sqlx")]
use sqlx::{PgPool, Row};
use std::sync::{Arc, Mutex};

/// Repository adapter for Member persistence.
#[derive(Clone)]
pub struct PgMemberRepository {
    #[cfg(feature = "sqlx")]
    pool: Option<PgPool>,
    fallback: Arc<Mutex<Vec<Member>>>,
}

impl PgMemberRepository {
    /// Creates a repository backed by a real SQLx connection pool.
    #[cfg(feature = "sqlx")]
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Some(pool),
            fallback: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Creates an in-memory repository instance.
    pub fn in_memory() -> Self {
        Self {
            #[cfg(feature = "sqlx")]
            pool: None,
            fallback: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl MemberRepository for PgMemberRepository {
    fn save<'a>(
        &'a self,
        member: &'a Member,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>> {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                sqlx::query(
                    r#"
                    INSERT INTO tenant_members (id, tenant_id, email, name, role, created_at)
                    VALUES ($1, $2, $3, $4, $5, NOW())
                    ON CONFLICT (id) DO UPDATE SET
                        email = EXCLUDED.email,
                        name = EXCLUDED.name,
                        role = EXCLUDED.role
                    "#,
                )
                .bind(member.id.as_str())
                .bind(member.tenant_id.as_str())
                .bind(member.email.as_str())
                .bind(member.name.as_str())
                .bind(member.role.as_str())
                .execute(pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to save member: {}", e)))?;
            }

            let mut mem = self.fallback.lock().unwrap();
            if let Some(idx) = mem.iter().position(|m| m.id == member.id) {
                mem[idx] = member.clone();
            } else {
                mem.push(member.clone());
            }

            Ok(())
        })
    }

    fn find_by_id<'a>(
        &'a self,
        id: &'a MemberId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<Member>>> + Send + 'a>>
    {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, email, name, role, created_at::text as created_at
                    FROM tenant_members
                    WHERE id = $1
                    "#,
                )
                .bind(id.as_str())
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to fetch member: {}", e)))?;

                if let Some(row) = row {
                    let id_str: String = row.get("id");
                    let tid_str: String = row.get("tenant_id");
                    let email_str: String = row.get("email");
                    let name_str: String = row.get("name");
                    let role_str: String = row.get("role");
                    let created_at: String = row.get("created_at");

                    let member = Member::reconstitute(
                        MemberId::new(id_str),
                        TenantId::new(tid_str),
                        MemberEmail::new(email_str)?,
                        MemberName::new(name_str)?,
                        MemberRole::from_str_loose(&role_str)?,
                        created_at,
                    );
                    return Ok(Some(member));
                }
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem.iter().find(|m| &m.id == id).cloned())
        })
    }

    fn find_by_tenant_and_email<'a>(
        &'a self,
        tenant_id: &'a TenantId,
        email: &'a MemberEmail,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<Member>>> + Send + 'a>>
    {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, email, name, role, created_at::text as created_at
                    FROM tenant_members
                    WHERE tenant_id = $1 AND email = $2
                    "#,
                )
                .bind(tenant_id.as_str())
                .bind(email.as_str())
                .fetch_optional(pool)
                .await
                .map_err(|e| {
                    AppError::Internal(format!("Failed to fetch member by email: {}", e))
                })?;

                if let Some(row) = row {
                    let id_str: String = row.get("id");
                    let tid_str: String = row.get("tenant_id");
                    let email_str: String = row.get("email");
                    let name_str: String = row.get("name");
                    let role_str: String = row.get("role");
                    let created_at: String = row.get("created_at");

                    let member = Member::reconstitute(
                        MemberId::new(id_str),
                        TenantId::new(tid_str),
                        MemberEmail::new(email_str)?,
                        MemberName::new(name_str)?,
                        MemberRole::from_str_loose(&role_str)?,
                        created_at,
                    );
                    return Ok(Some(member));
                }
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem
                .iter()
                .find(|m| &m.tenant_id == tenant_id && &m.email == email)
                .cloned())
        })
    }

    fn list_by_tenant<'a>(
        &'a self,
        tenant_id: &'a TenantId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Member>>> + Send + 'a>>
    {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, email, name, role, created_at::text as created_at
                    FROM tenant_members
                    WHERE tenant_id = $1
                    ORDER BY created_at ASC
                    "#,
                )
                .bind(tenant_id.as_str())
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to list tenant members: {}", e)))?;

                let mut list = Vec::with_capacity(rows.len());
                for row in rows {
                    let id_str: String = row.get("id");
                    let tid_str: String = row.get("tenant_id");
                    let email_str: String = row.get("email");
                    let name_str: String = row.get("name");
                    let role_str: String = row.get("role");
                    let created_at: String = row.get("created_at");

                    list.push(Member::reconstitute(
                        MemberId::new(id_str),
                        TenantId::new(tid_str),
                        MemberEmail::new(email_str)?,
                        MemberName::new(name_str)?,
                        MemberRole::from_str_loose(&role_str)?,
                        created_at,
                    ));
                }
                return Ok(list);
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem
                .iter()
                .filter(|m| &m.tenant_id == tenant_id)
                .cloned()
                .collect())
        })
    }

    fn list_by_email<'a>(
        &'a self,
        email: &'a MemberEmail,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Member>>> + Send + 'a>>
    {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, email, name, role, created_at::text as created_at
                    FROM tenant_members
                    WHERE email = $1
                    ORDER BY created_at ASC
                    "#,
                )
                .bind(email.as_str())
                .fetch_all(pool)
                .await
                .map_err(|e| {
                    AppError::Internal(format!("Failed to list memberships by email: {}", e))
                })?;

                let mut list = Vec::with_capacity(rows.len());
                for row in rows {
                    let id_str: String = row.get("id");
                    let tid_str: String = row.get("tenant_id");
                    let email_str: String = row.get("email");
                    let name_str: String = row.get("name");
                    let role_str: String = row.get("role");
                    let created_at: String = row.get("created_at");

                    list.push(Member::reconstitute(
                        MemberId::new(id_str),
                        TenantId::new(tid_str),
                        MemberEmail::new(email_str)?,
                        MemberName::new(name_str)?,
                        MemberRole::from_str_loose(&role_str)?,
                        created_at,
                    ));
                }
                return Ok(list);
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem.iter().filter(|m| &m.email == email).cloned().collect())
        })
    }

    fn delete<'a>(
        &'a self,
        id: &'a MemberId,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>> {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                sqlx::query("DELETE FROM tenant_members WHERE id = $1")
                    .bind(id.as_str())
                    .execute(pool)
                    .await
                    .map_err(|e| AppError::Internal(format!("Failed to delete member: {}", e)))?;
            }

            let mut mem = self.fallback.lock().unwrap();
            mem.retain(|m| &m.id != id);
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::TenantRepository;

    #[tokio::test]
    async fn test_pg_member_repository_in_memory() {
        let repo = PgMemberRepository::in_memory();
        let tid = TenantId::new("tnt_test");
        let id = MemberId::new("mbr_1");
        let email = MemberEmail::new("member@test.com").unwrap();
        let name = MemberName::new("Test User").unwrap();
        let role = MemberRole::Admin;

        let member = Member::new(id.clone(), tid.clone(), email.clone(), name.clone(), role);
        repo.save(&member).await.unwrap();

        let found = repo
            .find_by_tenant_and_email(&tid, &email)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found.id, id);
        assert_eq!(found.name, name);
        assert_eq!(found.role, MemberRole::Admin);

        let list = repo.list_by_tenant(&tid).await.unwrap();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn test_pg_member_repository_real_postgres() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev".to_string()
        });

        if let Ok(pool) = sqlx::PgPool::connect(&db_url).await {
            let tenant_repo = crate::PgTenantRepository::new(pool.clone());
            let unique_slug = format!(
                "test-mbr-tnt-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );
            let tid = domain::TenantId::new(format!("tnt_{}", unique_slug));
            let tname = domain::TenantName::new("Member Test Org").unwrap();
            let tslug = domain::TenantSlug::new(unique_slug).unwrap();
            tenant_repo
                .save(&domain::Tenant::new(tid.clone(), tname, tslug))
                .await
                .unwrap();

            let member_repo = PgMemberRepository::new(pool);
            let mid = MemberId::generate();
            let email = MemberEmail::new("alice@membertest.com").unwrap();
            let name = MemberName::new("Alice Engineer").unwrap();
            let role = MemberRole::Admin;

            let member = Member::new(mid.clone(), tid.clone(), email.clone(), name.clone(), role);
            member_repo
                .save(&member)
                .await
                .expect("member save should succeed");

            let found = member_repo
                .find_by_id(&mid)
                .await
                .unwrap()
                .expect("should find member by id");
            assert_eq!(found.id, mid);
            assert_eq!(found.email, email);
            assert_eq!(found.role, MemberRole::Admin);

            let found_by_email = member_repo
                .find_by_tenant_and_email(&tid, &email)
                .await
                .unwrap()
                .expect("should find member by tenant and email");
            assert_eq!(found_by_email.id, mid);

            let list = member_repo.list_by_tenant(&tid).await.unwrap();
            assert_eq!(list.len(), 1);

            // Test duplicate email in same tenant fails database unique constraint
            let dup_member = Member::new(
                MemberId::generate(),
                tid.clone(),
                email,
                name,
                MemberRole::Member,
            );
            let dup_res = member_repo.save(&dup_member).await;
            assert!(
                dup_res.is_err(),
                "Duplicate email in same tenant must fail database constraint"
            );
        }
    }
}
