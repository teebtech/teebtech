use leptos::prelude::*;

// ─── ICON COMPONENT ──────────────────────────────────────────────────────────

/// SVG Icon renderer matching the reference design icon set.
#[component]
pub fn Icon(
    #[prop(into)] name: String,
    #[prop(optional)] size: Option<usize>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let size_val = size.unwrap_or(16);
    let class_str = class.unwrap_or_default();

    let inner_svg = match name.as_str() {
        "check" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/> }.into_any(),
        "x" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/> }.into_any(),
        "chevronDown" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/> }.into_any(),
        "chevronRight" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/> }.into_any(),
        "chevronLeft" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7"/> }.into_any(),
        "chevronUp" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M5 15l7-7 7 7"/> }.into_any(),
        "checkCircle" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/> }.into_any(),
        "xCircle" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"/> }.into_any(),
        "shield" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/> }.into_any(),
        "lock" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/> }.into_any(),
        "building" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/> }.into_any(),
        "users" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/> }.into_any(),
        "user" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/> }.into_any(),
        "briefcase" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/> }.into_any(),
        "layers" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M12 2l10 6.5-10 6.5L2 8.5 12 2zM2 15.5L12 22l10-6.5M2 11.5l10 6.5 10-6.5"/> }.into_any(),
        "activity" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M22 12h-4l-3 9L9 3l-3 9H2"/> }.into_any(),
        "refresh" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/> }.into_any(),
        "logout" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/> }.into_any(),
        "arrowRight" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M14 5l7 7m0 0l-7 7m7-7H3"/> }.into_any(),
        "arrowUp" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M5 10l7-7m0 0l7 7m-7-7v18"/> }.into_any(),
        "arrowDown" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M19 14l-7 7m0 0l-7-7m7 7V3"/> }.into_any(),
        "chart" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/> }.into_any(),
        "globe" => view! { <circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10 15.3 15.3 0 01-4-10 15.3 15.3 0 014-10z"/> }.into_any(),
        "code" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/> }.into_any(),
        "server" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"/> }.into_any(),
        "cpu" => view! { <rect x="4" y="4" width="16" height="16" rx="2"/><rect x="9" y="9" width="6" height="6"/><line x1="9" y1="1" x2="9" y2="4"/><line x1="15" y1="1" x2="15" y2="4"/><line x1="9" y1="20" x2="9" y2="23"/><line x1="15" y1="20" x2="15" y2="23"/><line x1="20" y1="9" x2="23" y2="9"/><line x1="20" y1="14" x2="23" y2="14"/><line x1="1" y1="9" x2="4" y2="9"/><line x1="1" y1="14" x2="4" y2="14"/> }.into_any(),
        "database" => view! { <ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/><path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/> }.into_any(),
        "settings" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z"/> }.into_any(),
        "trending" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/> }.into_any(),
        "star" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"/> }.into_any(),
        "mail" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/> }.into_any(),
        "chat" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/> }.into_any(),
        "menu" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"/> }.into_any(),
        "externalLink" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/> }.into_any(),
        "search" => view! { <circle cx="11" cy="11" r="8"/><path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-4.35-4.35"/> }.into_any(),
        "eye" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/><path stroke-linecap="round" stroke-linejoin="round" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/> }.into_any(),
        "eyeOff" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l18 18"/> }.into_any(),
        "fileText" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/> }.into_any(),
        "alertCircle" => view! { <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/> }.into_any(),
        "info" => view! { <circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/> }.into_any(),
        "arrowLeft" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18"/> }.into_any(),
        "clock" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/> }.into_any(),
        "messageSquare" => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M21 15a2 2 0 01-2 2H7l-4 4V5a2 2 0 012-2h14a2 2 0 012 2z"/> }.into_any(),
        _ => view! { <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/> }.into_any(),
    };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size_val.to_string()
            height=size_val.to_string()
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.75"
            class=class_str
        >
            {inner_svg}
        </svg>
    }
}

// ─── SECTION HEADING COMPONENT ────────────────────────────────────────────────

/// Section Heading with eyebrow, title, and description matching reference UI.
#[component]
pub fn SectionHeading(
    #[prop(optional, into)] eyebrow: Option<String>,
    #[prop(into)] title: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] center: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let is_center = center.unwrap_or(false);
    let extra_class = class.unwrap_or_default();
    let align_class = if is_center { "text-center" } else { "" };

    view! {
        <div class=format!("{} {}", align_class, extra_class)>
            {eyebrow.map(|eb| view! {
                <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                    {eb}
                </p>
            })}
            <h2 class="font-display text-3xl lg:text-4xl font-semibold text-slate-900 leading-tight">
                {title}
            </h2>
            {description.map(|desc| {
                let max_w_class = if is_center { "max-w-2xl mx-auto" } else { "max-w-2xl" };
                view! {
                    <p class=format!("mt-3 text-slate-500 text-base leading-relaxed {}", max_w_class)>
                        {desc}
                    </p>
                }
            })}
        </div>
    }
}

// ─── BUTTON COMPONENT ─────────────────────────────────────────────────────────

/// Reusable Button component matching reference UI styles.
#[component]
pub fn Button(
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional)] loading: Option<bool>,
    #[prop(optional)] full_width: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    children: Children,
) -> impl IntoView {
    let var = variant.unwrap_or_else(|| "primary".to_string());
    let sz = size.unwrap_or_else(|| "md".to_string());
    let is_loading = loading.unwrap_or(false);
    let is_disabled = disabled.unwrap_or(false) || is_loading;
    let is_full = full_width.unwrap_or(false);
    let extra_class = class.unwrap_or_default();

    let var_classes = match var.as_str() {
        "secondary" => "bg-slate-100 text-slate-800 hover:bg-slate-200 active:bg-slate-300 border border-slate-200",
        "outline" => "border border-slate-300 text-slate-700 hover:bg-slate-50 active:bg-slate-100 bg-white shadow-sm",
        "ghost" => "text-slate-600 hover:bg-slate-100 active:bg-slate-200",
        "danger" => "bg-red-600 text-white hover:bg-red-700 active:bg-red-800 shadow-sm border border-red-700",
        "success" => "bg-emerald-600 text-white hover:bg-emerald-700 shadow-sm border border-emerald-700",
        "navy" => "bg-navy-900 text-white hover:bg-navy-800 shadow-sm border border-navy-800",
        _ => "bg-brand-600 text-white hover:bg-brand-700 active:bg-brand-800 shadow-sm border border-brand-700",
    };

    let size_classes = match sz.as_str() {
        "xs" => "h-6 px-2 text-xs gap-1",
        "sm" => "h-8 px-3 text-xs gap-1.5",
        "lg" => "h-11 px-6 text-sm gap-2",
        _ => "h-9 px-4 text-sm gap-2",
    };

    let width_class = if is_full { "w-full" } else { "" };

    view! {
        <button
            disabled=is_disabled
            class=format!(
                "inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand-500 focus-visible:ring-offset-1 disabled:opacity-50 disabled:cursor-not-allowed select-none whitespace-nowrap cursor-pointer {} {} {} {}",
                var_classes, size_classes, width_class, extra_class
            )
        >
            {children()}
        </button>
    }
}

// ─── BADGE COMPONENT ──────────────────────────────────────────────────────────

/// Reusable Badge component matching reference UI styles.
#[component]
pub fn Badge(
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional)] dot: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let var = variant.unwrap_or_else(|| "default".to_string());
    let show_dot = dot.unwrap_or(false);
    let extra_class = class.unwrap_or_default();

    let (bg_class, dot_color) = match var.as_str() {
        "success" => (
            "bg-emerald-50 text-emerald-700 border border-emerald-200",
            "bg-emerald-500",
        ),
        "warning" => (
            "bg-amber-50 text-amber-700 border border-amber-200",
            "bg-amber-500",
        ),
        "error" => ("bg-red-50 text-red-700 border border-red-200", "bg-red-500"),
        "info" => (
            "bg-cyan-50 text-cyan-700 border border-cyan-200",
            "bg-cyan-500",
        ),
        "navy" => ("bg-navy-900 text-white border border-navy-800", "bg-white"),
        "neutral" => (
            "bg-slate-100 text-slate-600 border border-slate-200",
            "bg-slate-400",
        ),
        "cyan" => ("bg-cyan-600 text-white border border-cyan-700", "bg-white"),
        "purple" => (
            "bg-purple-50 text-purple-700 border border-purple-200",
            "bg-purple-500",
        ),
        _ => (
            "bg-brand-50 text-brand-700 border border-brand-200",
            "bg-brand-500",
        ),
    };

    view! {
        <span class=format!("inline-flex items-center gap-1 font-medium rounded-full px-2.5 py-0.5 text-xs {} {}", bg_class, extra_class)>
            {if show_dot {
                view! { <span class=format!("w-1.5 h-1.5 rounded-full {}", dot_color)></span> }.into_any()
            } else {
                view! { <span class="hidden"></span> }.into_any()
            }}
            {children()}
        </span>
    }
}

// ─── CARD COMPONENT ───────────────────────────────────────────────────────────

/// Reusable Card component wrapper matching reference UI styles.
#[component]
pub fn Card(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] subtitle: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let extra_class = class.unwrap_or_default();

    view! {
        <div class=format!("bg-white border border-slate-200 rounded-lg p-6 shadow-sm {}", extra_class)>
            {if title.is_some() || subtitle.is_some() {
                view! {
                    <div class="mb-4 border-b border-slate-100 pb-3">
                        {title.map(|t| view! { <h3 class="font-display text-lg font-semibold text-slate-900">{t}</h3> })}
                        {subtitle.map(|st| view! { <p class="text-xs text-slate-500 mt-0.5">{st}</p> })}
                    </div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}
            <div>
                {children()}
            </div>
        </div>
    }
}

// ─── STAT CARD COMPONENT ──────────────────────────────────────────────────────

/// Reusable KPI Stat Card matching reference UI styles.
#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional, into)] delta: Option<String>,
    #[prop(optional)] delta_up: Option<bool>,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let extra_class = class.unwrap_or_default();
    let is_up = delta_up.unwrap_or(true);

    view! {
        <div class=format!("bg-white border border-slate-200 rounded-lg p-5 shadow-sm {}", extra_class)>
            <div class="flex items-start justify-between">
                <div>
                    <p class="text-xs font-medium text-slate-500 uppercase tracking-wider">{label}</p>
                    <p class="mt-1 text-2xl font-semibold text-slate-900 font-display">{value}</p>
                    {delta.map(|d| {
                        let text_color = if is_up { "text-emerald-600" } else { "text-red-500" };
                        let icon_name = if is_up { "arrowUp" } else { "arrowDown" };
                        view! {
                            <p class=format!("mt-1 text-xs font-medium flex items-center gap-1 {}", text_color)>
                                <Icon name=icon_name.to_string() size=10 />
                                {d}
                            </p>
                        }
                    })}
                </div>
                {icon.map(|ic| view! {
                    <div class="w-9 h-9 rounded-md bg-slate-50 border border-slate-100 flex items-center justify-center text-brand-600">
                        <Icon name=ic size=18 />
                    </div>
                })}
            </div>
        </div>
    }
}

// ─── ALERT COMPONENT ──────────────────────────────────────────────────────────

/// Reusable Alert banner matching reference UI styles.
#[component]
pub fn Alert(
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let var = variant.unwrap_or_else(|| "error".to_string());
    let extra_class = class.unwrap_or_default();

    let (bg_class, icon_name) = match var.as_str() {
        "success" => (
            "bg-emerald-50 border-emerald-200 text-emerald-800",
            "checkCircle",
        ),
        "warning" => ("bg-amber-50 border-amber-200 text-amber-800", "shield"),
        "info" => ("bg-brand-50 border-brand-200 text-brand-800", "globe"),
        _ => ("bg-red-50 border-red-200 text-red-800", "xCircle"),
    };

    view! {
        <div class=format!("p-3.5 rounded-lg border text-xs flex items-center gap-2.5 {} {}", bg_class, extra_class)>
            <Icon name=icon_name.to_string() size=16 class="flex-shrink-0" />
            <div class="flex-1 font-medium">
                {children()}
            </div>
        </div>
    }
}
