use application::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    hooks::use_location,
    StaticSegment,
};
use ui::{Badge, Card, Icon, PortalLayout, PortalNavItem, StatCard};

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

// ─── ADMIN SERVER FUNCTIONS ──────────────────────────────────────────────────

#[cfg(feature = "ssr")]
async fn resolve_admin_auth_context() -> Result<(String, String), ServerFnError> {
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

    // Zero-Trust: verify platform administrator identity
    let is_admin = session.user.email == "admin@teebtech.com";

    if !is_admin {
        return Err(ServerFnError::new("Forbidden: Administrative privilege required"));
    }

    Ok((session.user.id, session.user.display_name))
}

#[server(GetAdminSessionServerFn, "/api")]
pub async fn get_admin_session() -> Result<Option<ClientSessionDto>, ServerFnError> {
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

        // Zero-Trust: strictly restrict to platform administrator
        if let Some(ref s) = res {
            if s.user.email != "admin@teebtech.com" {
                return Ok(None);
            }
        }

        Ok(res)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminOverviewServerFn, "/api")]
pub async fn get_admin_overview() -> Result<AdminOverviewDto, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_overview()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminClientsServerFn, "/api")]
pub async fn get_admin_clients() -> Result<Vec<AdminClientSummaryDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_clients()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminCatalogServerFn, "/api")]
pub async fn get_admin_catalog() -> Result<Vec<CatalogServiceDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
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

#[server(GetAdminServicesServerFn, "/api")]
pub async fn get_admin_services() -> Result<Vec<ClientServiceItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_all_services()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminProjectsServerFn, "/api")]
pub async fn get_admin_projects() -> Result<Vec<ProjectItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_all_projects()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminRequestsServerFn, "/api")]
pub async fn get_admin_requests() -> Result<Vec<ChangeRequestItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_all_change_requests()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AdminUpdateRequestServerFn, "/api")]
pub async fn admin_update_request_status(
    request_id: String,
    status: String,
    notes: Option<String>,
    cost: Option<String>,
    days: Option<i32>,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (admin_id, admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .update_change_request_status(
                &request_id,
                &status,
                notes.as_deref(),
                &admin_id,
                &admin_name,
            )
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminConversationsServerFn, "/api")]
pub async fn get_admin_conversations() -> Result<Vec<ConversationItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_all_conversations()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AdminSendMessageServerFn, "/api")]
pub async fn admin_send_message(
    conversation_id: String,
    body: String,
    is_internal: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (admin_id, admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        let role = if is_internal { "Staff Internal" } else { "Super Admin" };
        pool.portal_repository()
            .send_message(&conversation_id, &admin_id, &admin_name, role, &body, is_internal)
            .await
            .map(|_| ())
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminDeliverablesServerFn, "/api")]
pub async fn get_admin_deliverables() -> Result<Vec<DeliverableItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_all_deliverables()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminInvoicesServerFn, "/api")]
pub async fn get_admin_invoices() -> Result<Vec<InvoiceItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_all_invoices()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AdminMarkInvoicePaidServerFn, "/api")]
pub async fn admin_mark_invoice_paid(
    invoice_id: String,
    payment_method: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (admin_id, admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .admin_mark_invoice_paid(&invoice_id, &payment_method, &admin_id, &admin_name)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAdminAuditLogsServerFn, "/api")]
pub async fn get_admin_audit_logs() -> Result<Vec<AuditLogItemDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let (_admin_id, _admin_name) = resolve_admin_auth_context().await?;
        let config = config::ServerAppConfig::from_env();
        let pool = database::DatabasePool::connect(&config.database_url)
            .await
            .unwrap_or_else(|_| database::DatabasePool::new(&config.database_url));
        pool.portal_repository()
            .get_admin_audit_logs()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AdminLogoutServerFn, "/api")]
pub async fn admin_logout() -> Result<(), ServerFnError> {
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

// ─── TOP-LEVEL APP & ROUTER ──────────────────────────────────────────────────

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/admin.css"/>
        <Title text="TeebTech — Centralized Admin Management Platform"/>

        <Router>
            <Routes fallback=AdminNotFoundView>
                <ParentRoute path=StaticSegment("") view=AdminPortalShell>
                    <Route path=StaticSegment("") view=AdminOverviewView/>
                    <Route path=StaticSegment("overview") view=AdminOverviewView/>
                    <Route path=StaticSegment("clients") view=AdminClientsView/>
                    <Route path=StaticSegment("catalog") view=AdminCatalogView/>
                    <Route path=StaticSegment("services") view=AdminServicesView/>
                    <Route path=StaticSegment("projects") view=AdminProjectsView/>
                    <Route path=StaticSegment("requests") view=AdminRequestsView/>
                    <Route path=StaticSegment("messages") view=AdminMessagesView/>
                    <Route path=StaticSegment("deliverables") view=AdminDeliverablesView/>
                    <Route path=StaticSegment("billing") view=AdminBillingView/>
                    <Route path=StaticSegment("invoices") view=AdminBillingView/>
                    <Route path=StaticSegment("audit") view=AdminAuditLogsView/>
                    <Route path=StaticSegment("history") view=AdminAuditLogsView/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn AdminPortalShell() -> impl IntoView {
    let session_resource = Resource::new(|| (), |_| async move { get_admin_session().await });
    let logout_action = Action::new(move |_: &()| async move { admin_logout().await });

    Effect::new(move |_| {
        if let Some(res) = logout_action.value().get() {
            if res.is_ok() {
                session_resource.refetch();
            }
        }
    });

    view! {
        <Suspense fallback=|| view! {
            <div class="min-h-screen flex items-center justify-center bg-navy-950 text-white">
                <div class="p-6 bg-navy-900 border border-navy-800 rounded-lg shadow-sm text-center">
                    <p class="text-xs text-slate-400 font-mono">"Verifying platform administrative boundaries..."</p>
                </div>
            </div>
        }>
            {move || {
                session_resource.get().map(|res| match res {
                    Ok(Some(session)) => {
                        let is_logging_out = logout_action.pending().get();
                        view! {
                            <AuthenticatedAdminPortal
                                session=session
                                is_logging_out=is_logging_out
                                on_logout=Callback::new(move |()| {
                                    logout_action.dispatch(());
                                })
                            />
                        }.into_any()
                    }
                    _ => view! {
                        <AdminAccessDeniedView/>
                    }.into_any(),
                })
            }}
        </Suspense>
    }
}

// ─── AUTHENTICATED ADMIN PORTAL ──────────────────────────────────────────────

#[component]
fn AuthenticatedAdminPortal(
    session: ClientSessionDto,
    #[allow(unused_variables)] is_logging_out: bool,
    on_logout: Callback<()>,
) -> impl IntoView {
    provide_context(session.clone());
    provide_context(on_logout);

    let u_name = session.user.display_name.clone();
    let u_email = session.user.email.clone();

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
                label: "Admin Overview".to_string(),
                label_ar: "لوحة التحكم الرئيسية".to_string(),
                href: "/".to_string(),
                icon: "home".to_string(),
                active: is_active("/", true),
                badge: None,
            },
            PortalNavItem {
                label: "Client Accounts".to_string(),
                label_ar: "إدارة العملاء".to_string(),
                href: "/clients".to_string(),
                icon: "users".to_string(),
                active: is_active("/clients", false),
                badge: None,
            },
            PortalNavItem {
                label: "Service Catalog".to_string(),
                label_ar: "كتالوج الخدمات".to_string(),
                href: "/catalog".to_string(),
                icon: "globe".to_string(),
                active: is_active("/catalog", false),
                badge: None,
            },
            PortalNavItem {
                label: "Active Services".to_string(),
                label_ar: "الخدمات النشطة".to_string(),
                href: "/services".to_string(),
                icon: "layers".to_string(),
                active: is_active("/services", false),
                badge: None,
            },
            PortalNavItem {
                label: "Projects Directory".to_string(),
                label_ar: "إدارة المشاريع".to_string(),
                href: "/projects".to_string(),
                icon: "briefcase".to_string(),
                active: is_active("/projects", false),
                badge: None,
            },
            PortalNavItem {
                label: "Request Triage".to_string(),
                label_ar: "طلبات التعديل والتقييم".to_string(),
                href: "/requests".to_string(),
                icon: "refresh".to_string(),
                active: is_active("/requests", false),
                badge: Some("Review".to_string()),
            },
            PortalNavItem {
                label: "Communications".to_string(),
                label_ar: "قنوات التواصل والدعم".to_string(),
                href: "/messages".to_string(),
                icon: "messageSquare".to_string(),
                active: is_active("/messages", false),
                badge: None,
            },
            PortalNavItem {
                label: "Deliverables".to_string(),
                label_ar: "المخرجات والملفات".to_string(),
                href: "/deliverables".to_string(),
                icon: "fileText".to_string(),
                active: is_active("/deliverables", false),
                badge: None,
            },
            PortalNavItem {
                label: "Platform Billing".to_string(),
                label_ar: "الفواتير والتحصيلات".to_string(),
                href: "/billing".to_string(),
                icon: "chart".to_string(),
                active: is_active("/billing", false) || is_active("/invoices", false),
                badge: None,
            },
            PortalNavItem {
                label: "Security Audit Logs".to_string(),
                label_ar: "سجل العمليات والأمان".to_string(),
                href: "/audit".to_string(),
                icon: "shield".to_string(),
                active: is_active("/audit", false) || is_active("/history", false),
                badge: None,
            },
        ]
    };

    view! {
        <PortalLayout
            portal_name="Admin Management Portal".to_string()
            user_name=u_name.clone()
            user_email=u_email.clone()
            user_role="Super Admin (Platform Operations)".to_string()
            is_admin=true
            nav_items=nav_items()
            on_logout=on_logout
        >
            {/* Quick Interactive Nav Tab Bar */}
            <div class="flex items-center gap-1.5 overflow-x-auto pb-2 border-b border-slate-200 text-xs">
                {vec![
                    ("/", "Overview", "home"),
                    ("/clients", "Clients", "users"),
                    ("/catalog", "Catalog", "globe"),
                    ("/services", "Subscriptions", "layers"),
                    ("/projects", "Projects", "briefcase"),
                    ("/requests", "Change Requests", "refresh"),
                    ("/messages", "Communications", "messageSquare"),
                    ("/deliverables", "Deliverables", "fileText"),
                    ("/billing", "Billing", "chart"),
                    ("/audit", "Security Logs", "shield"),
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
                                    "bg-navy-900 text-white shadow-xs"
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

            {/* Dynamic Admin View Body via Nested Route Outlet */}
            <div class="mt-4">
                <Outlet/>
            </div>
        </PortalLayout>
    }
}

// ─── 1. ADMIN OVERVIEW VIEW ──────────────────────────────────────────────────

#[component]
fn AdminOverviewView() -> impl IntoView {
    let overview_resource = Resource::new(|| (), |_| async move { get_admin_overview().await });

    view! {
        <Suspense fallback=|| view! { <div class="py-12 text-center text-slate-400 font-mono text-xs">"Aggregating platform metrics..."</div> }>
            {move || {
                overview_resource.get().map(|res| match res {
                    Ok(data) => {
                        view! {
                            <div class="space-y-6">
                                {/* Admin KPI Stat Cards */}
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-4">
                                    <StatCard
                                        label="Registered Clients".to_string()
                                        value=data.total_clients.to_string()
                                        delta="Active Tenants".to_string()
                                        delta_up=true
                                        icon="users".to_string()
                                    />
                                    <StatCard
                                        label="Active Service Contracts".to_string()
                                        value=data.active_services.to_string()
                                        delta="Enterprise SLA".to_string()
                                        delta_up=true
                                        icon="layers".to_string()
                                    />
                                    <StatCard
                                        label="Active Projects".to_string()
                                        value=data.active_projects.to_string()
                                        delta="In Progress".to_string()
                                        delta_up=true
                                        icon="briefcase".to_string()
                                    />
                                    <StatCard
                                        label="Pending Change Requests".to_string()
                                        value=data.pending_requests.to_string()
                                        delta="Action Required".to_string()
                                        delta_up=false
                                        icon="refresh".to_string()
                                    />
                                    <StatCard
                                        label="Total Platform Revenue".to_string()
                                        value=format!("{} SAR", data.total_revenue_sar)
                                        delta="Invoiced & Settled".to_string()
                                        delta_up=true
                                        icon="chart".to_string()
                                    />
                                </div>

                                {/* Platform Telemetry & Quick Action Bar */}
                                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                                    {/* Client Workspaces Summary */}
                                    <div class="lg:col-span-2">
                                        <Card
                                            title="Tenant Client Directory".to_string()
                                            subtitle="Active organizations with service subscriptions and delivery projects.".to_string()
                                        >
                                            <div class="divide-y divide-slate-100">
                                                {data.clients.into_iter().map(|c| view! {
                                                    <div class="py-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                                                        <div>
                                                            <div class="flex items-center gap-2">
                                                                <span class="text-xs font-bold text-slate-900">{c.name}</span>
                                                                <span class="text-[10px] text-slate-400 font-mono">"slug: " {c.slug}</span>
                                                            </div>
                                                            <p class="text-[11px] text-slate-500 mt-0.5">
                                                                "Services: " <strong class="text-slate-800">{c.active_services_count}</strong>
                                                                " • Projects: " <strong class="text-slate-800">{c.active_projects_count}</strong>
                                                                " • Registered: " <span class="font-mono">{c.created_at}</span>
                                                            </p>
                                                        </div>
                                                        <div class="flex items-center gap-3">
                                                            <span class="text-xs font-mono font-bold text-slate-900">{c.total_billed_sar} " SAR"</span>
                                                            <Badge variant="success".to_string() dot=true>"Active"</Badge>
                                                        </div>
                                                    </div>
                                                }).collect_view()}
                                            </div>
                                            <div class="pt-3 border-t border-slate-100">
                                                <a
                                                    href="/clients"
                                                    class="text-xs text-navy-800 hover:text-navy-950 font-semibold inline-flex items-center gap-1 text-decoration-none"
                                                >
                                                    <span>"Manage all client organizations & permissions"</span>
                                                    <Icon name="arrowRight".to_string() size=12 />
                                                </a>
                                            </div>
                                        </Card>
                                    </div>

                                    {/* System Health Status */}
                                    <Card
                                        title="System Infrastructure Health".to_string()
                                        subtitle="Operational status across all services.".to_string()
                                    >
                                        <div class="space-y-3 text-xs">
                                            <div class="p-2.5 rounded bg-slate-50 border border-slate-200 flex items-center justify-between">
                                                <div class="flex items-center gap-2">
                                                    <Icon name="database".to_string() size=14 class="text-brand-600".to_string() />
                                                    <span class="font-semibold text-slate-800">"PostgreSQL 16 Multi-Tenant DB"</span>
                                                </div>
                                                <Badge variant="success".to_string() dot=true>"Port 5433"</Badge>
                                            </div>
                                            <div class="p-2.5 rounded bg-slate-50 border border-slate-200 flex items-center justify-between">
                                                <div class="flex items-center gap-2">
                                                    <Icon name="shield".to_string() size=14 class="text-brand-600".to_string() />
                                                    <span class="font-semibold text-slate-800">"Zero-Trust Policy Engine"</span>
                                                </div>
                                                <Badge variant="navy".to_string()>"Enforced"</Badge>
                                            </div>
                                            <div class="p-2.5 rounded bg-slate-50 border border-slate-200 flex items-center justify-between">
                                                <div class="flex items-center gap-2">
                                                    <Icon name="lock".to_string() size=14 class="text-brand-600".to_string() />
                                                    <span class="font-semibold text-slate-800">"HttpOnly Session Boundary"</span>
                                                </div>
                                                <Badge variant="cyan".to_string() dot=true>"Active"</Badge>
                                            </div>
                                        </div>
                                    </Card>
                                </div>

                                {/* Platform Audit Trail */}
                                <Card
                                    title="Recent Operational Activity Stream".to_string()
                                    subtitle="Live tamper-evident administrative events.".to_string()
                                >
                                    <div class="divide-y divide-slate-100">
                                        {data.recent_activity.into_iter().map(|a| view! {
                                            <div class="py-2.5 flex items-start justify-between gap-3 text-xs">
                                                <div class="space-y-0.5">
                                                    <div class="flex items-center gap-2">
                                                        <span class="font-bold text-slate-900">{a.action}</span>
                                                        <Badge variant="navy".to_string()>{a.resource_type}</Badge>
                                                    </div>
                                                    <p class="text-slate-500 text-[11px]">
                                                        "Actor: " <span class="font-semibold text-slate-700">{a.actor_name}</span>
                                                        " (" {a.actor_role} ") • IP: " <span class="font-mono">{a.ip_address}</span>
                                                    </p>
                                                    {a.details.map(|d| view! {
                                                        <p class="text-slate-600 font-mono text-[11px] bg-slate-50 p-1 rounded inline-block">{d}</p>
                                                    })}
                                                </div>
                                                <span class="text-[10px] text-slate-400 font-mono flex-shrink-0">{a.recorded_at}</span>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                </Card>
                            </div>
                        }.into_any()
                    }
                    Err(e) => view! {
                        <div class="p-4 bg-red-50 text-red-700 text-xs rounded-md">{e.to_string()}</div>
                    }.into_any(),
                })
            }}
        </Suspense>
    }
}

// ─── 2. CLIENT MANAGEMENT VIEW ──────────────────────────────────────────────

#[component]
fn AdminClientsView() -> impl IntoView {
    let clients_resource = Resource::new(|| (), |_| async move { get_admin_clients().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Client Account Management"</h2>
                <p class="text-xs text-slate-500">"Directory of all tenant organizations, active contracts, and lifetime billed volume."</p>
            </div>

            <Card
                title="Client Accounts Registry".to_string()
                subtitle="Manage tenant organizations, memberships, and SLA configurations.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading clients..."</div> }>
                    {move || {
                        clients_resource.get().map(|res| match res {
                            Ok(clients) => {
                                view! {
                                    <div class="divide-y divide-slate-100">
                                        {clients.into_iter().map(|c| view! {
                                            <div class="py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
                                                <div class="flex items-center gap-3">
                                                    <div class="w-10 h-10 rounded-lg bg-navy-50 text-navy-800 flex items-center justify-center font-bold text-sm">
                                                        <Icon name="building".to_string() size=20 />
                                                    </div>
                                                    <div>
                                                        <h3 class="text-sm font-bold text-slate-900">{c.name}</h3>
                                                        <p class="text-xs text-slate-400 font-mono">"ID: " {c.tenant_id} " • slug: " {c.slug}</p>
                                                        <p class="text-[11px] text-slate-500 mt-0.5">
                                                            "Registered: " <span class="font-mono">{c.created_at}</span>
                                                        </p>
                                                    </div>
                                                </div>

                                                <div class="flex flex-wrap items-center gap-4 text-xs">
                                                    <div class="p-2 bg-slate-50 rounded border border-slate-100 text-center">
                                                        <span class="text-slate-400 block text-[10px]">"Active Services"</span>
                                                        <span class="font-bold text-slate-800">{c.active_services_count}</span>
                                                    </div>
                                                    <div class="p-2 bg-slate-50 rounded border border-slate-100 text-center">
                                                        <span class="text-slate-400 block text-[10px]">"Active Projects"</span>
                                                        <span class="font-bold text-slate-800">{c.active_projects_count}</span>
                                                    </div>
                                                    <div class="p-2 bg-slate-50 rounded border border-slate-100 text-center">
                                                        <span class="text-slate-400 block text-[10px]">"Total Billed"</span>
                                                        <span class="font-bold font-mono text-emerald-700">{c.total_billed_sar} " SAR"</span>
                                                    </div>
                                                    <Badge variant="success".to_string() dot=true>"Compliant"</Badge>
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
            </Card>
        </div>
    }
}

// ─── 3. CATALOG VIEW ────────────────────────────────────────────────────────

#[component]
fn AdminCatalogView() -> impl IntoView {
    let catalog_resource = Resource::new(|| (), |_| async move { get_admin_catalog().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Service Catalog & Pricing Tier Management"</h2>
                <p class="text-xs text-slate-500">"Configure service offerings, deliverables scope, and contract pricing packages."</p>
            </div>

            <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading catalog offerings..."</div> }>
                {move || {
                    catalog_resource.get().map(|res| match res {
                        Ok(catalog) => {
                            view! {
                                <div class="space-y-6">
                                    {catalog.into_iter().map(|srv| view! {
                                        <div class="bg-white border border-slate-200 rounded-xl p-5 shadow-xs space-y-4">
                                            <div class="flex items-start justify-between gap-3">
                                                <div>
                                                    <span class="text-[10px] font-mono uppercase tracking-wider text-navy-800 font-semibold">{srv.category}</span>
                                                    <h3 class="text-base font-bold text-slate-900 mt-0.5">{srv.title}</h3>
                                                    <p class="text-xs text-slate-500 font-arabic">{srv.title_ar}</p>
                                                    <p class="text-xs text-slate-600 mt-1 max-w-2xl">{srv.description}</p>
                                                </div>
                                                <Badge variant="navy".to_string()>"Published"</Badge>
                                            </div>

                                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 pt-2">
                                                {srv.packages.into_iter().map(|pkg| view! {
                                                    <div class="p-3.5 bg-slate-50 border border-slate-200 rounded-lg space-y-2">
                                                        <div class="flex justify-between items-start">
                                                            <h4 class="text-xs font-bold text-slate-900">{pkg.name}</h4>
                                                            <span class="text-xs font-mono font-bold text-navy-900">{pkg.price} " " {pkg.currency}</span>
                                                        </div>
                                                        <p class="text-[10px] text-slate-400 font-mono">"Billing: " {pkg.billing_period}</p>
                                                        <ul class="text-[11px] text-slate-600 space-y-1 pt-1 border-t border-slate-200">
                                                            {pkg.features.into_iter().map(|f| view! {
                                                                <li class="flex items-center gap-1.5">
                                                                    <Icon name="check".to_string() size=10 class="text-emerald-600 flex-shrink-0".to_string() />
                                                                    <span>{f}</span>
                                                                </li>
                                                            }).collect_view()}
                                                        </ul>
                                                    </div>
                                                }).collect_view()}
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

// ─── 4. ACTIVE SERVICES (SUBSCRIPTIONS) VIEW ─────────────────────────────────

#[component]
fn AdminServicesView() -> impl IntoView {
    let services_resource = Resource::new(|| (), |_| async move { get_admin_services().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Provisioned Services Directory"</h2>
                <p class="text-xs text-slate-500">"All active client service contracts, assigned architects, and SLA policies across all tenants."</p>
            </div>

            <Card
                title="Active Contracts Registry".to_string()
                subtitle="Service delivery oversight and lead engineer assignments.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading services..."</div> }>
                    {move || {
                        services_resource.get().map(|res| match res {
                            Ok(services) => {
                                view! {
                                    <div class="divide-y divide-slate-100">
                                        {services.into_iter().map(|s| view! {
                                            <div class="py-4 flex flex-col md:flex-row md:items-center justify-between gap-3">
                                                <div>
                                                    <div class="flex items-center gap-2">
                                                        <h3 class="text-xs font-bold text-slate-900">{s.title}</h3>
                                                        <Badge variant="cyan".to_string()>{s.sla_tier}</Badge>
                                                        <Badge variant="success".to_string() dot=true>{s.status}</Badge>
                                                    </div>
                                                    <p class="text-[11px] text-slate-500 mt-0.5">
                                                        {s.package_name} " • Assigned Architect: " <span class="font-semibold text-slate-800">{s.assigned_lead}</span>
                                                    </p>
                                                    <p class="text-[10px] text-slate-400 font-mono">
                                                        "Started: " {s.start_date} {s.renewal_date.map(|r| format!(" • Renewal: {}", r)).unwrap_or_default()}
                                                    </p>
                                                </div>
                                                <div class="text-end">
                                                    <span class="text-xs font-mono font-bold text-slate-900">{s.price} " " {s.currency}</span>
                                                    <span class="text-[10px] text-slate-400 block font-mono">"Per billing term"</span>
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
            </Card>
        </div>
    }
}

// ─── 5. PROJECTS MANAGEMENT VIEW ────────────────────────────────────────────

#[component]
fn AdminProjectsView() -> impl IntoView {
    let projects_resource = Resource::new(|| (), |_| async move { get_admin_projects().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Platform-Wide Projects & Roadmaps"</h2>
                <p class="text-xs text-slate-500">"Monitor development sprints, squad velocity, and milestone completion across all clients."</p>
            </div>

            <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading platform projects..."</div> }>
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
                                                        <h3 class="text-sm font-bold text-slate-900">{p.title}</h3>
                                                        <Badge variant="navy".to_string()>{p.priority}</Badge>
                                                        <Badge variant="success".to_string() dot=true>{p.status}</Badge>
                                                    </div>
                                                    <p class="text-xs text-slate-500 mt-1 max-w-2xl">{p.description}</p>
                                                </div>
                                                <div class="text-end">
                                                    <span class="text-xs font-mono font-semibold text-slate-800">"Squad: " {p.assigned_team}</span>
                                                    {p.deadline.map(|d| view! {
                                                        <p class="text-[11px] font-mono text-slate-400 mt-0.5">"Deadline: " {d}</p>
                                                    })}
                                                </div>
                                            </div>

                                            <div class="space-y-1">
                                                <div class="flex justify-between text-xs font-semibold text-slate-700">
                                                    <span>"Progress"</span>
                                                    <span class="font-mono text-brand-600">{p.progress_pct} "% Completed"</span>
                                                </div>
                                                <div class="w-full bg-slate-100 rounded-full h-2 overflow-hidden">
                                                    <div class="bg-navy-800 h-2 rounded-full" style=format!("width: {}%", p.progress_pct)></div>
                                                </div>
                                            </div>

                                            <div class="pt-2 border-t border-slate-100">
                                                <p class="text-[10px] font-bold uppercase tracking-wider text-slate-400 mb-2">"Milestone Verification"</p>
                                                <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                                                    {p.milestones.into_iter().map(|m| {
                                                        let is_done = m.status == "Completed";
                                                        view! {
                                                            <div class="p-2.5 rounded bg-slate-50 border border-slate-200 text-xs flex items-center justify-between">
                                                                <span class="font-medium text-slate-800 truncate">{m.title}</span>
                                                                <Badge variant=if is_done { "success".to_string() } else { "cyan".to_string() } dot=true>
                                                                    {m.status}
                                                                </Badge>
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

// ─── 6. CHANGE REQUESTS TRIAGE VIEW ─────────────────────────────────────────

#[component]
fn AdminRequestsView() -> impl IntoView {
    let requests_resource = Resource::new(|| (), |_| async move { get_admin_requests().await });
    let action_msg = RwSignal::new(Option::<String>::None);

    let update_action = Action::new(move |(id, status, notes, cost, days): &(String, String, Option<String>, Option<String>, Option<i32>)| {
        let i = id.clone();
        let s = status.clone();
        let n = notes.clone();
        let c = cost.clone();
        let d = *days;
        async move {
            admin_update_request_status(i, s, n, c, d).await
        }
    });

    Effect::new(move |_| {
        if let Some(res) = update_action.value().get() {
            match res {
                Ok(_) => {
                    action_msg.set(Some("Request status updated successfully.".to_string()));
                    requests_resource.refetch();
                }
                Err(e) => {
                    action_msg.set(Some(format!("Error: {}", e)));
                }
            }
        }
    });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Scope Change Request Triage"</h2>
                <p class="text-xs text-slate-500">"Review, estimate, approve, or reject scope additions submitted by client organizations."</p>
            </div>

            {move || action_msg.get().map(|msg| view! {
                <div class="p-3 bg-navy-50 border border-navy-200 text-navy-900 text-xs rounded-md">
                    {msg}
                </div>
            })}

            <Card
                title="Pending & Evaluated Change Requests".to_string()
                subtitle="Direct governance actions with recorded audit trail.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading triage queue..."</div> }>
                    {move || {
                        requests_resource.get().map(|res| match res {
                            Ok(requests) => {
                                if requests.is_empty() {
                                    view! {
                                        <div class="py-8 text-center text-xs text-slate-500">
                                            "No change requests in queue."
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="divide-y divide-slate-100">
                                            {requests.into_iter().map(|cr| {
                                                let id_app = cr.id.clone();
                                                let id_rej = cr.id.clone();
                                                view! {
                                                    <div class="py-4 space-y-3">
                                                        <div class="flex items-start justify-between gap-3">
                                                            <div>
                                                                <h4 class="text-xs font-bold text-slate-900">{cr.title}</h4>
                                                                <p class="text-[11px] text-slate-500">
                                                                    "Requester: " <span class="font-semibold text-slate-700">{cr.requested_by_name}</span>
                                                                    " • Submitted: " <span class="font-mono">{cr.created_at}</span>
                                                                </p>
                                                            </div>
                                                            <Badge variant=match cr.status.as_str() {
                                                                "Approved" => "success",
                                                                "Rejected" => "danger",
                                                                _ => "navy",
                                                            }.to_string() dot=true>{cr.status}</Badge>
                                                        </div>

                                                        <p class="text-xs text-slate-700 bg-slate-50 p-3 rounded border border-slate-100 leading-relaxed">
                                                            {cr.scope_description}
                                                        </p>

                                                        <div class="flex flex-wrap items-center justify-between gap-3 text-xs">
                                                            <div class="flex items-center gap-4 font-mono text-slate-500">
                                                                <span>"Quote: " <strong class="text-slate-900">{cr.estimated_cost} " " {cr.currency}</strong></span>
                                                                <span>"Delivery: " <strong class="text-slate-900">{cr.estimated_days} " days"</strong></span>
                                                            </div>
                                                            <div class="flex items-center gap-2">
                                                                <button
                                                                    type="button"
                                                                    on:click=move |_| {
                                                                        update_action.dispatch((
                                                                            id_app.clone(),
                                                                            "Approved".to_string(),
                                                                            Some("Approved by Architecture Board with standard SLA.".to_string()),
                                                                            None,
                                                                            None,
                                                                        ));
                                                                    }
                                                                    class="px-3 py-1.5 rounded bg-emerald-600 hover:bg-emerald-700 text-white font-medium text-xs transition-colors cursor-pointer"
                                                                >
                                                                    "Approve Request"
                                                                </button>
                                                                <button
                                                                    type="button"
                                                                    on:click=move |_| {
                                                                        update_action.dispatch((
                                                                            id_rej.clone(),
                                                                            "Rejected".to_string(),
                                                                            Some("Scope conflicts with current sprint architecture freeze.".to_string()),
                                                                            None,
                                                                            None,
                                                                        ));
                                                                    }
                                                                    class="px-3 py-1.5 rounded bg-rose-600 hover:bg-rose-700 text-white font-medium text-xs transition-colors cursor-pointer"
                                                                >
                                                                    "Reject"
                                                                </button>
                                                            </div>
                                                        </div>
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

// ─── 7. COMMUNICATIONS (WITH INTERNAL STAFF NOTES) ───────────────────────────

#[component]
fn AdminMessagesView() -> impl IntoView {
    let conv_resource = Resource::new(|| (), |_| async move { get_admin_conversations().await });
    let message_input = RwSignal::new(String::new());
    let is_internal_note = RwSignal::new(false);
    let active_conv_id = RwSignal::new(Option::<String>::None);
    let is_sending = RwSignal::new(false);

    let send_action = Action::new(move |(cid, body, internal): &(String, String, bool)| {
        let c = cid.clone();
        let b = body.clone();
        let i = *internal;
        async move {
            admin_send_message(c, b, i).await
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

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Unified Communications & Staff Support Triage"</h2>
                <p class="text-xs text-slate-500">
                    "Zero-trust message center. Staff internal notes are strictly isolated from client-facing streams."
                </p>
            </div>

            <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading channels..."</div> }>
                {move || {
                    conv_resource.get().map(|res| match res {
                        Ok(convs) => {
                            if convs.is_empty() {
                                view! {
                                    <div class="p-8 text-center bg-white border border-slate-200 rounded-lg">
                                        <p class="text-xs text-slate-500">"No conversation channels found."</p>
                                    </div>
                                }.into_any()
                            } else {
                                let selected = {
                                    let cid = active_conv_id.get();
                                    convs.iter().find(|c| Some(c.id.clone()) == cid).cloned()
                                        .unwrap_or_else(|| convs.first().unwrap().clone())
                                };
                                let cur_cid = selected.id.clone();

                                view! {
                                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 bg-white border border-slate-200 rounded-xl overflow-hidden shadow-xs min-h-[520px]">
                                        {/* Channel Selector */}
                                        <div class="border-b md:border-b-0 md:border-r border-slate-200 p-4 space-y-2 bg-slate-50/50">
                                            <p class="text-[10px] font-bold uppercase tracking-wider text-slate-400 mb-2">
                                                "Client Channels"
                                            </p>
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
                                                                if is_sel { "bg-navy-900 text-white font-semibold shadow-xs" } else { "hover:bg-slate-100 text-slate-700" }
                                                            )
                                                        >
                                                            <p class="truncate">{c.title}</p>
                                                            <span class="text-[10px] opacity-75 font-mono block mt-0.5">{c.channel_type}</span>
                                                        </button>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>

                                        {/* Thread & Composer */}
                                        <div class="md:col-span-2 flex flex-col justify-between p-4 sm:p-6 space-y-4">
                                            <div class="border-b border-slate-100 pb-3 flex items-center justify-between">
                                                <div>
                                                    <h3 class="text-xs font-bold text-slate-900">{selected.title}</h3>
                                                    <span class="text-[10px] font-mono text-slate-400">{selected.channel_type}</span>
                                                </div>
                                                <Badge variant="navy".to_string() dot=true>"Staff Supervised"</Badge>
                                            </div>

                                            {/* Messages */}
                                            <div class="flex-1 overflow-y-auto space-y-3 pr-2 max-h-[360px]">
                                                {selected.messages.into_iter().map(|m| {
                                                    if m.is_internal {
                                                        view! {
                                                            <div class="p-3 rounded-lg bg-amber-50 border border-amber-200 text-xs space-y-1 text-amber-950">
                                                                <div class="flex items-center justify-between text-[10px]">
                                                                    <div class="flex items-center gap-1.5 font-bold">
                                                                        <Icon name="lock".to_string() size=10 class="text-amber-700".to_string() />
                                                                        <span>"STAFF INTERNAL NOTE (Hidden from Client)"</span>
                                                                    </div>
                                                                    <span class="font-mono">{m.sent_at}</span>
                                                                </div>
                                                                <p class="leading-relaxed">{m.body}</p>
                                                                <span class="text-[10px] text-amber-700 block font-medium">"By: " {m.sender_name}</span>
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        let is_admin_msg = m.sender_role.contains("Admin") || m.sender_role.contains("Staff");
                                                        view! {
                                                            <div class=format!(
                                                                "p-3 rounded-lg text-xs max-w-lg space-y-1 {}",
                                                                if is_admin_msg {
                                                                    "ms-auto bg-navy-900 text-white shadow-xs"
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
                                                        }.into_any()
                                                    }
                                                }).collect_view()}
                                            </div>

                                            {/* Composer with Internal Note Checkbox */}
                                            <div class="pt-3 border-t border-slate-100 space-y-3">
                                                <div class="flex items-center gap-2">
                                                    <input
                                                        type="checkbox"
                                                        id="internal_note_cb"
                                                        prop:checked=move || is_internal_note.get()
                                                        on:change=move |e| is_internal_note.set(event_target_checked(&e))
                                                        class="rounded border-slate-300 text-amber-600 focus:ring-amber-500"
                                                    />
                                                    <label for="internal_note_cb" class="text-xs font-semibold text-amber-800 flex items-center gap-1 cursor-pointer">
                                                        <Icon name="lock".to_string() size=11 />
                                                        <span>"Post as Staff Internal Note (Strictly Confidential — Hidden from Client)"</span>
                                                    </label>
                                                </div>

                                                <div class="flex gap-2">
                                                    <input
                                                        type="text"
                                                        placeholder=move || if is_internal_note.get() { "Write confidential internal engineering note..." } else { "Write response to client..." }
                                                        prop:value=move || message_input.get()
                                                        on:input=move |e| message_input.set(event_target_value(&e))
                                                        class="flex-1 px-3 py-2 text-xs border border-slate-300 rounded-md focus:ring-2 focus:ring-navy-800 focus:outline-none"
                                                    />
                                                    <button
                                                        type="button"
                                                        disabled=message_input.get().trim().is_empty() || is_sending.get()
                                                        on:click={
                                                            let cid_send = cur_cid.clone();
                                                            move |_| {
                                                                is_sending.set(true);
                                                                send_action.dispatch((cid_send.clone(), message_input.get(), is_internal_note.get()));
                                                            }
                                                        }
                                                        class="px-4 py-2 bg-navy-900 text-white rounded-md text-xs font-semibold hover:bg-navy-800 disabled:opacity-50 transition-colors cursor-pointer"
                                                    >
                                                        {if is_sending.get() { "Posting..." } else { "Post Note / Reply" }}
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

// ─── 8. DELIVERABLES VIEW ───────────────────────────────────────────────────

#[component]
fn AdminDeliverablesView() -> impl IntoView {
    let files_resource = Resource::new(|| (), |_| async move { get_admin_deliverables().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Platform Deliverables & Engineering Releases"</h2>
                <p class="text-xs text-slate-500">"Central repository of architecture specifications, audit certifications, and production builds."</p>
            </div>

            <Card
                title="Published Client Deliverables".to_string()
                subtitle="Verification history and download artifacts.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading deliverables..."</div> }>
                    {move || {
                        files_resource.get().map(|res| match res {
                            Ok(files) => {
                                view! {
                                    <div class="divide-y divide-slate-100">
                                        {files.into_iter().map(|f| view! {
                                            <div class="py-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                                                <div class="flex items-center gap-3">
                                                    <div class="w-8 h-8 rounded bg-navy-50 text-navy-800 flex items-center justify-center">
                                                        <Icon name="fileText".to_string() size=16 />
                                                    </div>
                                                    <div>
                                                        <p class="text-xs font-bold text-slate-900">{f.title}</p>
                                                        <p class="text-[11px] text-slate-400 font-mono">
                                                            {f.file_name} " • " {f.file_size_formatted} " • Published " {f.published_at}
                                                        </p>
                                                    </div>
                                                </div>
                                                <div class="flex items-center gap-2">
                                                    <Badge variant="navy".to_string()>{f.version}</Badge>
                                                    <Badge variant="success".to_string() dot=true>{f.status}</Badge>
                                                    <a
                                                        href=f.file_url
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                        class="inline-flex items-center gap-1 px-3 py-1.5 rounded bg-slate-100 hover:bg-slate-200 text-slate-800 text-xs font-medium transition-colors"
                                                    >
                                                        <span>"Download"</span>
                                                        <Icon name="arrowDown".to_string() size=12 />
                                                    </a>
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
            </Card>
        </div>
    }
}

// ─── 9. BILLING & SETTLEMENTS VIEW ──────────────────────────────────────────

#[component]
fn AdminBillingView() -> impl IntoView {
    let invoices_resource = Resource::new(|| (), |_| async move { get_admin_invoices().await });
    let payment_msg = RwSignal::new(Option::<String>::None);

    let mark_paid_action = Action::new(move |(id, method): &(String, String)| {
        let i = id.clone();
        let m = method.clone();
        async move {
            admin_mark_invoice_paid(i, m).await
        }
    });

    Effect::new(move |_| {
        if let Some(res) = mark_paid_action.value().get() {
            match res {
                Ok(_) => {
                    payment_msg.set(Some("Invoice successfully marked as Paid.".to_string()));
                    invoices_resource.refetch();
                }
                Err(e) => {
                    payment_msg.set(Some(format!("Error: {}", e)));
                }
            }
        }
    });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Platform Financials & Invoices Settlement"</h2>
                <p class="text-xs text-slate-500">"Tax invoice records, settlement verification, and payment auditing."</p>
            </div>

            {move || payment_msg.get().map(|msg| view! {
                <div class="p-3 bg-emerald-50 border border-emerald-200 text-emerald-800 text-xs rounded-md">
                    {msg}
                </div>
            })}

            <Card
                title="Platform Invoices Registry".to_string()
                subtitle="All issued client invoices and payment verification controls.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading platform invoices..."</div> }>
                    {move || {
                        invoices_resource.get().map(|res| match res {
                            Ok(invoices) => {
                                view! {
                                    <div class="space-y-4">
                                        {invoices.into_iter().map(|inv| {
                                            let is_paid = inv.status == "Paid";
                                            let inv_id_pay = inv.id.clone();
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
                                                        <div class="flex items-center gap-3">
                                                            <div class="text-end">
                                                                <span class="text-base font-bold font-mono text-slate-900">{inv.total_amount} " " {curr.clone()}</span>
                                                                <span class="text-[10px] text-slate-400 block font-mono">"VAT Incl."</span>
                                                            </div>
                                                            {if !is_paid {
                                                                view! {
                                                                    <button
                                                                        type="button"
                                                                        on:click=move |_| {
                                                                            mark_paid_action.dispatch((inv_id_pay.clone(), "Bank Wire Verified".to_string()));
                                                                        }
                                                                        class="px-3 py-1.5 rounded bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-semibold shadow-xs transition-colors cursor-pointer"
                                                                    >
                                                                        "Mark as Paid"
                                                                    </button>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <span class="text-[11px] font-mono text-emerald-700 bg-emerald-50 px-2.5 py-1 rounded border border-emerald-200 font-semibold">
                                                                        "Paid: " {inv.paid_at.unwrap_or_default()}
                                                                    </span>
                                                                }.into_any()
                                                            }}
                                                        </div>
                                                    </div>

                                                    <div class="divide-y divide-slate-100 text-xs">
                                                        {
                                                            let c = curr.clone();
                                                            inv.items.into_iter().map(move |it| {
                                                                let item_c = c.clone();
                                                                view! {
                                                                    <div class="py-1 flex justify-between">
                                                                        <span class="text-slate-600">{it.description} " (x" {it.quantity} ")"</span>
                                                                        <span class="font-mono text-slate-800">{it.total_amount} " " {item_c}</span>
                                                                    </div>
                                                                }
                                                            }).collect_view()
                                                        }
                                                    </div>
                                                </div>
                                            }
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
            </Card>
        </div>
    }
}

// ─── 10. SECURITY AUDIT LOGS VIEW ───────────────────────────────────────────

#[component]
fn AdminAuditLogsView() -> impl IntoView {
    let logs_resource = Resource::new(|| (), |_| async move { get_admin_audit_logs().await });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-lg font-bold text-slate-900">"Tamper-Evident Security & Audit Trail"</h2>
                <p class="text-xs text-slate-500">"Immutable recording of all authentication, authorization, financial, and lifecycle state changes across the platform."</p>
            </div>

            <Card
                title="System Audit Stream (Latest 50 Events)".to_string()
                subtitle="Zero-trust forensic log with actor ID and IP addresses.".to_string()
            >
                <Suspense fallback=|| view! { <div class="py-8 text-center text-xs text-slate-400 font-mono">"Loading audit logs..."</div> }>
                    {move || {
                        logs_resource.get().map(|res| match res {
                            Ok(logs) => {
                                view! {
                                    <div class="divide-y divide-slate-100">
                                        {logs.into_iter().map(|log| view! {
                                            <div class="py-3 flex items-start justify-between gap-3 text-xs">
                                                <div class="space-y-0.5">
                                                    <div class="flex items-center gap-2">
                                                        <span class="font-bold text-slate-900 font-mono">{log.action}</span>
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

// ─── ADMIN ACCESS DENIED & NOT FOUND VIEWS ──────────────────────────────────

#[component]
fn AdminAccessDeniedView() -> impl IntoView {
    let loc = use_location();
    let cur = move || loc.pathname.get();
    let login_url = move || {
        let p = cur();
        if p.is_empty() || p == "/" {
            "http://127.0.0.1:3000/login?return_to=http://127.0.0.1:3004".to_string()
        } else {
            format!("http://127.0.0.1:3000/login?return_to=http://127.0.0.1:3004{}", p)
        }
    };

    view! {
        <div class="min-h-screen bg-navy-950 flex items-center justify-center p-6 text-white">
            <div class="w-full max-w-md bg-navy-900 border border-navy-800 rounded-lg p-8 shadow-xl text-center space-y-4">
                <div class="w-12 h-12 rounded-full bg-red-950/60 border border-red-800 text-red-400 flex items-center justify-center mx-auto">
                    <Icon name="shield".to_string() size=24 />
                </div>
                <h1 class="font-display text-2xl font-semibold text-white">"Administrative Privileges Required"</h1>
                <p class="text-xs text-slate-400 leading-relaxed">
                    "The TeebTech Admin Management Portal is strictly restricted to platform administrators. Your session is unauthenticated or lacks the required administrative role."
                </p>
                <div class="pt-2 flex flex-col sm:flex-row items-center justify-center gap-2">
                    <a
                        href=login_url
                        class="w-full sm:w-auto inline-flex items-center justify-center font-medium rounded-md text-xs px-4 py-2 bg-brand-600 text-white hover:bg-brand-700 transition-colors shadow-sm text-decoration-none"
                    >
                        "Sign In as Admin"
                    </a>
                    <a
                        href="http://127.0.0.1:3002"
                        class="w-full sm:w-auto inline-flex items-center justify-center font-medium rounded-md text-xs px-4 py-2 bg-navy-800 text-slate-300 hover:bg-navy-700 transition-colors border border-navy-700 text-decoration-none"
                    >
                        "Client Portal"
                    </a>
                    <a
                        href="http://127.0.0.1:3000"
                        class="w-full sm:w-auto inline-flex items-center justify-center font-medium rounded-md text-xs px-4 py-2 bg-navy-800 text-slate-300 hover:bg-navy-700 transition-colors border border-navy-700 text-decoration-none"
                    >
                        "Home"
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn AdminNotFoundView() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-navy-950 p-6 text-white">
            <div class="max-w-md w-full bg-navy-900 border border-navy-800 rounded-xl p-8 shadow-xl text-center space-y-4">
                <div class="w-12 h-12 rounded-xl bg-rose-950/60 text-rose-400 border border-rose-800/50 flex items-center justify-center mx-auto">
                    <Icon name="alertCircle".to_string() size=24 />
                </div>
                <h1 class="text-xl font-bold text-white">"Admin Route Not Found"</h1>
                <p class="text-xs text-slate-400 leading-relaxed">
                    "The administrative endpoint or dashboard resource you requested does not exist."
                </p>
                <div class="pt-2">
                    <a
                        href="/"
                        class="inline-flex items-center justify-center px-4 py-2 rounded-lg bg-navy-800 border border-navy-700 text-white font-medium text-xs hover:bg-navy-700 transition-colors shadow-xs text-decoration-none"
                    >
                        "Return to Admin Overview"
                    </a>
                </div>
            </div>
        </div>
    }
}
