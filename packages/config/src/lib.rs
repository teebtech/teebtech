//! # Config Package
//!
//! Strict configuration boundaries for TeebTech platform.
//!
//! ## Security Invariants
//! 1. **PublicAppConfig**: Contains ONLY safe, client-visible configuration. Safe for WASM hydration and browser execution.
//! 2. **ServerAppConfig**: Contains server secrets, database URLs, token signing keys, and private integration credentials.
//!    This struct is strictly restricted to server execution and must NEVER be exposed or serialized to client WASM.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Browser-safe application configuration.
///
/// Contains only non-sensitive public metadata and endpoints intended for client hydration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PublicAppConfig {
    pub env: String,
    pub app_name: String,
    pub api_base_url: String,
}

impl Default for PublicAppConfig {
    fn default() -> Self {
        Self {
            env: "development".to_string(),
            app_name: "TeebTech".to_string(),
            api_base_url: "/api".to_string(),
        }
    }
}

/// Server-only configuration container.
///
/// Holds secrets, database connection parameters, and sensitive credentials.
/// Restricted to non-WASM server targets.
#[cfg(all(feature = "server", not(target_arch = "wasm32")))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerAppConfig {
    pub public: PublicAppConfig,
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

#[cfg(all(feature = "server", not(target_arch = "wasm32")))]
impl Default for ServerAppConfig {
    fn default() -> Self {
        Self {
            public: PublicAppConfig::default(),
            host: "127.0.0.1".to_string(),
            port: 3000,
            database_url: "postgres://postgres:postgres@localhost:5433/teebtech_dev".to_string(),
            jwt_secret: "dev_jwt_secret_change_in_production_32_bytes_min".to_string(),
        }
    }
}

#[cfg(all(feature = "server", not(target_arch = "wasm32")))]
impl ServerAppConfig {
    /// Loads configuration from environment variables, falling back to development defaults.
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();

        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev".to_string()
        });
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "dev_jwt_secret_change_in_production_32_bytes_min".to_string());

        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "TeebTech".to_string());
        let api_base_url = std::env::var("API_BASE_URL").unwrap_or_else(|_| "/api".to_string());

        Self {
            public: PublicAppConfig {
                env,
                app_name,
                api_base_url,
            },
            host,
            port,
            database_url,
            jwt_secret,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_config_defaults() {
        let config = PublicAppConfig::default();
        assert_eq!(config.env, "development");
        assert_eq!(config.app_name, "TeebTech");
        assert_eq!(config.api_base_url, "/api");
    }

    #[cfg(all(feature = "server", not(target_arch = "wasm32")))]
    #[test]
    fn test_server_config_from_env() {
        let config = ServerAppConfig::from_env();
        assert!(!config.database_url.is_empty());
        assert!(!config.jwt_secret.is_empty());
        assert_eq!(config.public.app_name, "TeebTech");
    }
}
