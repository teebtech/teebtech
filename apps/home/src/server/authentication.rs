use application::CurrentUserDto;
use leptos::prelude::*;

/// Server function to register a new user identity and set an HttpOnly session cookie.
#[server(RegisterUserServerFn, "/api")]
pub async fn register_user(
    email: String,
    password: String,
    display_name: String,
) -> Result<CurrentUserDto, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use application::{RegisterUserDto, RegisterUserUseCase};
        use config::ServerAppConfig;
        use database::DatabasePool;
        use leptos_axum::ResponseOptions;

        let config = ServerAppConfig::from_env();
        let pool = DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| DatabasePool::new(&config.database_url));
        let user_repo = pool.user_repository();

        let use_case = RegisterUserUseCase::new(&user_repo, &config.jwt_secret);
        let auth_res = use_case
            .execute(RegisterUserDto {
                email,
                password,
                display_name,
            })
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        if let Some(resp) = use_context::<ResponseOptions>() {
            let cookie_val = auth::build_session_cookie(
                &auth_res.token,
                auth_res.expires_in,
                config.public.env == "production",
            );
            resp.insert_header(
                axum::http::header::SET_COOKIE,
                axum::http::HeaderValue::from_str(&cookie_val)
                    .map_err(|e| ServerFnError::new(e.to_string()))?,
            );
        }

        Ok(CurrentUserDto {
            id: auth_res.id,
            email: auth_res.email,
            display_name: auth_res.display_name,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (email, password, display_name);
        unreachable!()
    }
}

/// Server function to verify credentials and establish an HttpOnly session cookie.
#[server(LoginServerFn, "/api")]
pub async fn login(email: String, password: String) -> Result<CurrentUserDto, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use application::{LoginDto, LoginUseCase};
        use config::ServerAppConfig;
        use database::DatabasePool;
        use leptos_axum::ResponseOptions;

        let config = ServerAppConfig::from_env();
        let pool = DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| DatabasePool::new(&config.database_url));
        let user_repo = pool.user_repository();

        let use_case = LoginUseCase::new(&user_repo, &config.jwt_secret);
        let auth_res = use_case
            .execute(LoginDto { email, password })
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        if let Some(resp) = use_context::<ResponseOptions>() {
            let cookie_val = auth::build_session_cookie(
                &auth_res.token,
                auth_res.expires_in,
                config.public.env == "production",
            );
            resp.insert_header(
                axum::http::header::SET_COOKIE,
                axum::http::HeaderValue::from_str(&cookie_val)
                    .map_err(|e| ServerFnError::new(e.to_string()))?,
            );
        }

        Ok(CurrentUserDto {
            id: auth_res.id,
            email: auth_res.email,
            display_name: auth_res.display_name,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (email, password);
        unreachable!()
    }
}

/// Server function to clear the HttpOnly session cookie.
#[server(LogoutServerFn, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;

        if let Some(resp) = use_context::<ResponseOptions>() {
            let cookie_val = auth::build_logout_cookie();
            resp.insert_header(
                axum::http::header::SET_COOKIE,
                axum::http::HeaderValue::from_str(&cookie_val)
                    .map_err(|e| ServerFnError::new(e.to_string()))?,
            );
        }

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

/// Server function to resolve the current authenticated user from request cookie header.
#[server(GetCurrentUserServerFn, "/api")]
pub async fn get_current_user() -> Result<Option<CurrentUserDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use application::GetCurrentUserUseCase;
        use axum::http::HeaderMap;
        use config::ServerAppConfig;
        use database::DatabasePool;

        let headers: HeaderMap = leptos_axum::extract()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let cookie_header = headers
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok());

        let token = match cookie_header.and_then(auth::extract_token_from_cookie_header) {
            Some(t) => t,
            None => return Ok(None),
        };

        let config = ServerAppConfig::from_env();
        let pool = DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| DatabasePool::new(&config.database_url));
        let user_repo = pool.user_repository();

        let use_case = GetCurrentUserUseCase::new(&user_repo, &config.jwt_secret);
        let user = use_case
            .execute(token)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(user)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
