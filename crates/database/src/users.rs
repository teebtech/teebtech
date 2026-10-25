//! # User Repository Implementation
//!
//! Secondary adapter implementing `domain::UserRepository` backed by SQLx PostgreSQL.

use domain::{DisplayName, PasswordHash, User, UserEmail, UserId, UserRepository};
use shared::{AppError, AppResult};
#[cfg(feature = "sqlx")]
use sqlx::{PgPool, Row};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

/// PostgreSQL adapter for User identity persistence.
#[derive(Clone)]
pub struct PgUserRepository {
    #[cfg(feature = "sqlx")]
    pool: Option<PgPool>,
    fallback: Arc<Mutex<Vec<User>>>,
}

impl PgUserRepository {
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

impl UserRepository for PgUserRepository {
    fn save<'a>(
        &'a self,
        user: &'a User,
    ) -> Pin<Box<dyn Future<Output = AppResult<()>> + Send + 'a>> {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let id = user.id.as_str();
                let email = user.email.as_str();
                let password_hash = user.password_hash.as_str();
                let display_name = user.display_name.as_str();

                let res = sqlx::query(
                    r#"
                    INSERT INTO users (id, email, password_hash, display_name)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT (id) DO UPDATE
                    SET email = EXCLUDED.email,
                        password_hash = EXCLUDED.password_hash,
                        display_name = EXCLUDED.display_name
                    "#,
                )
                .bind(id)
                .bind(email)
                .bind(password_hash)
                .bind(display_name)
                .execute(pool)
                .await;

                match res {
                    Ok(_) => return Ok(()),
                    Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                        return Err(AppError::Conflict(format!(
                            "User email '{}' is already registered",
                            email
                        )));
                    }
                    Err(e) => {
                        return Err(AppError::Internal(format!("Failed to save user: {}", e)))
                    }
                }
            }

            let mut mem = self.fallback.lock().unwrap();
            if let Some(pos) = mem.iter().position(|u| u.id == user.id) {
                mem[pos] = user.clone();
            } else {
                if mem.iter().any(|u| u.email == user.email) {
                    return Err(AppError::Conflict(format!(
                        "User email '{}' is already registered",
                        user.email
                    )));
                }
                mem.push(user.clone());
            }
            Ok(())
        })
    }

    fn find_by_id<'a>(
        &'a self,
        id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = AppResult<Option<User>>> + Send + 'a>> {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let row = sqlx::query(
                    r#"
                    SELECT id, email, password_hash, display_name, created_at::text as created_at
                    FROM users
                    WHERE id = $1
                    "#,
                )
                .bind(id.as_str())
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to fetch user by id: {}", e)))?;

                if let Some(r) = row {
                    let uid = UserId::new(r.get::<String, _>("id"));
                    let email = UserEmail::new(r.get::<String, _>("email"))?;
                    let pass_hash = PasswordHash::new(r.get::<String, _>("password_hash"))?;
                    let dname = DisplayName::new(r.get::<String, _>("display_name"))?;
                    let created_at: String = r.get("created_at");
                    return Ok(Some(User::reconstitute(
                        uid, email, pass_hash, dname, created_at,
                    )));
                }
                return Ok(None);
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem.iter().find(|u| &u.id == id).cloned())
        })
    }

    fn find_by_email<'a>(
        &'a self,
        email: &'a UserEmail,
    ) -> Pin<Box<dyn Future<Output = AppResult<Option<User>>> + Send + 'a>> {
        Box::pin(async move {
            #[cfg(feature = "sqlx")]
            if let Some(pool) = &self.pool {
                let row = sqlx::query(
                    r#"
                    SELECT id, email, password_hash, display_name, created_at::text as created_at
                    FROM users
                    WHERE email = $1
                    "#,
                )
                .bind(email.as_str())
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to fetch user by email: {}", e)))?;

                if let Some(r) = row {
                    let uid = UserId::new(r.get::<String, _>("id"));
                    let uemail = UserEmail::new(r.get::<String, _>("email"))?;
                    let pass_hash = PasswordHash::new(r.get::<String, _>("password_hash"))?;
                    let dname = DisplayName::new(r.get::<String, _>("display_name"))?;
                    let created_at: String = r.get("created_at");
                    return Ok(Some(User::reconstitute(
                        uid, uemail, pass_hash, dname, created_at,
                    )));
                }
                return Ok(None);
            }

            let mem = self.fallback.lock().unwrap();
            Ok(mem.iter().find(|u| &u.email == email).cloned())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pg_user_repository_in_memory() {
        let repo = PgUserRepository::in_memory();

        let id = UserId::generate();
        let email = UserEmail::new("user@example.com").unwrap();
        let hash = PasswordHash::new("$2b$12$dummyhash").unwrap();
        let name = DisplayName::new("Test User").unwrap();

        let user = User::new(id.clone(), email.clone(), hash, name.clone());
        repo.save(&user).await.unwrap();

        let found = repo.find_by_id(&id).await.unwrap().unwrap();
        assert_eq!(found.id, id);
        assert_eq!(found.display_name, name);

        let found_email = repo.find_by_email(&email).await.unwrap().unwrap();
        assert_eq!(found_email.id, id);
    }

    #[tokio::test]
    async fn test_pg_user_repository_real_postgres() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let repo = PgUserRepository::new(pool);
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let id = UserId::new(format!("usr_test_{:x}", ts));
        let email = UserEmail::new(format!("test_user_{:x}@example.com", ts)).unwrap();
        let hash = PasswordHash::new("$2b$12$realhashvalue1234567890").unwrap();
        let name = DisplayName::new("Real Test User").unwrap();

        let user = User::new(id.clone(), email.clone(), hash, name.clone());
        repo.save(&user).await.unwrap();

        let found = repo
            .find_by_id(&id)
            .await
            .unwrap()
            .expect("User should exist in DB");
        assert_eq!(found.id, id);
        assert_eq!(found.email, email);
        assert_eq!(found.display_name, name);
    }
}
