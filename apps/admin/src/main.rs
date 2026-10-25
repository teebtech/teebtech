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
async fn async_main() {
    use admin_app::app::*;
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(Some("apps/admin/Cargo.toml"))
        .or_else(|_| get_configuration(None))
        .unwrap();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3004));
    let mut leptos_options = conf.leptos_options;
    leptos_options.site_addr = addr;
    leptos_options.site_root = "target/site/admin".into();
    leptos_options.output_name = "admin".into();
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/health", axum::routing::get(|| async {
            ([(axum::http::header::CONTENT_TYPE, "application/json")], r#"{"status":"healthy","app":"admin-app","version":"0.1.0"}"#)
        }))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("TeebTech Admin Portal listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
