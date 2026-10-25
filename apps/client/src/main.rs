#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .thread_stack_size(8 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main());
}

#[cfg(feature = "ssr")]
async fn download_deliverable_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
    headers: axum::http::HeaderMap,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let cookie_header = headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok());

    let token = cookie_header
        .and_then(auth::extract_token_from_cookie_header)
        .ok_or_else(|| (axum::http::StatusCode::UNAUTHORIZED, "No session token".to_string()))?;

    let config = config::ServerAppConfig::from_env();
    let pool = database::DatabasePool::connect(&config.database_url)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user_repo = pool.user_repository();
    let member_repo = pool.member_repository();
    let tenant_repo = pool.tenant_repository();

    let use_case = application::GetClientSessionUseCase::new(
        &user_repo,
        &member_repo,
        &tenant_repo,
        &config.jwt_secret,
    );

    let session = use_case
        .execute(token)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (axum::http::StatusCode::UNAUTHORIZED, "Invalid session".to_string()))?;

    let membership = session.memberships.first()
        .ok_or_else(|| (axum::http::StatusCode::FORBIDDEN, "No tenant membership".to_string()))?;

    let deliverable = pool.portal_repository()
        .get_deliverable_for_download(&membership.tenant_id, &id)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, "Deliverable not found or unauthorized".to_string()))?;

    // Record tamper-evident audit log
    let _ = pool.portal_repository().record_audit_log(
        Some(&membership.tenant_id),
        &session.user.id,
        &session.user.display_name,
        "Client",
        "DELIVERABLE_DOWNLOADED",
        "Deliverable",
        &id,
        Some(&format!("File: {}", deliverable.file_name)),
    ).await;

    let content = format!(
        "=================================================================\n\
         TEEBTECH AUTHORIZED CLIENT DELIVERABLE\n\
         Zero-Trust Boundary: Verified Tenant Signature\n\
         =================================================================\n\
         Artifact ID   : {}\n\
         Title         : {}\n\
         File Name     : {}\n\
         Version       : {}\n\
         Status        : {}\n\
         Tenant        : {} ({})\n\
         Authorized To : {} ({})\n\
         Timestamp     : {}\n\
         =================================================================\n\n\
         CONFIDENTIAL & PROPRIETARY — STRICTLY FOR CLIENT WORKSPACE USE.\n\
         TeebTech Enterprise Architecture Squad\n",
        deliverable.id,
        deliverable.title,
        deliverable.file_name,
        deliverable.version,
        deliverable.status,
        membership.tenant_name,
        membership.tenant_id,
        session.user.display_name,
        session.user.email,
        deliverable.published_at,
    );

    let response = axum::response::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, "application/octet-stream")
        .header(
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", deliverable.file_name),
        )
        .body(axum::body::Body::from(content))
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(response)
}

#[cfg(feature = "ssr")]
async fn async_main() {
    use axum::Router;
    use client_app::app::*;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos::server_fn::ServerFn;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(Some("apps/client/Cargo.toml"))
        .or_else(|_| get_configuration(None))
        .unwrap();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3002));
    let mut leptos_options = conf.leptos_options;
    leptos_options.site_addr = addr;
    leptos_options.site_root = "target/site/client".into();
    leptos_options.output_name = "client".into();
    let routes = generate_route_list(App);

    log!(
        "GetClientSession URL: {}",
        <GetClientSessionServerFn as ServerFn>::url()
    );
    log!(
        "ClientLogout URL: {}",
        <ClientLogoutServerFn as ServerFn>::url()
    );

    let app = Router::new()
        .route("/health", axum::routing::get(|| async {
            ([(axum::http::header::CONTENT_TYPE, "application/json")], r#"{"status":"healthy","app":"client-app","version":"0.1.0"}"#)
        }))
        .route("/api/deliverables/{id}/download", axum::routing::get(download_deliverable_handler))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("TeebTech Client Portal listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
