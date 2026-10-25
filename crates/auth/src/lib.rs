//! # Auth Crate
//!
//! Identity, authentication tokens, session credentials, cookie management, and password hashing.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use shared::{AppError, AppResult};

/// Standard cookie name used for session authentication.
pub const AUTH_COOKIE_NAME: &str = "teebtech_session";

/// Authentication token container.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

/// Decoded authentication claims from signed session tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuthClaims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

/// Builds an HttpOnly, SameSite=Lax session cookie header value.
pub fn build_session_cookie(token: &str, max_age_secs: u64, secure: bool) -> String {
    let mut cookie = format!(
        "{}={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}",
        AUTH_COOKIE_NAME, token, max_age_secs
    );
    if secure {
        cookie.push_str("; Secure");
    }
    cookie
}

/// Builds an expired cookie header value to invalidate browser session.
pub fn build_logout_cookie() -> String {
    format!(
        "{}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0",
        AUTH_COOKIE_NAME
    )
}

/// Extracts the raw token value from an HTTP Cookie header string.
pub fn extract_token_from_cookie_header(cookie_header: &str) -> Option<&str> {
    for cookie in cookie_header.split(';') {
        let trimmed = cookie.trim();
        if let Some((name, val)) = trimmed.split_once('=') {
            if name.trim() == AUTH_COOKIE_NAME {
                let token_val = val.trim();
                if !token_val.is_empty() {
                    return Some(token_val);
                }
            }
        }
    }
    None
}

/// Secure password hashing and verification service using bcrypt.
pub struct PasswordService;

impl PasswordService {
    /// Hashes a plaintext password using bcrypt with standard work cost factor.
    pub fn hash_password(plain: &str) -> AppResult<String> {
        #[cfg(feature = "bcrypt")]
        {
            bcrypt::hash(plain, bcrypt::DEFAULT_COST)
                .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))
        }
        #[cfg(not(feature = "bcrypt"))]
        {
            let _ = plain;
            Err(AppError::Internal("Bcrypt feature disabled".to_string()))
        }
    }

    /// Verifies a plaintext password against a stored bcrypt hash.
    pub fn verify_password(plain: &str, hash: &str) -> AppResult<bool> {
        #[cfg(feature = "bcrypt")]
        {
            bcrypt::verify(plain, hash)
                .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))
        }
        #[cfg(not(feature = "bcrypt"))]
        {
            let _ = (plain, hash);
            Err(AppError::Internal("Bcrypt feature disabled".to_string()))
        }
    }
}

/// Signed session token generation and verification service.
pub struct TokenService;

impl TokenService {
    /// Creates a signed HMAC-SHA256 JWT session token.
    pub fn create_session_token(
        user_id: &str,
        email: &str,
        secret: &str,
        duration_secs: u64,
    ) -> AppResult<AuthToken> {
        #[cfg(feature = "jsonwebtoken")]
        {
            use jsonwebtoken::{encode, EncodingKey, Header};

            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as usize;

            let claims = AuthClaims {
                sub: user_id.to_string(),
                email: email.to_string(),
                exp: now + (duration_secs as usize),
                iat: now,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )
            .map_err(|e| AppError::Internal(format!("Token creation failed: {}", e)))?;

            Ok(AuthToken {
                access_token: token,
                token_type: "Bearer".to_string(),
                expires_in: duration_secs,
            })
        }
        #[cfg(not(feature = "jsonwebtoken"))]
        {
            let _ = (user_id, email, secret, duration_secs);
            Err(AppError::Internal("JWT feature disabled".to_string()))
        }
    }

    /// Validates and decodes a signed HMAC-SHA256 JWT session token.
    pub fn verify_session_token(token: &str, secret: &str) -> AppResult<AuthClaims> {
        #[cfg(feature = "jsonwebtoken")]
        {
            use jsonwebtoken::{decode, DecodingKey, Validation};

            let data = decode::<AuthClaims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            )
            .map_err(|e| AppError::Unauthorized(format!("Invalid or expired session: {}", e)))?;

            Ok(data.claims)
        }
        #[cfg(not(feature = "jsonwebtoken"))]
        {
            let _ = (token, secret);
            Err(AppError::Internal("JWT feature disabled".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "SuperSecretPassword123!";
        let hash = PasswordService::hash_password(password).unwrap();

        assert_ne!(password, hash);
        assert!(PasswordService::verify_password(password, &hash).unwrap());
        assert!(!PasswordService::verify_password("WrongPassword!", &hash).unwrap());
    }

    #[test]
    fn test_session_token_creation_and_verification() {
        let secret = "super_secret_jwt_key_minimum_32_bytes_long";
        let token =
            TokenService::create_session_token("usr_123", "alice@example.com", secret, 3600)
                .unwrap();

        let claims = TokenService::verify_session_token(&token.access_token, secret).unwrap();
        assert_eq!(claims.sub, "usr_123");
        assert_eq!(claims.email, "alice@example.com");

        // Invalid secret should fail
        assert!(TokenService::verify_session_token(&token.access_token, "wrong_secret").is_err());
    }

    #[test]
    fn test_cookie_helpers() {
        let cookie = build_session_cookie("sample_token_123", 86400, false);
        assert!(cookie.contains("teebtech_session=sample_token_123"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Path=/"));
        assert!(cookie.contains("Max-Age=86400"));

        let logout_cookie = build_logout_cookie();
        assert!(logout_cookie.contains("teebtech_session="));
        assert!(logout_cookie.contains("Max-Age=0"));

        let header = "other_cookie=xyz; teebtech_session=extracted_jwt_token_abc; theme=dark";
        assert_eq!(
            extract_token_from_cookie_header(header),
            Some("extracted_jwt_token_abc")
        );
        assert_eq!(extract_token_from_cookie_header("other_cookie=xyz"), None);
    }
}
