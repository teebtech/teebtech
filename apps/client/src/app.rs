use application::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{A, Outlet, ParentRoute, Route, Router, Routes},
    hooks::{use_location, use_navigate},
    StaticSegment,
};
use ui::{Badge, Button, Card, Icon, PortalLayout, PortalNavItem, StatCard};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-slate-50 text-slate-900 antialiased min-h-screen font-sans">
                <App/>
            </body>
        </html>
    }
}

// ─── SERVER FUNCTIONS ────────────────────────────────────────────────────────

#[cfg(feature = "ssr")]
async fn resolve_client_auth_context() -> Result<(String, String, String), ServerFnError> {
    use axum::http::HeaderMap;
    use config::ServerAppConfig;
    use database::DatabasePool;
    use application::GetClientSessionUseCase;

    let headers: HeaderMap = leptos_axum::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("Header extraction failed: {}", e)))?;

    let cookie_header = headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok());

    let token = cookie_header
        .and_then(auth::extract_token_from_cookie_header)
        .ok_or_else(|| ServerFnError::new("Unauthorized: No session token found"))?;

    let config = ServerAppConfig::from_env();
    let pool = DatabasePool::connect(&config.database_url)
        .await
        .map_err(|e| ServerFnError::new(format!("Database connect failed: {}", e)))?;

    let user_repo = pool.user_repository();
    let member_repo = pool.member_repository();
    let tenant_repo = pool.tenant_repository();

    let use_case = GetClientSessionUseCase::new(
        &user_repo,
        &member_repo,
        &tenant_repo,
        &config.jwt_secret,
    );

    let session = use_case
        .execute(token)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Unauthorized: Session invalid or expired"))?;

    let membership = session.memberships.first()
        .ok_or_else(|| ServerFnError::new("Unauthorized: No active tenant membership"))?;

    Ok((membership.tenant_id.clone(), session.user.id, session.user.display_name))
}

#[server(GetClientSessionServerFn, "/api")]
pub async fn get_client_session() -> Result<Option<ClientSessionDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::http::HeaderMap;
        use config::ServerAppConfig;
        use database::DatabasePool;
        use application::GetClientSessionUseCase;

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
            .map_err(|e| ServerFnError::new(format!("Database connect failed: {}", e)))?;
        let user_repo = pool.user_repository();
        let member_repo = pool.member_repository();
        let tenant_repo = pool.tenant_repository();

        let use_case = GetClientSessionUseCase::new(
            &user_repo,
            &member_repo,
            &tenant_repo,
            &config.jwt_secret,
        );

        let res = use_case
            .execute(token)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(res)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetClientOverviewServerFn, "/api")]
pub async fn get_client_overview() -> Result<ClientOverviewDto, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_client_overview(&tenant_id, &uname)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetClientServicesServerFn, "/api")]
pub async fn get_client_services() -> Result<Vec<ClientServiceItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_client_services(&tenant_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetClientProjectsServerFn, "/api")]
pub async fn get_client_projects() -> Result<Vec<ProjectItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_client_projects(&tenant_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetServiceCatalogServerFn, "/api")]
pub async fn get_service_catalog() -> Result<Vec<CatalogServiceDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_service_catalog()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetClientChangeRequestsServerFn, "/api")]
pub async fn get_client_change_requests() -> Result<Vec<ChangeRequestItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_client_change_requests(&tenant_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(CreateChangeRequestServerFn, "/api")]
pub async fn create_change_request(
    project_id: Option<String>,
    title: String,
    scope_description: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, uid, uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .create_change_request(
                &tenant_id,
                &uid,
                &uname,
                project_id,
                &title,
                &scope_description,
            )
            .await
            .map(|_| ())
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(ClientDecideChangeRequestServerFn, "/api")]
pub async fn client_decide_change_request(
    request_id: String,
    approved: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, uid, uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .client_decide_change_request(&tenant_id, &request_id, approved, &uid, &uname)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(OrderServicePackageServerFn, "/api")]
pub async fn order_service_package(
    service_id: String,
    package_name: String,
    notes: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, uid, uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .order_service_package(&tenant_id, &uid, &uname, &service_id, &package_name, &notes)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(CreateClientConversationServerFn, "/api")]
pub async fn create_client_conversation(
    title: String,
    channel_type: String,
    initial_message: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, uid, uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .create_client_conversation(&tenant_id, &uid, &uname, &title, &channel_type, &initial_message)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(MarkClientNotificationsReadServerFn, "/api")]
pub async fn mark_client_notifications_read() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .mark_client_notifications_read(&tenant_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(UpdateClientProfileServerFn, "/api")]
pub async fn update_client_profile(display_name: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_tenant_id, uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .update_client_profile(&uid, &display_name)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetClientConversationsServerFn, "/api")]
pub async fn get_client_conversations() -> Result<Vec<ConversationItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        // is_staff = false strictly enforces filtering out internal notes
        pool.portal_repository()
            .get_client_conversations(&tenant_id, false)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(SendMessageServerFn, "/api")]
pub async fn send_client_message(
    conversation_id: String,
    body: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_tenant_id, uid, uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .send_message(&conversation_id, &uid, &uname, "Client Lead", &body, false)
            .await
            .map(|_| ())
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetClientDeliverablesServerFn, "/api")]
pub async fn get_client_deliverables() -> Result<Vec<DeliverableItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_client_deliverables(&tenant_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetClientInvoicesServerFn, "/api")]
pub async fn get_client_invoices() -> Result<Vec<InvoiceItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_client_invoices(&tenant_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetClientAuditLogsServerFn, "/api")]
pub async fn get_client_audit_logs() -> Result<Vec<AuditLogItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (tenant_id, _uid, _uname) = resolve_client_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_client_audit_logs(&tenant_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(ClientLogoutServerFn, "/api")]
pub async fn client_logout() -> Result<(), ServerFnError> {
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

// ─── ROUTING & TOP-LEVEL SHELL ───────────────────────────────────────────────

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/client.css"/>
        <Title text="TeebTech — Client Service Portal"/>

        <Router>
            <Routes fallback=ClientNotFoundView>
                <ParentRoute path=StaticSegment("") view=ClientPortalShell>
                    <Route path=StaticSegment("") view=ClientOverviewView/>
                    <Route path=StaticSegment("overview") view=ClientOverviewView/>
                    <Route path=StaticSegment("services") view=ClientServicesView/>
                    <Route path=StaticSegment("projects") view=ClientProjectsView/>
                    <Route path=StaticSegment("catalog") view=ClientCatalogView/>
                    <Route path=StaticSegment("requests") view=ClientRequestsView/>
                    <Route path=StaticSegment("messages") view=ClientMessagesView/>
                    <Route path=StaticSegment("deliverables") view=ClientDeliverablesView/>
                    <Route path=StaticSegment("files") view=ClientDeliverablesView/>
                    <Route path=StaticSegment("billing") view=ClientBillingView/>
                    <Route path=StaticSegment("invoices") view=ClientBillingView/>
                    <Route path=StaticSegment("activity") view=ClientHistoryView/>
                    <Route path=StaticSegment("history") view=ClientHistoryView/>
                    <Route path=StaticSegment("settings") view=ClientSettingsView/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn ClientPortalShell() -> impl IntoView {
    let session_resource = Resource::new(|| (), |_| async move { get_client_session().await });
    let logout_action = Action::new(move |_: &()| async move { client_logout().await });

    Effect::new(move |_| {
        if let Some(res) = logout_action.value().get() {
            if res.is_ok() {
                session_resource.refetch();
            }
        }
    });

    view! {
        <Suspense fallback=|| view! {
            <div class="min-h-screen flex items-center justify-center bg-slate-50">
                <div class="p-6 bg-white border border-slate-200 rounded-lg shadow-sm text-center">
                    <p class="text-xs text-slate-500 font-mono">"Verifying authenticated session boundary..."</p>
                </div>
            </div>
        }>
            {move || {
                session_resource.get().map(|res| match res {
                    Ok(Some(session)) => {
                        let is_logging_out = logout_action.pending().get();
                        view! {
                            <AuthenticatedClientPortal
                                session=session
                                is_logging_out=is_logging_out
                                on_logout=Callback::new(move |()| {
                                    logout_action.dispatch(());
                                })
                            />
                        }.into_any()
                    }
                    _ => view! {
                        <AccessDeniedView/>
                    }.into_any(),
                })
            }}
        </Suspense>
    }
}

// ─── AUTHENTICATED PORTAL WITH ROUTE-SYNCED SIDEBAR ──────────────────────────

#[component]
fn AuthenticatedClientPortal(
    session: ClientSessionDto,
    is_logging_out: bool,
    on_logout: Callback<()>,
) -> impl IntoView {
    provide_context(session.clone());
    let prefill_request = RwSignal::new(Option::<String>::None);
    provide_context(prefill_request);
    provide_context(on_logout);
    let is_logging_out_sig = RwSignal::new(is_logging_out);
    provide_context(is_logging_out_sig);

    let u_name = session.user.display_name.clone();
    let u_email = session.user.email.clone();
    let tenant_org = session.memberships.first()
        .map(|m| m.tenant_name.clone())
        .unwrap_or_else(|| "Acme Corporation".to_string());
    let tenant_role = session.memberships.first()
        .map(|m| m.role.clone())
        .unwrap_or_else(|| "Owner".to_string());

    let loc = use_location();
    let nav_items = move || {
        let cur = loc.pathname.get();
        let is_active = |prefix: &str, is_root: bool| {
            if is_root {
                cur == "/" || cur == "/overview"
            } else {
                cur == prefix || cur.starts_with(&format!("{}/", prefix))
            }
        };
        vec![
            PortalNavItem {
                label: "Overview".to_string(),
                label_ar: "نظرة عامة".to_string(),
                href: "/".to_string(),
                icon: "home".to_string(),
                active: is_active("/", true),
                badge: None,
            },
            PortalNavItem {
                label: "My Services".to_string(),
                label_ar: "خدماتي المشتركة".to_string(),
                href: "/services".to_string(),
                icon: "layers".to_string(),
                active: is_active("/services", false),
                badge: Some("Active".to_string()),
            },
            PortalNavItem {
                label: "Projects".to_string(),
                label_ar: "المشاريع الجارية".to_string(),
                href: "/projects".to_string(),
                icon: "briefcase".to_string(),
                active: is_active("/projects", false),
                badge: None,
            },
            PortalNavItem {
                label: "Service Catalog".to_string(),
                label_ar: "دليل الخدمات".to_string(),
                href: "/catalog".to_string(),
                icon: "globe".to_string(),
                active: is_active("/catalog", false),
                badge: None,
            },
            PortalNavItem {
                label: "Change Requests".to_string(),
                label_ar: "طلبات التعديل".to_string(),
                href: "/requests".to_string(),
                icon: "refresh".to_string(),
                active: is_active("/requests", false),
                badge: None,
            },
            PortalNavItem {
                label: "Messages & Support".to_string(),
                label_ar: "المحادثات والدعم".to_string(),
                href: "/messages".to_string(),
                icon: "messageSquare".to_string(),
                active: is_active("/messages", false),
                badge: None,
            },
            PortalNavItem {
                label: "Deliverables".to_string(),
                label_ar: "الملفات والمخرجات".to_string(),
                href: "/deliverables".to_string(),
                icon: "fileText".to_string(),
                active: is_active("/deliverables", false) || is_active("/files", false),
                badge: None,
            },
            PortalNavItem {
                label: "Billing & Invoices".to_string(),
                label_ar: "الفواتير والمدفوعات".to_string(),
                href: "/billing".to_string(),
                icon: "chart".to_string(),
                active: is_active("/billing", false) || is_active("/invoices", false),
                badge: None,
            },
            PortalNavItem {
                label: "Activity History".to_string(),
                label_ar: "سجل العمليات".to_string(),
                href: "/activity".to_string(),
                icon: "activity".to_string(),
                active: is_active("/activity", false) || is_active("/history", false),
                badge: None,
            },
            PortalNavItem {
                label: "Account & Settings".to_string(),
                label_ar: "إعدادات الحساب".to_string(),
                href: "/settings".to_string(),
                icon: "settings".to_string(),
                active: is_active("/settings", false),
                badge: None,
            },
        ]
    };

    view! {
        <PortalLayout
            portal_name="Client Portal".to_string()
            user_name=u_name.clone()
            user_email=u_email.clone()
            user_role=format!("{} ({})", tenant_role, tenant_org)
            is_admin=false
            nav_items=nav_items()
            on_logout=on_logout
        >
            {/* Quick Interactive Tab Bar for Instant Mobile/Tablet Access */}
            <div class="flex items-center gap-1.5 overflow-x-auto pb-2 border-b border-slate-200 text-xs">
                {vec![
                    ("/", "Overview", "home"),
                    ("/services", "My Services", "layers"),
                    ("/projects", "Projects", "briefcase"),
                    ("/catalog", "Catalog", "globe"),
                    ("/requests", "Change Requests", "refresh"),
                    ("/messages", "Messages", "messageSquare"),
                    ("/deliverables", "Deliverables", "fileText"),
                    ("/billing", "Billing", "chart"),
                    ("/activity", "Activity", "activity"),
                    ("/settings", "Settings", "settings"),
                ].into_iter().map(|(nav_path, title, icon)| {
                    let is_active = move || {
                        let cur = loc.pathname.get();
                        if nav_path == "/" {
                            cur == "/" || cur == "/overview"
                        } else {
                            cur == nav_path || cur.starts_with(&format!("{}/", nav_path))
                        }
                    };
                    view! {
                        <a
                            href=nav_path
                            class=move || format!(
                                "inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md font-medium transition-colors cursor-pointer whitespace-nowrap {}",
                                if is_active() {
                                    "bg-brand-600 text-white shadow-xs"
                                } else {
                                    "bg-white border border-slate-200 text-slate-600 hover:bg-slate-100 hover:text-slate-900"
                                }
                            )
                        >
                            <Icon name=icon.to_string() size=13 />
                            <span>{title}</span>
                        </a>
                    }
                }).collect_view()}
            </div>

            {/* Dynamic View Canvas via Nested Route Outlet */}
            <div class="mt-4">
                <Outlet/>
            </div>
        </PortalLayout>
    }
}

// ─── 1. OVERVIEW VIEW ────────────────────────────────────────────────────────

#[component]
fn ClientOverviewView() -> impl IntoView {
    let overview_resource = Resource::new(|| (), |_| async move { get_client_overview().await });
    let mark_read_action = Action::new(move |_: &()| async move { mark_client_notifications_read().await });

    Effect::new(move |_| {
        if let Some(res) = mark_read_action.value().get() {
            if res.is_ok() {
                overview_resource.refetch();
            }
        }
    });

    view! {
        <Suspense fallback=|| view! {
            <div class="py-12 text-center text-slate-400 font-mono text-xs">
                "Loading workspace overview metrics..."
            </div>
        }>
            {move || {
                overview_resource.get().map(|res| match res {
                    Ok(data) => {
                        view! {
                            <div class="space-y-6">
                                {/* Welcome Header */}
                                <div
                                    class="bg-slate-900 text-white rounded-xl p-6 sm:p-8 shadow-md flex flex-col md:flex-row md:items-center justify-between gap-4 border border-slate-800"
                                    style="background: linear-gradient(135deg, #090d16 0%, #0f172a 50%, #172554 100%);"
                                >
                                    <div class="space-y-1">
                                        <div class="inline-flex items-center gap-2 px-2.5 py-0.5 rounded-full bg-white/10 text-brand-300 text-xs font-mono">
                                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                                            <span>"Zero-Trust Tenant Boundary Active"</span>
                                        </div>
                                        <h2 class="text-xl sm:text-2xl font-bold tracking-tight">
                                            "Welcome back, " {data.client_name}
                                        </h2>
                                        <p class="text-xs text-slate-300 max-w-xl">
                                            "Workspace: " <span class="font-semibold text-white">{data.organization_name}</span>
                                            " — Track your enterprise digital transformations, request scope updates, review deliverables, and collaborate with your assigned engineering squads."
                                        </p>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <a
                                            href="/requests"
                                            class="px-3.5 py-2 rounded-md bg-brand-500 hover:bg-brand-600 text-white font-medium text-xs shadow-sm transition-colors text-decoration-none"
                                        >
                                            "+ Submit Request"
                                        </a>
                                        <a
                                            href="/catalog"
                                            class="px-3.5 py-2 rounded-md bg-white/10 hover:bg-white/20 text-white font-medium text-xs transition-colors text-decoration-none"
                                        >
                                            "Browse Catalog"
                                        </a>
                                    </div>
                                </div>

                                {/* Stat Cards Grid */}
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                                    <StatCard
                                        label="Active Services".to_string()
                                        value=data.active_services_count.to_string()
                                        delta="Enterprise SLA".to_string()
                                        delta_up=true
                                        icon="layers".to_string()
                                    />
                                    <StatCard
                                        label="Active Projects".to_string()
                                        value=data.active_projects_count.to_string()
                                        delta="In Progress".to_string()
                                        delta_up=true
                                        icon="briefcase".to_string()
                                    />
                                    <StatCard
                                        label="Pending Change Requests".to_string()
                                        value=data.pending_requests_count.to_string()
                                        delta="Under Review".to_string()
                                        delta_up=false
                                        icon="refresh".to_string()
                                    />
                                    <StatCard
                                        label="Unread Notifications".to_string()
                                        value=data.unread_notifications_count.to_string()
                                        delta="Live Updates".to_string()
                                        delta_up=true
                                        icon="activity".to_string()
                                    />
                                </div>

                                {/* Main Active Panels */}
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    {/* Subscribed Services Summary */}
                                    <Card
                                        title="Active Subscriptions & SLAs".to_string()
                                        subtitle="Services currently provisioned for this organization.".to_string()
                                    >
                                        {if data.recent_services.is_empty() {
                                            view! {
                                                <div class="py-8 text-center text-xs text-slate-500">
                                                    "No active services found. Explore the catalog to request solutions."
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="divide-y divide-slate-100">
                                                    {data.recent_services.into_iter().map(|s| view! {
                                                        <div class="py-3 flex items-center justify-between gap-3">
                                                            <div>
                                                                <p class="text-xs font-semibold text-slate-900">{s.title}</p>
                                                                <p class="text-[11px] text-slate-500">
                                                                    {s.package_name} " • Lead: " <span class="font-medium text-slate-700">{s.assigned_lead}</span>
                                                                </p>
                                                            </div>
                                                            <div class="flex items-center gap-2">
                                                                <Badge variant="cyan".to_string()>{s.sla_tier}</Badge>
                                                                <Badge variant="success".to_string() dot=true>{s.status}</Badge>
                                                            </div>
                                                        </div>
                                                    }).collect_view()}
                                                </div>
                                            }.into_any()
                                        }}
                                        <div class="pt-3 border-t border-slate-100">
                                            <a
                                                href="/services"
                                                class="text-xs text-brand-600 hover:text-brand-700 font-medium inline-flex items-center gap-1 text-decoration-none"
                                            >
                                                <span>"View all services & SLA terms"</span>
                                                <Icon name="arrowRight".to_string() size=12 />
                                            </a>
                                        </div>
                                    </Card>

                                    {/* Active Projects Summary */}
                                    <Card
                                        title="Active Projects & Milestones".to_string()
                                        subtitle="Engineering deliverables and roadmap progression.".to_string()
                                    >
                                        {if data.recent_projects.is_empty() {
                                            view! {
                                                <div class="py-8 text-center text-xs text-slate-500">
                                                    "No active projects in progress."
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="divide-y divide-slate-100">
                                                    {data.recent_projects.into_iter().map(|p| view! {
                                                        <div class="py-3 space-y-1.5">
                                                            <div class="flex items-center justify-between">
                                                                <p class="text-xs font-semibold text-slate-900">{p.title}</p>
                                                                <span class="text-xs font-mono font-semibold text-brand-600">{p.progress_pct} "%"</span>
                                                            </div>
                                                            <div class="w-full bg-slate-100 rounded-full h-1.5 overflow-hidden">
                                                                <div
                                                                    class="bg-brand-600 h-1.5 rounded-full"
                                                                    style=format!("width: {}%", p.progress_pct)
                                                                ></div>
                                                            </div>
                                                            <div class="flex items-center justify-between text-[11px] text-slate-500">
                                                                <span>"Squad: " {p.assigned_team}</span>
                                                                <Badge variant="navy".to_string()>{p.priority}</Badge>
                                                            </div>
                                                        </div>
                                                    }).collect_view()}
                                                </div>
                                            }.into_any()
                                        }}
                                        <div class="pt-3 border-t border-slate-100">
                                            <a
                                                href="/projects"
                                                class="text-xs text-brand-600 hover:text-brand-700 font-medium inline-flex items-center gap-1 text-decoration-none"
                                            >
                                                <span>"View all projects & sprint deliverables"</span>
                                                <Icon name="arrowRight".to_string() size=12 />
                                            </a>
                                        </div>
                                    </Card>
                                </div>

                                {/* Notifications Feed */}
                                <Card
                                    title="Recent Workspace Activity Notifications".to_string()
                                    subtitle="Real-time security and operational events.".to_string()
                                >
                                    <div class="flex items-center justify-between pb-3 mb-2 border-b border-slate-100">
                                        <span class="text-[11px] text-slate-500 font-mono">
                                            {format!("{} unread alerts pending", data.unread_notifications_count)}
                                        </span>
                                        {if data.unread_notifications_count > 0 {
                                            view! {
                                                <button
                                                    type="button"
                                                    on:click=move |_| { mark_read_action.dispatch(()); }
                                                    disabled=mark_read_action.pending().get()
                                                    class="text-[11px] font-semibold text-brand-600 hover:text-brand-700 bg-brand-50 hover:bg-brand-100 px-2.5 py-1 rounded transition-colors cursor-pointer"
                                                >
                                                    {if mark_read_action.pending().get() { "Updating..." } else { "✓ Mark All as Read" }}
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span class="text-[10px] text-emerald-600 font-medium">"All caught up"</span> }.into_any()
                                        }}
                                    </div>
                                    <div class="divide-y divide-slate-100">
                                        {data.recent_notifications.into_iter().map(|n| view! {
                                            <div class="py-3 flex items-start gap-3">
                                                <div class="p-1.5 rounded bg-brand-50 text-brand-600 mt-0.5">
                                                    <Icon name="activity".to_string() size=14 />
                                                </div>
                                                <div class="flex-1">
                                                    <div class="flex items-center justify-between">
                                                        <p class="text-xs font-semibold text-slate-900">{n.title}</p>
                                                        <span class="text-[10px] text-slate-400 font-mono">{n.created_at}</span>
                                                    </div>
                                                    <p class="text-xs text-slate-600 mt-0.5">{n.body}</p>
                                                </div>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                </Card>
                            </div>
                        }.into_any()
                    }
                    Err(e) => view! {
                        <div class="p-6 bg-red-50 border border-red-200 rounded-lg text-red-700 text-xs">
                            "Error loading client overview: " {e.to_string()}
                        </div>
                    }.into_any(),
                })
            }}
        </Suspense>
    }
}

// ─── 2. MY SERVICES VIEW ────────────────────────────────────────────────────

#[component]
fn ClientServicesView() -> impl IntoView {
    let prefill_request = use_context::<RwSignal<Option<String>>>().unwrap_or_else(|| RwSignal::new(None));
    let services_resource = Resource::new(|| (), |_| async move { get_client_services().await });

    view! {
        <div class="space-y-6">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                <div>
                    <h2 class="text-lg font-bold text-slate-900">"My Subscribed Services"</h2>
                    <p class="text-xs text-slate-500">"Active enterprise contracts, SLA guarantees, and dedicated engineering leads."</p>
                </div>
                <a
                    href="/catalog"
                    class="px-3.5 py-2 rounded-md bg-brand-600 hover:bg-brand-700 text-white font-medium text-xs shadow-xs transition-colors text-decoration-none"
                >
                    "+ Request New Service"
                </a>
            </div>

            <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading subscribed services..."</div> }>
                {move || {
                    services_resource.get().map(|res| match res {
                        Ok(services) => {
                            if services.is_empty() {
                                view! {
                                    <div class="p-8 text-center bg-white border border-dashed border-slate-200 rounded-lg">
                                        <p class="text-xs text-slate-500">"No services currently registered for this account."</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
                                        {services.into_iter().map(|s| {
                                            let s_title = s.title.clone();
                                            view! {
                                            <div class="p-5 bg-white border border-slate-200 rounded-xl shadow-xs space-y-4">
                                                <div class="flex items-start justify-between gap-3">
                                                    <div>
                                                        <span class="text-[10px] font-mono uppercase tracking-wider text-brand-600 font-semibold">{s.category}</span>
                                                        <h3 class="text-sm font-bold text-slate-900 mt-0.5">{s.title}</h3>
                                                        <p class="text-xs text-slate-500 font-arabic">{s.title_ar}</p>
                                                    </div>
                                                    <Badge variant="success".to_string() dot=true>{s.status}</Badge>
                                                </div>

                                                <div class="p-3 bg-slate-50 border border-slate-100 rounded-lg space-y-2 text-xs">
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-500">"Tier & Package:"</span>
                                                        <span class="font-semibold text-slate-900">{s.package_name}</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-500">"SLA Policy:"</span>
                                                        <Badge variant="cyan".to_string()>{s.sla_tier}</Badge>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-500">"Assigned Lead:"</span>
                                                        <span class="font-medium text-slate-800">{s.assigned_lead}</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-500">"Fee:"</span>
                                                        <span class="font-mono font-bold text-slate-900">{s.price} " " {s.currency}</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-500">"Started:"</span>
                                                        <span class="font-mono text-slate-600">{s.start_date}</span>
                                                    </div>
                                                </div>

                                                <div class="flex items-center gap-2 pt-1">
                                                    <button
                                                        type="button"
                                                        on:click={
                                                            let st = s_title.clone();
                                                            move |_| {
                                                                prefill_request.set(Some(format!("Scope Modification: {}", st)));
                                                                use_navigate()("/requests", Default::default());
                                                            }
                                                        }
                                                        class="flex-1 py-1.5 px-3 text-xs font-medium rounded border border-slate-200 text-slate-700 hover:bg-slate-50 transition-colors text-center cursor-pointer"
                                                    >
                                                        "Request Scope Modification"
                                                    </button>
                                                    <a
                                                        href="/messages"
                                                        class="py-1.5 px-3 text-xs font-medium rounded bg-brand-50 text-brand-700 hover:bg-brand-100 transition-colors text-center text-decoration-none"
                                                    >
                                                        "Message Squad"
                                                    </a>
                                                </div>
                                            </div>
                                        }}).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        Err(e) => view! {
                            <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                        }.into_any(),
                    })
                }}
            </Suspense>
        </div>
    }
}

// ─── 3. PROJECTS & MILESTONES VIEW ──────────────────────────────────────────

#[component]
fn ClientProjectsView() -> impl IntoView {
    let projects_resource = Resource::new(|| (), |_| async move { get_client_projects().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Active Projects & Delivery Milestones"</h2>
                <p class="text-xs text-slate-500">"Sprint velocity, deliverable roadmap, and milestone verification."</p>
            </div>

            <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading project roadmaps..."</div> }>
                {move || {
                    projects_resource.get().map(|res| match res {
                        Ok(projects) => {
                            view! {
                                <div class="space-y-6">
                                    {projects.into_iter().map(|p| view! {
                                        <div class="bg-white border border-slate-200 rounded-xl p-5 shadow-xs space-y-4">
                                            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                                                <div>
                                                    <div class="flex items-center gap-2">
                                                        <h3 class="text-base font-bold text-slate-900">{p.title}</h3>
                                                        <Badge variant="navy".to_string()>{p.priority}</Badge>
                                                        <Badge variant="success".to_string() dot=true>{p.status}</Badge>
                                                    </div>
                                                    <p class="text-xs text-slate-500 mt-1 max-w-2xl">{p.description}</p>
                                                </div>
                                                <div class="text-end">
                                                    <span class="text-xs font-mono text-slate-500">"Assigned Squad: "</span>
                                                    <span class="text-xs font-semibold text-slate-800">{p.assigned_team}</span>
                                                    {p.deadline.map(|d| view! {
                                                        <p class="text-[11px] font-mono text-slate-400 mt-0.5">"Target: " {d}</p>
                                                    })}
                                                </div>
                                            </div>

                                            {/* Progress Bar */}
                                            <div class="space-y-1">
                                                <div class="flex justify-between text-xs font-semibold text-slate-700">
                                                    <span>"Roadmap Progress"</span>
                                                    <span class="font-mono text-brand-600">{p.progress_pct} "% Completed"</span>
                                                </div>
                                                <div class="w-full bg-slate-100 rounded-full h-2 overflow-hidden">
                                                    <div
                                                        class="bg-brand-600 h-2 rounded-full transition-all duration-300"
                                                        style=format!("width: {}%", p.progress_pct)
                                                    ></div>
                                                </div>
                                            </div>

                                            {/* Milestones Checklist */}
                                            <div class="pt-3 border-t border-slate-100">
                                                <h4 class="text-xs font-bold uppercase tracking-wider text-slate-400 mb-3">
                                                    "Milestone Deliverables"
                                                </h4>
                                                <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                                                    {p.milestones.into_iter().map(|m| {
                                                        let is_done = m.status == "Completed";
                                                        let is_active = m.status == "In Progress";
                                                        view! {
                                                            <div class=format!(
                                                                "p-3 rounded-lg border text-xs flex items-start gap-2.5 {}",
                                                                if is_done {
                                                                    "bg-emerald-50/50 border-emerald-200 text-emerald-950"
                                                                } else if is_active {
                                                                    "bg-brand-50/50 border-brand-200 text-brand-950"
                                                                } else {
                                                                    "bg-slate-50 border-slate-200 text-slate-700"
                                                                }
                                                            )>
                                                                <div class="mt-0.5">
                                                                    {if is_done {
                                                                        view! { <Icon name="checkCircle".to_string() size=16 class="text-emerald-600".to_string() /> }.into_any()
                                                                    } else if is_active {
                                                                        view! { <span class="w-3.5 h-3.5 rounded-full bg-brand-500 animate-pulse block"></span> }.into_any()
                                                                    } else {
                                                                        view! { <Icon name="clock".to_string() size=14 class="text-slate-400".to_string() /> }.into_any()
                                                                    }}
                                                                </div>
                                                                <div class="flex-1 min-w-0">
                                                                    <div class="flex items-center justify-between">
                                                                        <p class="font-semibold truncate">{m.title}</p>
                                                                        <span class="text-[10px] font-mono font-medium">{m.status}</span>
                                                                    </div>
                                                                    <p class="text-[11px] opacity-80 mt-0.5">{m.description}</p>
                                                                    {m.due_date.map(|d| view! {
                                                                        <span class="text-[10px] font-mono text-slate-400 block mt-1">"Due: " {d}</span>
                                                                    })}
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                        Err(e) => view! {
                            <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                        }.into_any(),
                    })
                }}
            </Suspense>
        </div>
    }
}

// ─── 4. SERVICE CATALOG VIEW ────────────────────────────────────────────────

#[component]
fn ClientCatalogView() -> impl IntoView {
    let catalog_resource = Resource::new(|| (), |_| async move { get_service_catalog().await });
    let selected_package = RwSignal::new(Option::<(String, String, String, String, String)>::None);
    let order_notes = RwSignal::new(String::new());
    let order_feedback = RwSignal::new(Option::<String>::None);
    let is_ordering = RwSignal::new(false);

    let order_action = Action::new(move |(sid, pkg, notes): &(String, String, String)| {
        let s = sid.clone();
        let p = pkg.clone();
        let n = notes.clone();
        async move {
            order_service_package(s, p, n).await
        }
    });

    Effect::new(move |_| {
        if let Some(res) = order_action.value().get() {
            is_ordering.set(false);
            match res {
                Ok(cr_id) => {
                    order_feedback.set(Some(format!("Engagement order [{}] successfully created and sent for architecture review!", cr_id)));
                }
                Err(e) => {
                    order_feedback.set(Some(format!("Order failed: {}", e)));
                }
            }
        }
    });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"TeebTech Service Catalog"</h2>
                <p class="text-xs text-slate-500">"Explore bespoke software engineering, enterprise cloud architectures, cybersecurity, and AI solutions."</p>
            </div>

            {/* Service Order / Consultation Modal */}
            {move || selected_package.get().map(|(sid, stitle, _pid, pname, pprice)| {
                view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/50 backdrop-blur-xs">
                        <div class="bg-white rounded-xl shadow-xl border border-slate-200 max-w-lg w-full p-6 space-y-4">
                            <div class="flex items-start justify-between">
                                <div>
                                    <span class="text-[10px] font-mono uppercase tracking-wider text-brand-600 font-bold">"Official Service Engagement Request"</span>
                                    <h3 class="text-base font-bold text-slate-900">{stitle}</h3>
                                    <p class="text-xs text-slate-500">"Tier: " <strong class="text-slate-800">{pname.clone()}</strong> " • " <span class="font-mono">{pprice}</span></p>
                                </div>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        selected_package.set(None);
                                        order_feedback.set(None);
                                    }
                                    class="text-slate-400 hover:text-slate-600 text-lg cursor-pointer"
                                >
                                    "✕"
                                </button>
                            </div>

                            {move || order_feedback.get().map(|fb| view! {
                                <div class="p-3 bg-emerald-50 border border-emerald-200 text-emerald-900 text-xs rounded-md space-y-2">
                                    <p class="font-medium">{fb}</p>
                                    <a
                                        href="/requests"
                                        on:click=move |_| {
                                            selected_package.set(None);
                                            order_feedback.set(None);
                                        }
                                        class="inline-block px-2.5 py-1 bg-emerald-700 text-white rounded text-[11px] font-semibold text-decoration-none"
                                    >
                                        "View in Change Requests →"
                                    </a>
                                </div>
                            })}

                            <div class="space-y-1.5 text-xs">
                                <label class="font-semibold text-slate-700">"Specific Scope Requirements & Technical Expectations"</label>
                                <textarea
                                    rows=4
                                    placeholder="Detail any specific deployment constraints, cloud accounts, deadline targets, or compliance standards..."
                                    prop:value=move || order_notes.get()
                                    on:input=move |e| order_notes.set(event_target_value(&e))
                                    class="w-full px-3 py-2 border border-slate-300 rounded-md focus:ring-2 focus:ring-brand-500 focus:outline-none"
                                ></textarea>
                            </div>

                            <div class="flex items-center justify-end gap-2 pt-2 border-t border-slate-100">
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        selected_package.set(None);
                                        order_feedback.set(None);
                                    }
                                    class="px-3 py-1.5 text-xs font-medium text-slate-600 hover:bg-slate-100 rounded transition-colors cursor-pointer"
                                >
                                    "Cancel"
                                </button>
                                <button
                                    type="button"
                                    disabled=is_ordering.get()
                                    on:click={
                                        let sid_c = sid.clone();
                                        let pname_c = pname.clone();
                                        move |_| {
                                            is_ordering.set(true);
                                            order_action.dispatch((sid_c.clone(), pname_c.clone(), order_notes.get()));
                                        }
                                    }
                                    class="px-4 py-1.5 text-xs font-semibold text-white bg-brand-600 hover:bg-brand-700 rounded transition-colors shadow-xs cursor-pointer disabled:opacity-50"
                                >
                                    {if is_ordering.get() { "Submitting Order..." } else { "Submit Engagement Request" }}
                                </button>
                            </div>
                        </div>
                    </div>
                }
            })}

            <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading catalog packages..."</div> }>
                {move || {
                    catalog_resource.get().map(|res| match res {
                        Ok(catalog) => {
                            view! {
                                <div class="space-y-8">
                                    {catalog.into_iter().map(|srv| {
                                        let srv_id = srv.id.clone();
                                        let srv_title = srv.title.clone();
                                        let srv_title_display = srv.title.clone();
                                        view! {
                                        <div class="bg-white border border-slate-200 rounded-xl p-6 shadow-xs space-y-6">
                                            <div class="flex items-start justify-between gap-4">
                                                <div>
                                                    <span class="px-2 py-0.5 rounded text-[10px] font-semibold bg-brand-50 text-brand-700 font-mono">
                                                        {srv.category}
                                                    </span>
                                                    <h3 class="text-base font-bold text-slate-900 mt-1">{srv_title_display}</h3>
                                                    <p class="text-xs text-slate-500 font-arabic">{srv.title_ar}</p>
                                                    <p class="text-xs text-slate-600 mt-2 max-w-3xl leading-relaxed">{srv.description}</p>
                                                </div>
                                            </div>

                                             {/* Packages Comparison Grid */}
                                             <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 pt-2">
                                                 {srv.packages.into_iter().map(move |pkg| {
                                                     let s_id = srv_id.clone();
                                                     let s_title = srv_title.clone();
                                                     let p_id = pkg.id.clone();
                                                     let p_name = pkg.name.clone();
                                                     let p_price = format!("{} {} / {}", pkg.price, pkg.currency, pkg.billing_period);
                                                     view! {
                                                     <div class="p-4 rounded-lg border border-slate-200 bg-slate-50/50 hover:bg-slate-50 hover:border-brand-300 transition-colors flex flex-col justify-between">
                                                         <div class="space-y-3">
                                                             <div>
                                                                 <p class="text-xs font-bold text-slate-900">{pkg.name}</p>
                                                                 <p class="text-[11px] text-slate-500 font-arabic">{pkg.name_ar}</p>
                                                             </div>
                                                             <div>
                                                                 <span class="text-lg font-bold font-mono text-slate-900">{pkg.price} " " {pkg.currency}</span>
                                                                 <span class="text-[10px] text-slate-500 font-mono">" / " {pkg.billing_period}</span>
                                                             </div>
                                                             <ul class="space-y-1.5 text-xs text-slate-600">
                                                                 {pkg.features.into_iter().map(|f| view! {
                                                                     <li class="flex items-start gap-1.5">
                                                                         <Icon name="check".to_string() size=12 class="text-brand-600 mt-0.5 flex-shrink-0".to_string() />
                                                                         <span>{f}</span>
                                                                     </li>
                                                                 }).collect_view()}
                                                             </ul>
                                                         </div>
                                                         <div class="pt-4 mt-4 border-t border-slate-200">
                                                             <button
                                                                 type="button"
                                                                 on:click={
                                                                     let sid = s_id.clone();
                                                                     let st = s_title.clone();
                                                                     let pid = p_id.clone();
                                                                     let pn = p_name.clone();
                                                                     let pp = p_price.clone();
                                                                     move |_| {
                                                                         order_feedback.set(None);
                                                                         order_notes.set(String::new());
                                                                         selected_package.set(Some((sid.clone(), st.clone(), pid.clone(), pn.clone(), pp.clone())));
                                                                     }
                                                                 }
                                                                 class="w-full py-1.5 px-3 rounded bg-brand-600 hover:bg-brand-700 text-white text-xs font-semibold shadow-xs transition-colors cursor-pointer"
                                                             >
                                                                 "Request Consultation / Order"
                                                             </button>
                                                         </div>
                                                     </div>
                                                 }}).collect_view()}
                                             </div>
                                         </div>
                                     }}).collect_view()}
                                 </div>
                             }.into_any()
                         }
                         Err(e) => view! {
                             <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                         }.into_any(),
                     })
                 }}
             </Suspense>
         </div>
     }
 }

// ─── 5. CHANGE REQUESTS VIEW ────────────────────────────────────────────────

#[component]
fn ClientRequestsView() -> impl IntoView {
    let prefill_request = use_context::<RwSignal<Option<String>>>().unwrap_or_else(|| RwSignal::new(None));
    let requests_resource = Resource::new(|| (), |_| async move { get_client_change_requests().await });
    let projects_resource = Resource::new(|| (), |_| async move { get_client_projects().await });
    let selected_project_id = RwSignal::new(Option::<String>::None);
    let title_input = RwSignal::new(String::new());
    let scope_input = RwSignal::new(String::new());
    let is_submitting = RwSignal::new(false);
    let submit_feedback = RwSignal::new(Option::<String>::None);

    Effect::new(move |_| {
        if let Some(pf) = prefill_request.get() {
            title_input.set(pf);
        }
    });

    let submit_action = Action::new(move |(pid, title, scope): &(Option<String>, String, String)| {
        let p = pid.clone();
        let t = title.clone();
        let s = scope.clone();
        async move {
            create_change_request(p, t, s).await
        }
    });

    let decide_action = Action::new(move |(req_id, approved): &(String, bool)| {
        let r = req_id.clone();
        let a = *approved;
        async move {
            client_decide_change_request(r, a).await
        }
    });

    Effect::new(move |_| {
        if let Some(res) = submit_action.value().get() {
            is_submitting.set(false);
            match res {
                Ok(_) => {
                    submit_feedback.set(Some("Change request successfully submitted to architecture review.".to_string()));
                    title_input.set(String::new());
                    scope_input.set(String::new());
                    prefill_request.set(None);
                    requests_resource.refetch();
                }
                Err(e) => {
                    submit_feedback.set(Some(format!("Error: {}", e)));
                }
            }
        }
    });

    Effect::new(move |_| {
        if let Some(res) = decide_action.value().get() {
            if res.is_ok() {
                requests_resource.refetch();
            }
        }
    });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Change Requests & Scope Governance"</h2>
                <p class="text-xs text-slate-500">"Formally submit, review, and track architectural scope additions, SLA alterations, and feature quotations."</p>
            </div>

            {/* Submit New Change Request Form */}
            <Card
                title="Submit New Scope Change Request".to_string()
                subtitle="Provide technical requirements and expected outcomes for evaluation.".to_string()
            >
                <div class="space-y-4">
                    {move || submit_feedback.get().map(|fb| view! {
                        <div class="p-3 bg-brand-50 border border-brand-200 text-brand-800 text-xs rounded-md">
                            {fb}
                        </div>
                    })}

                    <div class="space-y-1.5">
                        <label class="block text-xs font-semibold text-slate-700">"Associated Delivery Project (Optional)"</label>
                        <select
                            on:change=move |e| {
                                let val = event_target_value(&e);
                                if val.is_empty() {
                                    selected_project_id.set(None);
                                } else {
                                    selected_project_id.set(Some(val));
                                }
                            }
                            class="w-full px-3 py-2 text-xs border border-slate-300 rounded-md focus:ring-2 focus:ring-brand-500 focus:outline-none bg-white"
                        >
                            <option value="">"General Organization Scope (No specific project)"</option>
                            {move || {
                                projects_resource.get().and_then(|res| res.ok()).map(|projs| {
                                    projs.into_iter().map(|p| view! {
                                        <option value=p.id.clone()>{p.title}</option>
                                    }).collect_view()
                                })
                            }}
                        </select>
                    </div>

                    <div class="space-y-1.5">
                        <label class="block text-xs font-semibold text-slate-700">"Request Title / Scope Name"</label>
                        <input
                            type="text"
                            placeholder="e.g. Implement Multi-Region Active-Active Database Failover"
                            prop:value=move || title_input.get()
                            on:input=move |e| title_input.set(event_target_value(&e))
                            class="w-full px-3 py-2 text-xs border border-slate-300 rounded-md focus:ring-2 focus:ring-brand-500 focus:outline-none"
                        />
                    </div>

                    <div class="space-y-1.5">
                        <label class="block text-xs font-semibold text-slate-700">"Detailed Scope & Acceptance Criteria"</label>
                        <textarea
                            rows=3
                            placeholder="Detail functional requirements, constraints, API integrations, and desired completion window..."
                            prop:value=move || scope_input.get()
                            on:input=move |e| scope_input.set(event_target_value(&e))
                            class="w-full px-3 py-2 text-xs border border-slate-300 rounded-md focus:ring-2 focus:ring-brand-500 focus:outline-none"
                        ></textarea>
                    </div>

                    <div>
                        <Button
                            variant="primary".to_string()
                            size="sm".to_string()
                            loading=is_submitting.get()
                            disabled=title_input.get().trim().is_empty() || scope_input.get().trim().is_empty()
                            class="cursor-pointer".to_string()
                        >
                            <span on:click=move |_| {
                                is_submitting.set(true);
                                submit_action.dispatch((selected_project_id.get(), title_input.get(), scope_input.get()));
                            }>
                                {if is_submitting.get() { "Submitting Request..." } else { "Submit for Architecture Review" }}
                            </span>
                        </Button>
                    </div>
                </div>
            </Card>

            {/* List of Requests */}
            <Card
                title="Submitted Change Requests Directory".to_string()
                subtitle="Formal status log, quotations, and decision milestones.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-6 text-center text-xs text-slate-400 font-mono">"Loading change requests..."</div> }>
                    {move || {
                        requests_resource.get().map(|res| match res {
                            Ok(requests) => {
                                if requests.is_empty() {
                                    view! {
                                        <div class="py-8 text-center text-xs text-slate-500">
                                            "No change requests logged for this workspace."
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="divide-y divide-slate-100">
                                            {requests.into_iter().map(|cr| {
                                                let badge_var = match cr.status.as_str() {
                                                    "Approved" => "success",
                                                    "Rejected" => "danger",
                                                    "Quoted" => "navy",
                                                    _ => "cyan",
                                                };
                                                let id_app = cr.id.clone();
                                                let id_rej = cr.id.clone();
                                                let is_quoted = cr.status == "Quoted";
                                                let is_deciding = decide_action.pending().get();

                                                view! {
                                                    <div class="py-4 space-y-2">
                                                        <div class="flex items-start justify-between gap-3">
                                                            <div>
                                                                <h4 class="text-xs font-bold text-slate-900">{cr.title}</h4>
                                                                <p class="text-[11px] text-slate-500">
                                                                    "By: " <span class="font-medium text-slate-700">{cr.requested_by_name}</span>
                                                                    " • Submitted: " <span class="font-mono">{cr.created_at}</span>
                                                                </p>
                                                            </div>
                                                            <Badge variant=badge_var.to_string() dot=true>{cr.status.clone()}</Badge>
                                                        </div>
                                                        <p class="text-xs text-slate-600 bg-slate-50 p-2.5 rounded border border-slate-100">
                                                            {cr.scope_description}
                                                        </p>
                                                        <div class="flex items-center gap-4 text-xs font-mono text-slate-500">
                                                            <span>"Estimated Cost: " <strong class="text-slate-900">{cr.estimated_cost.clone()} " " {cr.currency.clone()}</strong></span>
                                                            <span>"Delivery SLA: " <strong class="text-slate-900">{cr.estimated_days} " days"</strong></span>
                                                        </div>
                                                        {cr.review_notes.map(|rn| view! {
                                                            <div class="p-2 bg-brand-50/60 rounded text-[11px] text-brand-900">
                                                                <span class="font-semibold">"Engineering Review Notes: "</span>
                                                                {rn}
                                                            </div>
                                                        })}

                                                        {/* Client Decision Action for Quoted Requests */}
                                                        {if is_quoted {
                                                            let ia = id_app.clone();
                                                            let ir = id_rej.clone();
                                                            view! {
                                                                <div class="p-3 bg-brand-50/80 rounded-lg border border-brand-200 space-y-2 mt-2">
                                                                    <div class="flex items-center justify-between text-xs">
                                                                        <span class="font-bold text-brand-950">"Quotation Ready for Client Approval"</span>
                                                                        <span class="text-[11px] text-brand-800 font-mono">"Cost: " {cr.estimated_cost} " " {cr.currency} " • SLA: " {cr.estimated_days} " days"</span>
                                                                    </div>
                                                                    <div class="flex items-center gap-2 pt-1">
                                                                        <button
                                                                            type="button"
                                                                            disabled=is_deciding
                                                                            on:click=move |_| { decide_action.dispatch((ia.clone(), true)); }
                                                                            class="px-3 py-1.5 rounded bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-semibold shadow-xs transition-colors cursor-pointer disabled:opacity-50"
                                                                        >
                                                                            "✓ Approve Scope & Cost"
                                                                        </button>
                                                                        <button
                                                                            type="button"
                                                                            disabled=is_deciding
                                                                            on:click=move |_| { decide_action.dispatch((ir.clone(), false)); }
                                                                            class="px-3 py-1.5 rounded bg-rose-600 hover:bg-rose-700 text-white text-xs font-semibold shadow-xs transition-colors cursor-pointer disabled:opacity-50"
                                                                        >
                                                                            "✕ Reject Quotation"
                                                                        </button>
                                                                    </div>
                                                                </div>
                                                            }.into_any()
                                                        } else {
                                                            view! { <div></div> }.into_any()
                                                        }}
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(e) => view! {
                                <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                            }.into_any(),
                        })
                    }}
                </Suspense>
            </Card>
        </div>
    }
}

// ─── 6. MESSAGES & SUPPORT VIEW ─────────────────────────────────────────────

#[component]
fn ClientMessagesView() -> impl IntoView {
    let conv_resource = Resource::new(|| (), |_| async move { get_client_conversations().await });
    let message_input = RwSignal::new(String::new());
    let active_conv_id = RwSignal::new(Option::<String>::None);
    let is_sending = RwSignal::new(false);

    let show_new_channel_modal = RwSignal::new(false);
    let new_channel_title = RwSignal::new(String::new());
    let new_channel_type = RwSignal::new("General Technical Support".to_string());
    let new_initial_message = RwSignal::new(String::new());
    let is_creating_channel = RwSignal::new(false);

    let send_action = Action::new(move |(cid, body): &(String, String)| {
        let c = cid.clone();
        let b = body.clone();
        async move {
            send_client_message(c, b).await
        }
    });

    let create_conv_action = Action::new(move |(title, ctype, msg): &(String, String, String)| {
        let t = title.clone();
        let ct = ctype.clone();
        let m = msg.clone();
        async move {
            create_client_conversation(t, ct, m).await
        }
    });

    Effect::new(move |_| {
        if let Some(res) = send_action.value().get() {
            is_sending.set(false);
            if res.is_ok() {
                message_input.set(String::new());
                conv_resource.refetch();
            }
        }
    });

    Effect::new(move |_| {
        if let Some(res) = create_conv_action.value().get() {
            is_creating_channel.set(false);
            if let Ok(new_id) = res {
                active_conv_id.set(Some(new_id));
                show_new_channel_modal.set(false);
                new_channel_title.set(String::new());
                new_initial_message.set(String::new());
                conv_resource.refetch();
            }
        }
    });

    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-lg font-bold text-slate-900">"Messages & Technical Support"</h2>
                    <p class="text-xs text-slate-500">"Direct communication channel with your assigned TeebTech engineering squad. Zero-trust isolation enforced."</p>
                </div>
                <button
                    type="button"
                    on:click=move |_| show_new_channel_modal.set(true)
                    class="px-3.5 py-1.5 rounded-md bg-brand-600 hover:bg-brand-700 text-white text-xs font-semibold shadow-xs transition-colors cursor-pointer"
                >
                    "+ New Support Thread"
                </button>
            </div>

            {/* New Channel Modal */}
            {move || if show_new_channel_modal.get() {
                view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/50 backdrop-blur-xs">
                        <div class="bg-white rounded-xl shadow-xl border border-slate-200 max-w-md w-full p-6 space-y-4">
                            <div class="flex items-start justify-between">
                                <div>
                                    <h3 class="text-base font-bold text-slate-900">"Start New Support Thread"</h3>
                                    <p class="text-xs text-slate-500">"Open a secure channel directly with platform engineers."</p>
                                </div>
                                <button
                                    type="button"
                                    on:click=move |_| show_new_channel_modal.set(false)
                                    class="text-slate-400 hover:text-slate-600 text-lg cursor-pointer"
                                >
                                    "✕"
                                </button>
                            </div>

                            <div class="space-y-3 text-xs">
                                <div class="space-y-1">
                                    <label class="font-semibold text-slate-700">"Thread Title / Subject"</label>
                                    <input
                                        type="text"
                                        placeholder="e.g. CI/CD Pipeline Deployment Assistance"
                                        prop:value=move || new_channel_title.get()
                                        on:input=move |e| new_channel_title.set(event_target_value(&e))
                                        class="w-full px-3 py-2 border border-slate-300 rounded-md focus:ring-2 focus:ring-brand-500 focus:outline-none"
                                    />
                                </div>

                                <div class="space-y-1">
                                    <label class="font-semibold text-slate-700">"Channel Category"</label>
                                    <select
                                        on:change=move |e| new_channel_type.set(event_target_value(&e))
                                        class="w-full px-3 py-2 border border-slate-300 rounded-md focus:ring-2 focus:ring-brand-500 focus:outline-none bg-white"
                                    >
                                        <option value="General Technical Support">"General Technical Support"</option>
                                        <option value="Sprint Review & Milestones">"Sprint Review & Milestones"</option>
                                        <option value="Architecture Consultation">"Architecture Consultation"</option>
                                        <option value="Security & Compliance">"Security & Compliance"</option>
                                    </select>
                                </div>

                                <div class="space-y-1">
                                    <label class="font-semibold text-slate-700">"Initial Message"</label>
                                    <textarea
                                        rows=3
                                        placeholder="Detail your question, issue, or request for the squad..."
                                        prop:value=move || new_initial_message.get()
                                        on:input=move |e| new_initial_message.set(event_target_value(&e))
                                        class="w-full px-3 py-2 border border-slate-300 rounded-md focus:ring-2 focus:ring-brand-500 focus:outline-none"
                                    ></textarea>
                                </div>
                            </div>

                            <div class="flex items-center justify-end gap-2 pt-2 border-t border-slate-100">
                                <button
                                    type="button"
                                    on:click=move |_| show_new_channel_modal.set(false)
                                    class="px-3 py-1.5 text-xs font-medium text-slate-600 hover:bg-slate-100 rounded transition-colors cursor-pointer"
                                >
                                    "Cancel"
                                </button>
                                <button
                                    type="button"
                                    disabled=new_channel_title.get().trim().is_empty() || new_initial_message.get().trim().is_empty() || is_creating_channel.get()
                                    on:click=move |_| {
                                        is_creating_channel.set(true);
                                        create_conv_action.dispatch((
                                            new_channel_title.get(),
                                            new_channel_type.get(),
                                            new_initial_message.get()
                                        ));
                                    }
                                    class="px-4 py-1.5 text-xs font-semibold text-white bg-brand-600 hover:bg-brand-700 rounded transition-colors shadow-xs cursor-pointer disabled:opacity-50"
                                >
                                    {if is_creating_channel.get() { "Opening..." } else { "Create & Send" }}
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}

            <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading conversation channels..."</div> }>
                {move || {
                    conv_resource.get().map(|res| match res {
                        Ok(convs) => {
                            if convs.is_empty() {
                                view! {
                                    <div class="p-8 text-center bg-white border border-slate-200 rounded-lg">
                                        <p class="text-xs text-slate-500">"No conversation threads open yet. Click '+ New Support Thread' to get started."</p>
                                    </div>
                                }.into_any()
                            } else {
                                let selected_conv = {
                                    let cid = active_conv_id.get();
                                    convs.iter().find(|c| Some(c.id.clone()) == cid).cloned()
                                        .unwrap_or_else(|| convs.first().unwrap().clone())
                                };
                                let cur_cid = selected_conv.id.clone();

                                view! {
                                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 bg-white border border-slate-200 rounded-xl overflow-hidden shadow-xs min-h-[500px]">
                                        {/* Conversations Selector */}
                                        <div class="border-b md:border-b-0 md:border-r border-slate-200 p-4 space-y-2 bg-slate-50/50">
                                            <div class="flex items-center justify-between mb-2">
                                                <p class="text-[10px] font-bold uppercase tracking-wider text-slate-400">
                                                    "Support Channels"
                                                </p>
                                                <span class="text-[10px] font-mono text-slate-400">{format!("{} active", convs.len())}</span>
                                            </div>
                                            <div class="space-y-1">
                                                {convs.into_iter().map(|c| {
                                                    let is_sel = c.id == cur_cid;
                                                    let cid_clone = c.id.clone();
                                                    view! {
                                                        <button
                                                            type="button"
                                                            on:click=move |_| active_conv_id.set(Some(cid_clone.clone()))
                                                            class=format!(
                                                                "w-full text-start p-3 rounded-lg text-xs transition-colors cursor-pointer {}",
                                                                if is_sel { "bg-brand-50 border border-brand-200 text-brand-900 font-semibold" } else { "hover:bg-slate-100 text-slate-700" }
                                                            )
                                                        >
                                                            <p class="truncate">{c.title}</p>
                                                            <span class="text-[10px] text-slate-400 font-mono block mt-0.5">{c.channel_type}</span>
                                                        </button>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>

                                        {/* Messages Thread & Composer */}
                                        <div class="md:col-span-2 flex flex-col justify-between p-4 sm:p-6 space-y-4">
                                            {/* Thread Header */}
                                            <div class="border-b border-slate-100 pb-3 flex items-center justify-between">
                                                <div>
                                                    <h3 class="text-xs font-bold text-slate-900">{selected_conv.title}</h3>
                                                    <span class="text-[10px] font-mono text-slate-400">"Channel Type: " {selected_conv.channel_type}</span>
                                                </div>
                                                <Badge variant="cyan".to_string() dot=true>"Secure Thread"</Badge>
                                            </div>

                                            {/* Messages Stream */}
                                            <div class="flex-1 overflow-y-auto space-y-3 pr-2 max-h-[360px]">
                                                {if selected_conv.messages.is_empty() {
                                                    view! {
                                                        <p class="text-xs text-slate-400 text-center py-8">"No messages in this channel yet."</p>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <div class="space-y-3">
                                                            {selected_conv.messages.into_iter().map(|m| {
                                                                let is_client = m.sender_role.contains("Client") || m.sender_role.contains("Owner");
                                                                view! {
                                                                    <div class=format!(
                                                                        "p-3 rounded-lg text-xs max-w-lg space-y-1 {}",
                                                                        if is_client {
                                                                            "ms-auto bg-brand-600 text-white shadow-xs"
                                                                        } else {
                                                                            "me-auto bg-slate-100 text-slate-900 border border-slate-200"
                                                                        }
                                                                    )>
                                                                        <div class="flex items-center justify-between gap-3 text-[10px] opacity-80">
                                                                            <span class="font-bold">{m.sender_name} " (" {m.sender_role} ")"</span>
                                                                            <span class="font-mono">{m.sent_at}</span>
                                                                        </div>
                                                                        <p class="leading-relaxed">{m.body}</p>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    }.into_any()
                                                }}
                                            </div>

                                            {/* Composer */}
                                            <div class="pt-3 border-t border-slate-100 space-y-2">
                                                <div class="flex gap-2">
                                                    <input
                                                        type="text"
                                                        placeholder="Type your message to the engineering squad..."
                                                        prop:value=move || message_input.get()
                                                        on:input=move |e| message_input.set(event_target_value(&e))
                                                        on:keydown={
                                                            let cid_key = cur_cid.clone();
                                                            move |e| {
                                                                if e.key() == "Enter" && !message_input.get().trim().is_empty() {
                                                                    is_sending.set(true);
                                                                    send_action.dispatch((cid_key.clone(), message_input.get()));
                                                                }
                                                            }
                                                        }
                                                        class="flex-1 px-3 py-2 text-xs border border-slate-300 rounded-md focus:ring-2 focus:ring-brand-500 focus:outline-none"
                                                    />
                                                    <button
                                                        type="button"
                                                        disabled=message_input.get().trim().is_empty() || is_sending.get()
                                                        on:click={
                                                            let cid_send = cur_cid.clone();
                                                            move |_| {
                                                                is_sending.set(true);
                                                                send_action.dispatch((cid_send.clone(), message_input.get()));
                                                            }
                                                        }
                                                        class="px-4 py-2 bg-brand-600 text-white rounded-md text-xs font-semibold hover:bg-brand-700 disabled:opacity-50 transition-colors cursor-pointer"
                                                    >
                                                        {if is_sending.get() { "Sending..." } else { "Send" }}
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }
                        Err(e) => view! {
                            <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                        }.into_any(),
                    })
                }}
            </Suspense>
        </div>
    }
}

// ─── 7. FILES & DELIVERABLES VIEW ───────────────────────────────────────────

#[component]
fn ClientDeliverablesView() -> impl IntoView {
    let files_resource = Resource::new(|| (), |_| async move { get_client_deliverables().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Deliverables & Technical Documentation"</h2>
                <p class="text-xs text-slate-500">"Production architecture blueprints, test reports, deployed source artifacts, and release notes."</p>
            </div>

            <Card
                title="Deliverable Artifacts Repository".to_string()
                subtitle="All published engineering outputs for this tenant organization.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading deliverables..."</div> }>
                    {move || {
                        files_resource.get().map(|res| match res {
                            Ok(files) => {
                                if files.is_empty() {
                                    view! {
                                        <div class="py-8 text-center text-xs text-slate-500">
                                            "No deliverables published yet."
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="divide-y divide-slate-100">
                                            {files.into_iter().map(|f| view! {
                                                <div class="py-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                                                    <div class="flex items-center gap-3">
                                                        <div class="w-8 h-8 rounded bg-brand-50 text-brand-600 flex items-center justify-center">
                                                            <Icon name="fileText".to_string() size=16 />
                                                        </div>
                                                        <div>
                                                            <p class="text-xs font-bold text-slate-900">{f.title}</p>
                                                            <p class="text-[11px] text-slate-400 font-mono">
                                                                {f.file_name.clone()} " • " {f.file_size_formatted} " • Published " {f.published_at}
                                                            </p>
                                                        </div>
                                                    </div>
                                                    <div class="flex items-center gap-2">
                                                        <Badge variant="navy".to_string()>{f.version}</Badge>
                                                        <Badge variant="success".to_string() dot=true>{f.status}</Badge>
                                                        <a
                                                            href=format!("/api/deliverables/{}/download", f.id)
                                                            download=f.file_name.clone()
                                                            class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-brand-50 hover:bg-brand-100 text-brand-700 text-xs font-semibold transition-colors border border-brand-200 shadow-xs"
                                                        >
                                                            <Icon name="arrowDown".to_string() size=12 />
                                                            <span>"Secure Download"</span>
                                                        </a>
                                                    </div>
                                                </div>
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(e) => view! {
                                <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                            }.into_any(),
                        })
                    }}
                </Suspense>
            </Card>
        </div>
    }
}

// ─── 8. BILLING & INVOICES VIEW ─────────────────────────────────────────────

#[component]
fn ClientBillingView() -> impl IntoView {
    let invoices_resource = Resource::new(|| (), |_| async move { get_client_invoices().await });
    let filter_status = RwSignal::new("all".to_string());
    let copied_iban = RwSignal::new(false);

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Invoices & Financial Settlements"</h2>
                <p class="text-xs text-slate-500">"Official tax invoices, itemized delivery billing, and corporate settlement details."</p>
            </div>

            {/* Invoices List */}
            <Card
                title="Workspace Invoices Directory".to_string()
                subtitle="Itemized billing breakdown and verification history.".to_string()
            >
                <div class="mb-4 flex items-center gap-2 border-b border-slate-100 pb-3">
                    <span class="text-xs font-medium text-slate-500 mr-2">"Filter by Status:"</span>
                    <button
                        type="button"
                        class=move || format!("px-2.5 py-1 text-xs font-medium rounded-md transition-colors cursor-pointer {}",
                            if filter_status.get() == "all" { "bg-slate-900 text-white" } else { "bg-slate-100 text-slate-600 hover:bg-slate-200" }
                        )
                        on:click=move |_| filter_status.set("all".to_string())
                    >
                        "All Invoices"
                    </button>
                    <button
                        type="button"
                        class=move || format!("px-2.5 py-1 text-xs font-medium rounded-md transition-colors cursor-pointer {}",
                            if filter_status.get() == "paid" { "bg-emerald-700 text-white" } else { "bg-emerald-50 text-emerald-700 hover:bg-emerald-100" }
                        )
                        on:click=move |_| filter_status.set("paid".to_string())
                    >
                        "Paid"
                    </button>
                    <button
                        type="button"
                        class=move || format!("px-2.5 py-1 text-xs font-medium rounded-md transition-colors cursor-pointer {}",
                            if filter_status.get() == "pending" { "bg-amber-700 text-white" } else { "bg-amber-50 text-amber-700 hover:bg-amber-100" }
                        )
                        on:click=move |_| filter_status.set("pending".to_string())
                    >
                        "Pending / Due"
                    </button>
                </div>

                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading billing invoices..."</div> }>
                    {move || {
                        invoices_resource.get().map(|res| match res {
                            Ok(invoices) => {
                                let filter = filter_status.get();
                                let filtered: Vec<_> = invoices.into_iter().filter(|inv| {
                                    match filter.as_str() {
                                        "paid" => inv.status.to_lowercase() == "paid",
                                        "pending" => inv.status.to_lowercase() != "paid",
                                        _ => true,
                                    }
                                }).collect();

                                if filtered.is_empty() {
                                    view! {
                                        <div class="py-8 text-center text-xs text-slate-500">
                                            "No invoices found matching current filter."
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-6">
                                            {filtered.into_iter().map(|inv| {
                                                let is_paid = inv.status == "Paid";
                                                let curr = inv.currency.clone();
                                                view! {
                                                    <div class="p-4 border border-slate-200 rounded-xl space-y-3 bg-slate-50/50">
                                                        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-slate-200 pb-3">
                                                            <div>
                                                                <div class="flex items-center gap-2">
                                                                    <span class="text-sm font-bold font-mono text-slate-900">{inv.invoice_number}</span>
                                                                    <Badge variant=if is_paid { "success".to_string() } else { "danger".to_string() } dot=true>
                                                                        {inv.status}
                                                                    </Badge>
                                                                </div>
                                                                <p class="text-[11px] text-slate-500 font-mono mt-0.5">
                                                                    "Issued: " {inv.issued_at} " • Due: " {inv.due_date}
                                                                </p>
                                                            </div>
                                                            <div class="text-end">
                                                                <span class="text-base font-bold font-mono text-slate-900">
                                                                    {inv.total_amount} " " {curr.clone()}
                                                                </span>
                                                                <p class="text-[10px] text-slate-400 font-mono">"VAT Inclusive (15%)"</p>
                                                            </div>
                                                        </div>

                                                        {/* Line Items Table */}
                                                        <div class="space-y-1">
                                                            <p class="text-[10px] font-bold uppercase tracking-wider text-slate-400">"Itemized Scope"</p>
                                                            <div class="divide-y divide-slate-100 text-xs">
                                                                {
                                                                    let c = curr.clone();
                                                                    inv.items.into_iter().map(move |it| {
                                                                        let item_c = c.clone();
                                                                        view! {
                                                                            <div class="py-1.5 flex justify-between">
                                                                                <span class="text-slate-700">{it.description} " (x" {it.quantity} ")"</span>
                                                                                <span class="font-mono font-medium text-slate-900">{it.total_amount} " " {item_c}</span>
                                                                            </div>
                                                                        }
                                                                    }).collect_view()
                                                                }
                                                            </div>
                                                        </div>

                                                        {inv.notes.map(|n| view! {
                                                            <p class="text-[11px] text-slate-500 italic bg-white p-2 rounded border border-slate-100">
                                                                "Note: " {n}
                                                            </p>
                                                        })}
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(e) => view! {
                                <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                            }.into_any(),
                        })
                    }}
                </Suspense>
            </Card>

            {/* Corporate Wire Settlement Instructions */}
            <Card
                title="Corporate Payment & Wire Instructions".to_string()
                subtitle="Settlement details for bank wire transfer in SAR.".to_string()
            >
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
                    <div class="p-3.5 bg-slate-50 rounded-lg border border-slate-200 space-y-1.5">
                        <span class="text-slate-500 font-medium">"Beneficiary Name:"</span>
                        <p class="font-semibold text-slate-900">"TeebTech Information Technology LLC"</p>
                        <span class="text-slate-500 font-medium block pt-1">"Corporate Bank:"</span>
                        <p class="font-semibold text-slate-900">"Al Rajhi Bank / Saudi National Bank (SNB)"</p>
                    </div>
                    <div class="p-3.5 bg-slate-50 rounded-lg border border-slate-200 space-y-1.5">
                        <div class="flex items-center justify-between">
                            <span class="text-slate-500 font-medium">"Corporate IBAN (SAR):"</span>
                            <button
                                type="button"
                                class="text-[11px] font-semibold text-brand-600 hover:text-brand-800 transition-colors cursor-pointer"
                                on:click=move |_| copied_iban.set(true)
                            >
                                {move || if copied_iban.get() { "✓ Copied!" } else { "Copy IBAN" }}
                            </button>
                        </div>
                        <p class="font-mono font-bold text-slate-900 select-all">"SA4480000456608010099887"</p>
                        <span class="text-slate-500 font-medium block pt-1">"Payment Reference:"</span>
                        <p class="font-mono text-slate-700">"Include invoice number (e.g. INV-2026-001) in wire memo"</p>
                    </div>
                </div>
            </Card>
        </div>
    }
}

// ─── 9. ACTIVITY HISTORY VIEW ───────────────────────────────────────────────

#[component]
fn ClientHistoryView() -> impl IntoView {
    let logs_resource = Resource::new(|| (), |_| async move { get_client_audit_logs().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Audit Logs & Workspace Activity"</h2>
                <p class="text-xs text-slate-500">"Tamper-evident audit timeline of security events, status modifications, and access requests."</p>
            </div>

            <Card
                title="Activity Timeline".to_string()
                subtitle="All events recorded within your tenant boundary.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading audit stream..."</div> }>
                    {move || {
                        logs_resource.get().map(|res| match res {
                            Ok(logs) => {
                                if logs.is_empty() {
                                    view! {
                                        <div class="py-8 text-center text-xs text-slate-500">
                                            "No activity logged yet."
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="divide-y divide-slate-100">
                                            {logs.into_iter().map(|log| view! {
                                                <div class="py-3 flex items-start justify-between gap-3 text-xs">
                                                    <div class="space-y-0.5">
                                                        <div class="flex items-center gap-2">
                                                            <span class="font-bold text-slate-900">{log.action}</span>
                                                            <Badge variant="navy".to_string()>{log.resource_type}</Badge>
                                                        </div>
                                                        <p class="text-slate-500 text-[11px]">
                                                            "Actor: " <span class="font-semibold text-slate-700">{log.actor_name}</span>
                                                            " (" {log.actor_role} ") • IP: " <span class="font-mono">{log.ip_address}</span>
                                                        </p>
                                                        {log.details.map(|d| view! {
                                                            <p class="text-slate-600 font-mono text-[11px] bg-slate-50 p-1 rounded inline-block">{d}</p>
                                                        })}
                                                    </div>
                                                    <span class="text-[10px] text-slate-400 font-mono flex-shrink-0">{log.recorded_at}</span>
                                                </div>
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            Err(e) => view! {
                                <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                            }.into_any(),
                        })
                    }}
                </Suspense>
            </Card>
        </div>
    }
}

// ─── 10. SETTINGS & PROFILE VIEW ────────────────────────────────────────────

#[component]
fn ClientSettingsView() -> impl IntoView {
    let session = use_context::<ClientSessionDto>().expect("Session context required in ClientSettingsView");
    let on_logout = use_context::<Callback<()>>().unwrap_or_else(|| Callback::new(|()| ()));
    let is_logging_out = use_context::<RwSignal<bool>>().map(|s| s.get()).unwrap_or(false);
    let display_name_input = RwSignal::new(session.user.display_name.clone());
    let update_profile_action = Action::new(move |name: &String| {
        let name = name.clone();
        async move { update_client_profile(name).await }
    });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Account & Security Configuration"</h2>
                <p class="text-xs text-slate-500">"Manage authenticated identity, tenant workspace memberships, and zero-trust session posture."</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <Card
                    title="Authenticated Profile".to_string()
                    subtitle="Server-validated user credentials & display preferences.".to_string()
                >
                    <div class="space-y-4 text-xs">
                        <div class="flex items-center justify-between border-b border-slate-100 pb-2">
                            <span class="text-slate-500 font-medium">"Full Name"</span>
                            <span class="font-semibold text-slate-900">{session.user.display_name.clone()}</span>
                        </div>
                        <div class="flex items-center justify-between border-b border-slate-100 pb-2">
                            <span class="text-slate-500 font-medium">"Email Address"</span>
                            <span class="font-mono text-slate-800">{session.user.email}</span>
                        </div>
                        <div class="flex items-center justify-between border-b border-slate-100 pb-2">
                            <span class="text-slate-500 font-medium">"User ID"</span>
                            <span class="font-mono text-[10px] text-slate-400">{session.user.id}</span>
                        </div>
                        <div class="flex items-center justify-between border-b border-slate-100 pb-2">
                            <span class="text-slate-500 font-medium">"Account Status"</span>
                            <Badge variant="success".to_string() dot=true>"Active & Verified"</Badge>
                        </div>

                        {/* Interactive Profile Update Form */}
                        <div class="pt-2 border-t border-slate-100 space-y-2">
                            <label class="block font-semibold text-slate-700">"Update Workspace Display Name"</label>
                            <div class="flex gap-2">
                                <input
                                    type="text"
                                    class="flex-1 px-3 py-1.5 border border-slate-200 rounded-md text-xs focus:outline-none focus:ring-1 focus:ring-brand-500"
                                    prop:value=move || display_name_input.get()
                                    on:input=move |ev| display_name_input.set(event_target_value(&ev))
                                />
                                <button
                                    type="button"
                                    class="px-3 py-1.5 bg-brand-600 hover:bg-brand-700 text-white font-medium rounded-md text-xs transition-colors cursor-pointer disabled:opacity-50"
                                    disabled=move || update_profile_action.pending().get() || display_name_input.get().trim().is_empty()
                                    on:click=move |_| {
                                        update_profile_action.dispatch(display_name_input.get());
                                    }
                                >
                                    {move || if update_profile_action.pending().get() { "Saving..." } else { "Save" }}
                                </button>
                            </div>
                            {move || update_profile_action.value().get().map(|res| match res {
                                Ok(()) => view! {
                                    <p class="text-[11px] text-emerald-600 font-medium">"✓ Profile display name successfully updated in database."</p>
                                }.into_any(),
                                Err(e) => view! {
                                    <p class="text-[11px] text-red-600 font-medium">{format!("Error: {}", e)}</p>
                                }.into_any(),
                            })}
                        </div>
                    </div>
                </Card>

                <Card
                    title="Tenant Organization Membership".to_string()
                    subtitle="Authorized organizational boundary.".to_string()
                >
                    <div class="space-y-3 text-xs">
                        {session.memberships.into_iter().map(|m| view! {
                            <div class="p-3 bg-slate-50 rounded-lg border border-slate-200 space-y-1">
                                <div class="flex items-center justify-between">
                                    <span class="font-bold text-slate-900">{m.tenant_name}</span>
                                    <Badge variant="navy".to_string()>{m.role}</Badge>
                                </div>
                                <p class="text-[10px] text-slate-400 font-mono">"Tenant ID: " {m.tenant_id}</p>
                                <p class="text-[10px] text-slate-400 font-mono">"Slug: " {m.tenant_slug}</p>
                            </div>
                        }).collect_view()}
                    </div>
                </Card>
            </div>

            <Card
                title="Zero-Trust Session Hardening".to_string()
                subtitle="Cryptographic safeguards enforced on all client transactions.".to_string()
            >
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 text-xs">
                    <div class="p-3 rounded-lg bg-slate-50 border border-slate-200">
                        <span class="font-bold text-slate-900 block">"HttpOnly Cookie"</span>
                        <p class="text-slate-500 text-[11px] mt-0.5">"Zero JavaScript access to session token."</p>
                    </div>
                    <div class="p-3 rounded-lg bg-slate-50 border border-slate-200">
                        <span class="font-bold text-slate-900 block">"SameSite=Lax"</span>
                        <p class="text-slate-500 text-[11px] mt-0.5">"Full CSRF protection across requests."</p>
                    </div>
                    <div class="p-3 rounded-lg bg-slate-50 border border-slate-200">
                        <span class="font-bold text-slate-900 block">"Multi-Tenant Isolation"</span>
                        <p class="text-slate-500 text-[11px] mt-0.5">"Queries strictly bounded by verified tenant_id."</p>
                    </div>
                </div>

                <div class="pt-4 mt-4 border-t border-slate-100 flex items-center justify-between">
                    <p class="text-xs text-slate-500">"End session and clear cryptographic credentials."</p>
                    <div on:click=move |_| on_logout.run(())>
                        <Button
                            variant="danger".to_string()
                            size="sm".to_string()
                            loading=is_logging_out
                            class="cursor-pointer".to_string()
                        >
                            {if is_logging_out { "Signing Out..." } else { "Sign Out of Client Portal" }}
                        </Button>
                    </div>
                </div>
            </Card>
        </div>
    }
}

// ─── ACCESS DENIED & NOT FOUND VIEWS ────────────────────────────────────────

#[component]
fn AccessDeniedView() -> impl IntoView {
    let loc = use_location();
    let cur = move || loc.pathname.get();
    let login_url = move || {
        let p = cur();
        if p.is_empty() || p == "/" {
            "http://127.0.0.1:3000/login?return_to=http://127.0.0.1:3002".to_string()
        } else {
            format!("http://127.0.0.1:3000/login?return_to=http://127.0.0.1:3002{}", p)
        }
    };

    view! {
        <div class="min-h-screen bg-slate-50 flex items-center justify-center p-6">
            <div class="w-full max-w-md bg-white border border-slate-200 rounded-lg p-8 shadow-sm text-center space-y-4">
                <div class="w-12 h-12 rounded-full bg-red-50 border border-red-200 text-red-600 flex items-center justify-center mx-auto">
                    <Icon name="lock".to_string() size=24 />
                </div>
                <h1 class="font-display text-2xl font-semibold text-slate-900">"Access Denied"</h1>
                <p class="text-xs text-slate-500 leading-relaxed">
                    "The Client Portal requires an active authenticated session. No valid session cookie was detected on this request."
                </p>
                <div class="pt-2 flex flex-col sm:flex-row items-center justify-center gap-2">
                    <a
                        href=login_url
                        class="w-full sm:w-auto inline-flex items-center justify-center font-medium rounded-md text-xs px-4 py-2 bg-brand-600 text-white hover:bg-brand-700 transition-colors shadow-sm"
                    >
                        "Sign In on Home Portal"
                    </a>
                    <a
                        href="http://127.0.0.1:3000"
                        class="w-full sm:w-auto inline-flex items-center justify-center font-medium rounded-md text-xs px-4 py-2 bg-slate-100 text-slate-700 hover:bg-slate-200 transition-colors border border-slate-200"
                    >
                        "Back to Home"
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ClientNotFoundView() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-slate-50 p-6">
            <div class="max-w-md w-full bg-white border border-slate-200 rounded-xl p-8 shadow-sm text-center space-y-4">
                <div class="w-12 h-12 rounded-xl bg-brand-50 text-brand-600 flex items-center justify-center mx-auto">
                    <Icon name="alertCircle".to_string() size=24 />
                </div>
                <h1 class="text-xl font-bold text-slate-900">"Portal Page Not Found"</h1>
                <p class="text-xs text-slate-500 leading-relaxed">
                    "The client portal resource or path you requested does not exist or has been relocated."
                </p>
                <div class="pt-2">
                    <A
                        href="/"
                        {..}
                        class="inline-flex items-center justify-center px-4 py-2 rounded-lg bg-brand-600 text-white font-medium text-xs hover:bg-brand-700 transition-colors shadow-xs"
                    >
                        "Return to Workspace Overview"
                    </A>
                </div>
            </div>
        </div>
    }
}
