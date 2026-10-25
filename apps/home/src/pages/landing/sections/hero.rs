//! # Interactive 3D Multi-Device Hero Section
//!
//! Faithfully recreates the Nuqta / Olive reference hero section adapted to TeebTech's
//! white/light modern SaaS brand:
//! - Left: High-impact typography, interactive business selector, platform toggle chips,
//!   dynamic SAR cost & days calculator with bundle discount, and WhatsApp pre-filled CTA.
//! - Right: Real 3D interactive multi-device stage (Web, iOS, Android, Desktop, Admin Dashboard,
//!   API & Backend) with CSS container queries, perspective, pointer tilt physics, floating micro-animation,
//!   shimmers, live charts, and active device auto-tour.

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BusinessType {
    pub emoji: &'static str,
    pub en: &'static str,
    pub ar: &'static str,
    pub tagline_en: &'static str,
    pub tagline_ar: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlatformInfo {
    pub key: &'static str,
    pub emoji: &'static str,
    pub en: &'static str,
    pub ar: &'static str,
    pub price: u32,
    pub days: u32,
    pub desc_en: &'static str,
    pub desc_ar: &'static str,
    pub z: &'static str,
    pub left: &'static str,
    pub top: &'static str,
    pub width: &'static str,
    pub aspect: &'static str,
}

const BUSINESS_TYPES: [BusinessType; 4] = [
    BusinessType {
        emoji: "🍽️",
        en: "Restaurant",
        ar: "مطعم",
        tagline_en: "Taste the tradition",
        tagline_ar: "تذوّق الأصالة",
    },
    BusinessType {
        emoji: "💍",
        en: "Weddings & events",
        ar: "أعراس وفعاليات",
        tagline_en: "Plan your perfect night",
        tagline_ar: "خطّط لليلة العمر",
    },
    BusinessType {
        emoji: "🛍️",
        en: "Online store",
        ar: "متجر إلكتروني",
        tagline_en: "Shop what you love",
        tagline_ar: "تسوّق ما تحب",
    },
    BusinessType {
        emoji: "🩺",
        en: "Clinic",
        ar: "عيادة",
        tagline_en: "Care that fits your day",
        tagline_ar: "رعاية تناسب يومك",
    },
];

const PLATFORMS: [PlatformInfo; 6] = [
    PlatformInfo {
        key: "web",
        emoji: "🌐",
        en: "Website",
        ar: "موقع ويب",
        price: 1800,
        days: 7,
        desc_en: "Fast, SEO-ready, Arabic and English",
        desc_ar: "سريع ومتوافق مع SEO بالعربية والإنجليزية",
        z: "0px",
        left: "12%",
        top: "3%",
        width: "76%",
        aspect: "16/10",
    },
    PlatformInfo {
        key: "ios",
        emoji: "📱",
        en: "iPhone app",
        ar: "تطبيق آيفون",
        price: 6000,
        days: 25,
        desc_en: "Built in Swift, ready for the App Store",
        desc_ar: "مبني بـ Swift وجاهز لمتجر آب ستور",
        z: "110px",
        left: "70%",
        top: "35%",
        width: "21%",
        aspect: "9/19.5",
    },
    PlatformInfo {
        key: "and",
        emoji: "🤖",
        en: "Android app",
        ar: "تطبيق أندرويد",
        price: 5500,
        days: 25,
        desc_en: "Built in Kotlin, ready for Google Play",
        desc_ar: "مبني بـ Kotlin وجاهز لمتجر جوجل بلاي",
        z: "80px",
        left: "43%",
        top: "40%",
        width: "19%",
        aspect: "9/19",
    },
    PlatformInfo {
        key: "desk",
        emoji: "🖥️",
        en: "Desktop app",
        ar: "تطبيق سطح المكتب",
        price: 4500,
        days: 20,
        desc_en: "One app for Windows and macOS",
        desc_ar: "تطبيق واحد لويندوز وماك",
        z: "40px",
        left: "0",
        top: "37%",
        width: "47%",
        aspect: "16/11",
    },
    PlatformInfo {
        key: "dash",
        emoji: "📊",
        en: "Admin dashboard",
        ar: "لوحة تحكم",
        price: 2500,
        days: 10,
        desc_en: "Orders, users and live reports",
        desc_ar: "الطلبات والمستخدمون وتقارير مباشرة",
        z: "130px",
        left: "63%",
        top: "-5%",
        width: "36%",
        aspect: "4/3",
    },
    PlatformInfo {
        key: "api",
        emoji: "⚙️",
        en: "API & backend",
        ar: "واجهة برمجية وخادم",
        price: 2000,
        days: 8,
        desc_en: "Secure, scalable, payments included",
        desc_ar: "آمن وقابل للتوسع مع الدفع",
        z: "120px",
        left: "-1%",
        top: "-2%",
        width: "37%",
        aspect: "16/9",
    },
];

const WHATSAPP_NUMBER: &str = "966500000000";

fn format_number(val: u32) -> String {
    let s = val.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + (len / 3));
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push(',');
        }
        out.push(b as char);
    }
    out
}

#[component]
pub fn Hero() -> impl IntoView {
    let locale_ctx = crate::i18n::use_locale();
    let selected_biz_idx = RwSignal::new(0usize);
    let enabled_platforms = RwSignal::new(vec!["web", "ios", "and", "dash"]);
    let active_platform = RwSignal::new("web");

    // 3D tilt angles
    let rx = RwSignal::new(0.0f64);
    let ry = RwSignal::new(0.0f64);

    let is_arabic = move || locale_ctx.is_arabic();

    let current_biz = move || {
        let idx = selected_biz_idx.get();
        BUSINESS_TYPES[idx.min(BUSINESS_TYPES.len() - 1)]
    };

    let is_plat_enabled = move |key: &'static str| enabled_platforms.get().contains(&key);

    let is_plat_active =
        move |key: &'static str| active_platform.get() == key && is_plat_enabled(key);

    // Cost & timeline calculations
    let pricing_stats = move || {
        let enabled = enabled_platforms.get();
        let mut total_price = 0u32;
        let mut total_days = 0u32;
        let mut count = 0usize;

        for p in PLATFORMS.iter() {
            if enabled.contains(&p.key) {
                total_price += p.price;
                total_days += p.days;
                count += 1;
            }
        }

        let has_discount = count >= 3;
        let final_price = if has_discount {
            ((total_price as f64) * 0.9).round() as u32
        } else {
            total_price
        };
        let final_days = ((total_days as f64) * 0.7).round() as u32;

        (final_price, final_days, has_discount, count)
    };

    // Active platform details for caption card
    let active_platform_info = move || {
        let act = active_platform.get();
        PLATFORMS
            .iter()
            .find(|p| p.key == act)
            .copied()
            .unwrap_or(PLATFORMS[0])
    };

    // Generate WhatsApp link
    let whatsapp_url = move || {
        let biz = current_biz();
        let (price, days, _, _) = pricing_stats();
        let enabled = enabled_platforms.get();
        let plat_names: Vec<&str> = PLATFORMS
            .iter()
            .filter(|p| enabled.contains(&p.key))
            .map(|p| if is_arabic() { p.ar } else { p.en })
            .collect();
        let plat_str = if plat_names.is_empty() {
            "-".to_string()
        } else {
            plat_names.join(", ")
        };

        let msg = if is_arabic() {
            format!(
                "مرحباً! أود بدء مشروع.\nالنشاط: {}\nالمنصات: {}\nالتقدير: {} ر.س / {} يوم",
                biz.ar,
                plat_str,
                format_number(price),
                days
            )
        } else {
            format!(
                "Salam! I want to build a product.\nBusiness: {}\nPlatforms: {}\nEstimate: {} SAR / {} days",
                biz.en, plat_str, format_number(price), days
            )
        };

        // URL encode simple characters
        let mut encoded = String::new();
        for b in msg.bytes() {
            match b {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    encoded.push(b as char);
                }
                b' ' => encoded.push_str("%20"),
                b'\n' => encoded.push_str("%0A"),
                b':' => encoded.push_str("%3A"),
                b',' => encoded.push_str("%2C"),
                b'/' => encoded.push_str("%2F"),
                _ => {
                    encoded.push_str(&format!("%{:02X}", b));
                }
            }
        }
        format!("https://wa.me/{}?text={}", WHATSAPP_NUMBER, encoded)
    };

    // Hydration effects: 3D pointer tracking & auto-tour
    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let auto_tour_active = active_platform;
        let auto_tour_enabled = enabled_platforms;

        if let Some(window) = web_sys::window() {
            let cb = Closure::wrap(Box::new(move || {
                let enabled = auto_tour_enabled.get();
                if enabled.len() >= 2 {
                    let cur = auto_tour_active.get();
                    if let Some(pos) = enabled.iter().position(|&k| k == cur) {
                        let next_pos = (pos + 1) % enabled.len();
                        auto_tour_active.set(enabled[next_pos]);
                    } else if !enabled.is_empty() {
                        auto_tour_active.set(enabled[0]);
                    }
                }
            }) as Box<dyn FnMut()>);

            let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                3400,
            );
            cb.forget();
        }
    }

    // Pointer move handler for 3D tilt
    let on_mouse_move = move |_ev: leptos::ev::MouseEvent| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            if let Some(target) = _ev.current_target() {
                if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                    let rect = el.get_bounding_client_rect();
                    let width = rect.width();
                    let height = rect.height();
                    if width > 0.0 && height > 0.0 {
                        let x = (_ev.client_x() as f64 - rect.left()) / width - 0.5;
                        let y = (_ev.client_y() as f64 - rect.top()) / height - 0.5;
                        rx.set(-y * 10.0);
                        ry.set(x * 14.0);
                    }
                }
            }
        }
    };

    let on_mouse_leave = move |_| {
        rx.set(0.0);
        ry.set(0.0);
    };

    view! {
        <section
            id="hero"
            class="relative isolate overflow-hidden bg-slate-50 border-b border-slate-200/80 transition-colors"
            dir=move || if is_arabic() { "rtl" } else { "ltr" }
            on:mousemove=on_mouse_move
            on:mouseleave=on_mouse_leave
        >
            // Ambient subtle dot grid background with light mask
            <div class="hero-stage-dots absolute inset-0 -z-10 pointer-events-none"></div>

            // Ambient soft glowing brand gradient blob
            <div
                class="absolute -top-32 -z-10 h-[36rem] w-[36rem] rounded-full bg-brand-200/40 opacity-70 blur-3xl pointer-events-none"
                style=move || if is_arabic() { "inset-inline-start: -6rem;" } else { "inset-inline-end: -6rem;" }
            ></div>

            <div class="mx-auto grid max-w-7xl items-center gap-10 px-4 sm:px-6 lg:px-8 pb-16 pt-6 lg:grid-cols-[1fr_1.05fr] lg:pt-10">
                // Left Column: Interactive Proposal & Cost Estimator
                <div class="hero-rise">
                    // Top language and trust pill
                    <div class="flex items-center gap-2 mb-4">
                        <button
                            type="button"
                            on:click=move |_| locale_ctx.toggle()
                            class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold bg-white border border-slate-200 text-slate-700 hover:border-brand-500 hover:text-brand-600 transition shadow-xs cursor-pointer"
                            aria-label="Switch language"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-slate-400">
                                <circle cx="12" cy="12" r="10"></circle>
                                <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
                                <path d="M2 12h20"></path>
                            </svg>
                            {move || if is_arabic() { "English" } else { "العربية" }}
                        </button>

                        <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-brand-50 text-brand-700 border border-brand-200/80">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                            {move || if is_arabic() { "فريق تقني متكامل" } else { "Cross-Platform Engineering Team" }}
                        </span>
                    </div>

                    // Main Heading
                    <h1 class="font-hero text-4xl sm:text-5xl lg:text-[3.6rem] font-extrabold text-slate-900 leading-[1.08] tracking-tight">
                        {move || if is_arabic() {
                            "فكرة واحدة، وكل الشاشات التي تحتاجها."
                        } else {
                            "One idea. Every screen it needs to live on."
                        }}
                    </h1>

                    // Subtitle
                    <p class="mt-4 text-base sm:text-lg text-slate-600 max-w-xl leading-relaxed">
                        {move || if is_arabic() {
                            "مواقع وتطبيقات آيفون وأندرويد وبرامج سطح المكتب والخادم خلفها، بفريق واحد. اختر منصة وشاهدها تظهر."
                        } else {
                            "Websites, iPhone and Android apps, desktop software and the backend behind them, built by one team. Tap a platform to watch it appear."
                        }}
                    </p>

                    // Interactive Configurator Card
                    <div class="mt-8 space-y-5 rounded-3xl border border-slate-200/90 bg-white/85 p-5 sm:p-6 backdrop-blur-xl shadow-lg shadow-slate-200/40">
                        // Business Type Selector
                        <div>
                            <p class="mb-2 text-xs font-bold uppercase tracking-wider text-slate-500">
                                {move || if is_arabic() { "نشاطك التجاري" } else { "Your business" }}
                            </p>
                            <div class="flex flex-wrap gap-2">
                                {BUSINESS_TYPES.iter().enumerate().map(|(i, b)| {
                                    let is_selected = move || selected_biz_idx.get() == i;
                                    view! {
                                        <button
                                            type="button"
                                            class=move || {
                                                format!(
                                                    "hero-chip inline-flex items-center gap-1.5 rounded-full border px-4 py-2 text-sm font-medium transition cursor-pointer select-none {}",
                                                    if is_selected() {
                                                        "bg-brand-600 text-white border-brand-600 shadow-sm font-bold"
                                                    } else {
                                                        "bg-white text-slate-700 border-slate-200 hover:border-slate-300 hover:bg-slate-50"
                                                    }
                                                )
                                            }
                                            aria-pressed=move || is_selected().to_string()
                                            on:click=move |_| {
                                                selected_biz_idx.set(i);
                                            }
                                        >
                                            <span>{b.emoji}</span>
                                            <span>{move || if is_arabic() { b.ar } else { b.en }}</span>
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // Platform Selector
                        <div>
                            <p class="mb-2 text-xs font-bold uppercase tracking-wider text-slate-500">
                                {move || if is_arabic() { "المنصات التي تحتاجها" } else { "Platforms you need" }}
                            </p>
                            <div class="flex flex-wrap gap-2">
                                {PLATFORMS.iter().map(|p| {
                                    let key = p.key;
                                    let is_enabled = move || is_plat_enabled(key);
                                    let is_active = move || is_plat_active(key);
                                    view! {
                                        <button
                                            type="button"
                                            class=move || {
                                                format!(
                                                    "hero-chip inline-flex items-center gap-1.5 rounded-full border px-4 py-2 text-sm font-medium transition cursor-pointer select-none {}",
                                                    if is_enabled() {
                                                        if is_active() {
                                                            "bg-brand-600 text-white border-brand-600 shadow-sm font-bold ring-2 ring-brand-400/50"
                                                        } else {
                                                            "bg-brand-600 text-white border-brand-600 shadow-sm font-bold"
                                                        }
                                                    } else {
                                                        "bg-white text-slate-600 border-slate-200 hover:border-slate-300 hover:bg-slate-50"
                                                    }
                                                )
                                            }
                                            aria-pressed=move || is_enabled().to_string()
                                            on:click=move |_| {
                                                let mut cur = enabled_platforms.get();
                                                if cur.contains(&key) {
                                                    if cur.len() > 1 {
                                                        cur.retain(|&k| k != key);
                                                        enabled_platforms.set(cur.clone());
                                                        if active_platform.get() == key {
                                                            active_platform.set(cur[0]);
                                                        }
                                                    }
                                                } else {
                                                    cur.push(key);
                                                    enabled_platforms.set(cur);
                                                    active_platform.set(key);
                                                }
                                            }
                                        >
                                            <span>{p.emoji}</span>
                                            <span>{move || if is_arabic() { p.ar } else { p.en }}</span>
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // Bottom Pricing & WhatsApp CTA Bar
                        <div class="flex flex-col sm:flex-row sm:items-end justify-between gap-4 border-t border-slate-200 pt-4">
                            <div>
                                <p class="text-xs font-medium text-slate-500 uppercase tracking-wide">
                                    {move || if is_arabic() { "التكلفة التقريبية" } else { "Estimated cost" }}
                                </p>
                                <p class="font-hero text-3xl font-extrabold text-slate-900 tracking-tight flex items-baseline gap-1.5">
                                    <span>{move || format_number(pricing_stats().0)}</span>
                                    <span class="text-base font-semibold text-slate-500">
                                        {move || if is_arabic() { "ر.س" } else { "SAR" }}
                                    </span>
                                </p>
                                <p class="text-sm text-slate-500 flex items-center gap-1.5 mt-0.5">
                                    <span class="font-bold text-slate-700">{move || pricing_stats().1}</span>
                                    <span>{move || if is_arabic() { "يوم للإطلاق" } else { "days to launch" }}</span>
                                    {move || {
                                        let (_, _, has_discount, _) = pricing_stats();
                                        if has_discount {
                                            view! {
                                                <span class="inline-flex items-center rounded-full bg-brand-100 text-brand-800 border border-brand-200 px-2 py-0.5 text-xs font-bold">
                                                    {move || if is_arabic() { "خصم الباقة −10%" } else { "Bundle −10%" }}
                                                </span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }}
                                </p>
                            </div>

                            <a
                                href=whatsapp_url
                                target="_blank"
                                rel="noopener noreferrer"
                                class="inline-flex items-center justify-center gap-2 rounded-2xl bg-brand-600 hover:bg-brand-700 text-white px-5 sm:px-6 py-3 sm:py-3.5 text-sm sm:text-base font-extrabold shadow-lg shadow-brand-500/25 transition hover:scale-[1.02] active:scale-[0.98] shrink-0 cursor-pointer"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="currentColor" class="text-white shrink-0">
                                    <path d="M12.031 6.172c-3.181 0-5.767 2.586-5.768 5.766-.001 1.298.38 2.27 1.019 3.287l-.582 2.128 2.182-.573c.978.58 1.911.928 3.145.929 3.178 0 5.767-2.587 5.768-5.766.001-3.187-2.575-5.77-5.764-5.771zm3.392 8.244c-.144.405-.837.774-1.17.824-.299.045-.677.063-1.092-.069-.252-.08-.575-.187-.988-.365-1.739-.751-2.874-2.502-2.961-2.617-.087-.116-.708-.94-.708-1.793s.448-1.273.607-1.446c.159-.173.346-.217.462-.217l.332.006c.106.005.249-.04.39.298.144.347.491 1.2.534 1.287.043.087.072.188.014.304-.058.116-.087.188-.173.289l-.26.304c-.087.086-.177.18-.076.354.101.174.449.741.964 1.201.662.591 1.221.774 1.394.86.173.086.275.072.376-.044.101-.116.433-.506.549-.68.116-.173.231-.145.39-.086s1.011.477 1.184.564.289.13.332.202c.045.072.045.419-.099.824zm-3.423-14.416c-6.627 0-12 5.373-12 12 0 2.158.572 4.183 1.572 5.932l-1.667 6.096 6.267-1.644c1.706.938 3.666 1.474 5.753 1.474 6.627 0 12-5.373 12-12s-5.373-12-12-12z"/>
                                </svg>
                                <span>{move || if is_arabic() { "أرسل هذه الخطة عبر واتساب" } else { "Send this plan on WhatsApp" }}</span>
                            </a>
                        </div>
                    </div>
                </div>

                // Right Column: 3D Interactive Multi-Device Stage
                <div class="mx-auto w-full max-w-xl select-none" role="img" aria-label="Live interactive 3D preview of software on web, iPhone, Android, desktop, dashboard, and API">
                    <div class="hero-cq">
                        <div class="hero-float">
                            <div
                                id="stage"
                                class="hero-tilt relative w-full"
                                style=move || {
                                    format!("aspect-ratio: 1 / 0.9; --rx: {:.2}deg; --ry: {:.2}deg;", rx.get(), ry.get())
                                }
                            >
                                // 1. API & Backend Terminal
                                <div
                                    class=move || format!("hero-dv{}", if is_plat_active("api") { " on" } else { "" })
                                    data-off=move || (!is_plat_enabled("api")).then_some("")
                                    style="--z: 120px; left: -1%; top: -2%; width: 37%;"
                                    on:click=move |_| {
                                        if is_plat_enabled("api") { active_platform.set("api"); }
                                    }
                                >
                                    <div class="hero-in rounded-[1.2cqw] border border-slate-700/60 p-[1.8cqw] font-mono shadow-xl cursor-pointer" style="aspect-ratio: 16/9; background: #0b1220; color: #34d399; font-size: 1.9cqw;" dir="ltr">
                                        <div class="flex items-center gap-[0.8cqw] pb-[1cqw] mb-[0.8cqw] border-b border-slate-800">
                                            <span class="h-[0.9cqw] w-[0.9cqw] rounded-full bg-emerald-500"></span>
                                            <span class="text-slate-400 text-[1.4cqw]">"API Gateway"</span>
                                        </div>
                                        <span class="hero-ty text-emerald-400">"POST /v1/orders"</span>
                                        <span class="hero-ty text-slate-300" style="animation-delay: 0.9s;">"200 OK · 24ms"</span>
                                        <span class="hero-ty text-brand-300" style="animation-delay: 1.8s;">"{\"status\": \"paid\"}"</span>
                                        <span class="hero-ty text-slate-400" style="animation-delay: 2.7s;">"push → iOS, Android"</span>
                                    </div>
                                </div>

                                // 2. Web Browser Mockup
                                <div
                                    class=move || format!("hero-dv{}", if is_plat_active("web") { " on" } else { "" })
                                    data-off=move || (!is_plat_enabled("web")).then_some("")
                                    style="--z: 0px; left: 12%; top: 3%; width: 76%;"
                                    on:click=move |_| {
                                        if is_plat_enabled("web") { active_platform.set("web"); }
                                    }
                                >
                                    <div class="hero-in rounded-[1.4cqw] shadow-xl cursor-pointer" style="aspect-ratio: 16/10;">
                                        <div class="hero-ld"></div>
                                        <div class="flex items-center gap-[1cqw] border-b border-slate-200/80 px-[2cqw] py-[1.2cqw] bg-slate-50/90">
                                            <i class="h-[1.2cqw] w-[1.2cqw] rounded-full bg-red-400"></i>
                                            <i class="h-[1.2cqw] w-[1.2cqw] rounded-full bg-amber-400"></i>
                                            <i class="h-[1.2cqw] w-[1.2cqw] rounded-full bg-emerald-500"></i>
                                            <span class="ms-[1cqw] flex-1 rounded-full bg-white border border-slate-200/80 px-[1.6cqw] py-[0.4cqw] text-slate-500 text-[1.6cqw] truncate">
                                                {move || if is_arabic() { "علامتك.sa" } else { "yourbrand.sa" }}
                                            </span>
                                        </div>
                                        <div class="p-[2.2cqw]">
                                            <div class="rounded-[1.4cqw] p-[2.6cqw] bg-brand-50/90 border border-brand-100">
                                                <p class="font-hero font-extrabold text-slate-900 tracking-tight" style="font-size: 3.6cqw;">
                                                    {move || {
                                                        let b = current_biz();
                                                        if is_arabic() { b.tagline_ar } else { b.tagline_en }
                                                    }}
                                                </p>
                                                <i class="hero-shim mt-[1.4cqw] block h-[1cqw] w-3/5 rounded bg-slate-200"></i>
                                                <span class="mt-[2cqw] inline-block rounded-[0.9cqw] bg-brand-600 px-[2.4cqw] py-[0.9cqw] font-bold text-white shadow-xs">
                                                    {move || current_biz().emoji}
                                                </span>
                                            </div>
                                            <div class="mt-[2cqw] grid grid-cols-3 gap-[1.6cqw]">
                                                {[0, 1, 2].into_iter().map(|_| {
                                                    view! {
                                                        <div class="rounded-[1cqw] border border-slate-200/80 p-[1.2cqw] bg-white">
                                                            <i class="hero-shim mb-[1cqw] block h-[6cqw] rounded-[0.8cqw]"></i>
                                                            <i class="hero-shim block h-[0.9cqw] w-4/5 rounded"></i>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // 3. Admin Dashboard
                                <div
                                    class=move || format!("hero-dv{}", if is_plat_active("dash") { " on" } else { "" })
                                    data-off=move || (!is_plat_enabled("dash")).then_some("")
                                    style="--z: 130px; left: 63%; top: -5%; width: 36%;"
                                    on:click=move |_| {
                                        if is_plat_enabled("dash") { active_platform.set("dash"); }
                                    }
                                >
                                    <div class="hero-in rounded-[1.4cqw] p-[1.8cqw] shadow-xl cursor-pointer" style="aspect-ratio: 4/3;">
                                        <div class="flex items-baseline justify-between">
                                            <b class="font-bold text-slate-800 text-[2cqw]">
                                                {move || if is_arabic() { "الإيرادات" } else { "Revenue" }}
                                            </b>
                                            <b class="text-brand-600 font-extrabold text-[2.2cqw]">"+24%"</b>
                                        </div>
                                        <div class="mt-[1.4cqw] flex h-[10cqw] items-end gap-[1cqw]">
                                            {[5, 7, 4, 8, 6, 9, 7].into_iter().enumerate().map(|(i, h)| {
                                                let style_str = format!(
                                                    "height: {}%; animation-delay: {:.2}s; opacity: {:.2};",
                                                    h * 10,
                                                    i as f64 * 0.2,
                                                    0.45 + i as f64 * 0.08
                                                );
                                                view! {
                                                    <i
                                                        class="hero-bar flex-1 rounded-t-[0.5cqw] bg-brand-600"
                                                        style=style_str
                                                    ></i>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                </div>

                                // 4. Desktop Application
                                <div
                                    class=move || format!("hero-dv{}", if is_plat_active("desk") { " on" } else { "" })
                                    data-off=move || (!is_plat_enabled("desk")).then_some("")
                                    style="--z: 40px; left: 0; top: 37%; width: 47%;"
                                    on:click=move |_| {
                                        if is_plat_enabled("desk") { active_platform.set("desk"); }
                                    }
                                >
                                    <div class="hero-in rounded-[1.4cqw] shadow-xl cursor-pointer" style="aspect-ratio: 16/11;">
                                        <div class="flex h-full">
                                            <div class="flex w-[7cqw] flex-col items-center gap-[1.6cqw] bg-brand-600 py-[2cqw]">
                                                {[0, 1, 2, 3].into_iter().map(|i| {
                                                    let op = if i == 0 { "bg-white" } else { "bg-white/40" };
                                                    let cls = format!("h-[2.4cqw] w-[2.4cqw] rounded-[0.6cqw] {}", op);
                                                    view! { <i class=cls></i> }
                                                }).collect_view()}
                                            </div>
                                            <div class="flex-1 p-[1.8cqw]">
                                                <b class="font-hero text-slate-900 block truncate" style="font-size: 2.8cqw;">
                                                    {move || {
                                                        let b = current_biz();
                                                        let name = if is_arabic() { b.ar } else { b.en };
                                                        format!("{} {}", b.emoji, name)
                                                    }}
                                                </b>
                                                <div class="mt-[1.4cqw]">
                                                    {[0, 1, 2].into_iter().map(|i| {
                                                        let delay = format!("animation-delay: {}s", i as f64 * 0.5);
                                                        view! {
                                                            <div class="hero-row mb-[1.2cqw] flex items-center gap-[1.2cqw] rounded-[1cqw] border border-slate-200/80 p-[1.2cqw]" style=delay>
                                                                <i class="hero-shim h-[3.4cqw] w-[3.4cqw] shrink-0 rounded-[0.8cqw]"></i>
                                                                <div class="flex-1">
                                                                    <i class="hero-shim mb-[0.6cqw] block h-[0.9cqw] w-4/5 rounded"></i>
                                                                    <i class="hero-shim block h-[0.9cqw] w-1/2 rounded"></i>
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // 5. Android Smartphone Mockup
                                <div
                                    class=move || format!("hero-dv{}", if is_plat_active("and") { " on" } else { "" })
                                    data-off=move || (!is_plat_enabled("and")).then_some("")
                                    style="--z: 80px; left: 43%; top: 40%; width: 19%;"
                                    on:click=move |_| {
                                        if is_plat_enabled("and") { active_platform.set("and"); }
                                    }
                                >
                                    <div
                                        class="hero-in relative flex h-full flex-col shadow-2xl cursor-pointer"
                                        style="aspect-ratio: 9/19; border-radius: 3cqw; border-width: 0.6cqw; border-color: #0f172a; background: #ffffff;"
                                    >
                                        <i class="mx-auto mt-[1cqw] block h-[1.1cqw] w-[1.1cqw] rounded-full bg-slate-900"></i>
                                        <div class="flex items-center justify-between px-[2cqw] py-[1.1cqw] bg-brand-600 text-white">
                                            <b class="whitespace-nowrap text-[1.8cqw]">
                                                {move || {
                                                    let b = current_biz();
                                                    let name = if is_arabic() { b.ar } else { b.en };
                                                    format!("{} {}", b.emoji, name)
                                                }}
                                            </b>
                                            <i class="h-[0.9cqw] w-[3.5cqw] rounded bg-white opacity-50"></i>
                                        </div>
                                        <div class="mt-[1cqw] flex-1 px-[1.4cqw]">
                                            {[0, 1, 2].into_iter().map(|i| {
                                                let delay = format!("animation-delay: {}s", i as f64 * 0.5);
                                                view! {
                                                    <div class="hero-row mb-[1.2cqw] flex items-center gap-[1.2cqw] rounded-[1cqw] border border-slate-200/80 p-[1.2cqw]" style=delay>
                                                        <i class="hero-shim h-[3.4cqw] w-[3.4cqw] shrink-0 rounded-[0.8cqw]"></i>
                                                        <div class="flex-1">
                                                            <i class="hero-shim mb-[0.6cqw] block h-[0.9cqw] w-4/5 rounded"></i>
                                                            <i class="hero-shim block h-[0.9cqw] w-1/2 rounded"></i>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <span class="hero-fab absolute bottom-[4.5cqw] h-[4.2cqw] w-[4.2cqw] rounded-[1.3cqw] bg-brand-600 shadow-md" style="inset-inline-end: 1.6cqw;"></span>
                                        <i class="mx-auto mb-[1cqw] block h-[0.6cqw] w-[7cqw] rounded-full bg-slate-400 opacity-60"></i>
                                    </div>
                                </div>

                                // 6. iPhone Mockup
                                <div
                                    class=move || format!("hero-dv{}", if is_plat_active("ios") { " on" } else { "" })
                                    data-off=move || (!is_plat_enabled("ios")).then_some("")
                                    style="--z: 110px; left: 70%; top: 35%; width: 21%;"
                                    on:click=move |_| {
                                        if is_plat_enabled("ios") { active_platform.set("ios"); }
                                    }
                                >
                                    <div
                                        class="hero-in flex h-full flex-col shadow-2xl cursor-pointer"
                                        style="aspect-ratio: 9/19.5; border-radius: 4.2cqw; border-width: 0.7cqw; border-color: #0f172a; background: #ffffff;"
                                    >
                                        <i class="mx-auto mt-[1cqw] block h-[1.5cqw] w-[6cqw] rounded-full bg-slate-900"></i>
                                        <div class="mt-[1cqw] px-[1.6cqw]">
                                            <b class="font-hero text-slate-900 block whitespace-nowrap" style="font-size: 2.2cqw;">
                                                {move || {
                                                    let b = current_biz();
                                                    let name = if is_arabic() { b.ar } else { b.en };
                                                    format!("{} {}", b.emoji, name)
                                                }}
                                            </b>
                                        </div>
                                        <div class="mt-[1cqw] flex-1 px-[1.6cqw]">
                                            {[0, 1, 2, 3].into_iter().map(|i| {
                                                let delay = format!("animation-delay: {}s", i as f64 * 0.5);
                                                view! {
                                                    <div class="hero-row mb-[1.2cqw] flex items-center gap-[1.2cqw] rounded-[1cqw] border border-slate-200/80 p-[1.2cqw]" style=delay>
                                                        <i class="hero-shim h-[3.4cqw] w-[3.4cqw] shrink-0 rounded-[0.8cqw]"></i>
                                                        <div class="flex-1">
                                                            <i class="hero-shim mb-[0.6cqw] block h-[0.9cqw] w-4/5 rounded"></i>
                                                            <i class="hero-shim block h-[0.9cqw] w-1/2 rounded"></i>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <div class="flex justify-around border-t border-slate-200/80 py-[1.2cqw] bg-slate-50/80">
                                            {[0, 1, 2].into_iter().map(|i| {
                                                let color_cls = if i == 0 { "bg-brand-600" } else { "bg-slate-300" };
                                                let cls = format!("h-[2cqw] w-[2cqw] rounded-full {}", color_cls);
                                                view! { <i class=cls></i> }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Active Platform Caption Card below Stage
                    <div
                        class="mt-6 flex items-center gap-3.5 rounded-2xl border border-slate-200/90 bg-white/90 p-4 backdrop-blur-xl shadow-md shadow-slate-200/50 transition-all duration-300"
                        aria-live="polite"
                    >
                        <span class="text-3xl shrink-0">
                            {move || active_platform_info().emoji}
                        </span>
                        <div class="min-w-0">
                            <p class="font-hero font-bold text-slate-900 text-base leading-snug truncate">
                                {move || {
                                    let p = active_platform_info();
                                    if is_arabic() { p.ar } else { p.en }
                                }}
                            </p>
                            <p class="text-xs sm:text-sm text-slate-500 leading-snug truncate">
                                {move || {
                                    let p = active_platform_info();
                                    if is_arabic() { p.desc_ar } else { p.desc_en }
                                }}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
