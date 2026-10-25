//! # Database Crate
//!
//! Persistence boundary, PostgreSQL connection pool management, and repository implementations.

pub mod members;
pub mod portal;
pub mod tenants;
pub mod users;

pub use members::*;
pub use portal::*;
pub use tenants::*;
pub use users::*;

use shared::{AppError, AppResult};

#[cfg(feature = "sqlx")]
use sqlx::{postgres::PgPoolOptions, PgPool};

/// PostgreSQL database connection pool container.
#[derive(Debug, Clone)]
pub struct DatabasePool {
    pub connection_url: String,
    #[cfg(feature = "sqlx")]
    pool: Option<PgPool>,
}

impl DatabasePool {
    /// Creates a database pool container with a specified connection URL without establishing an immediate connection.
    pub fn new(connection_url: impl Into<String>) -> Self {
        Self {
            connection_url: connection_url.into(),
            #[cfg(feature = "sqlx")]
            pool: None,
        }
    }

    /// Establishes an asynchronous PostgreSQL connection pool using SQLx.
    #[cfg(feature = "sqlx")]
    pub async fn connect(connection_url: &str) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(connection_url)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to connect to PostgreSQL: {}", e)))?;

        Ok(Self {
            connection_url: connection_url.to_string(),
            pool: Some(pool),
        })
    }

    /// Returns a reference to the underlying SQLx PgPool if connected.
    #[cfg(feature = "sqlx")]
    pub fn inner(&self) -> Option<&PgPool> {
        self.pool.as_ref()
    }

    /// Creates a PgTenantRepository instance using the active connection pool if available, or in-memory fallback.
    pub fn tenant_repository(&self) -> PgTenantRepository {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            return PgTenantRepository::new(pool.clone());
        }
        PgTenantRepository::in_memory()
    }

    /// Creates a PgMemberRepository instance using the active connection pool if available, or in-memory fallback.
    pub fn member_repository(&self) -> PgMemberRepository {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            return PgMemberRepository::new(pool.clone());
        }
        PgMemberRepository::in_memory()
    }

    /// Creates a PgUserRepository instance using the active connection pool if available, or in-memory fallback.
    pub fn user_repository(&self) -> PgUserRepository {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            return PgUserRepository::new(pool.clone());
        }
        PgUserRepository::in_memory()
    }

    /// Creates a PgPortalRepository instance using the active connection pool if available, or in-memory fallback.
    pub fn portal_repository(&self) -> PgPortalRepository {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            return PgPortalRepository::new(pool.clone());
        }
        PgPortalRepository::in_memory()
    }

    /// Verifies connection liveness to the database.
    pub async fn ping(&self) -> AppResult<()> {
        #[cfg(feature = "sqlx")]
        {
            if let Some(pool) = &self.pool {
                sqlx::query("SELECT 1")
                    .execute(pool)
                    .await
                    .map_err(|e| AppError::Internal(format!("PostgreSQL ping failed: {}", e)))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_pool_new() {
        let pool = DatabasePool::new("postgres://postgres:postgres@localhost:5433/teebtech_dev");
        assert_eq!(
            pool.connection_url,
            "postgres://postgres:postgres@localhost:5433/teebtech_dev"
        );
        #[cfg(feature = "sqlx")]
        assert!(pool.inner().is_none());
    }

    #[tokio::test]
    async fn test_database_pool_ping_unconnected() {
        let pool = DatabasePool::new("postgres://localhost/test");
        assert!(pool.ping().await.is_ok());
    }
}
