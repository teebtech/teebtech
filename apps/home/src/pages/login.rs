//! # Login Page
//!
//! Matches UI reference Login.tsx:
//! Split layout: dark left panel with benefits, right panel with login form,
//! password visibility toggle, forgot password modal, and cross-portal switcher.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use design_system::i18n::Locale;
use crate::i18n::context::use_locale;
use ui::components::Icon;
use ui::i18n::LanguageSwitcher;

#[component]
pub fn LoginPage() -> impl IntoView {
    let locale_ctx = use_locale();
    let is_ar = move || locale_ctx.get() == Locale::Ar;

    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let show_password = RwSignal::new(false);
    let remember_me = RwSignal::new(false);
    let error = RwSignal::new(String::new());
    let loading = RwSignal::new(false);

    // Forgot Password modal state
    let show_forgot_modal = RwSignal::new(false);
    let forgot_email = RwSignal::new(String::new());
    let forgot_submitted = RwSignal::new(false);
    let forgot_error = RwSignal::new(String::new());

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::ev::keydown;
        leptos::prelude::window_event_listener(keydown, move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" && show_forgot_modal.get() {
                show_forgot_modal.set(false);
            }
        });
    }

    let loc = leptos_router::hooks::use_location();

    let login_action = Action::new(move |(em, pw): &(String, String)| {
        let em = em.clone();
        let pw = pw.clone();
        async move { crate::server::authentication::login(em, pw).await }
    });

    Effect::new(move |_| {
        if let Some(res) = login_action.value().get() {
            loading.set(false);
            match res {
                Ok(user) => {
                    let return_to_param = loc.query.with(|q| q.get("return_to"));
                    let safe_return = return_to_param.and_then(|rt| {
                        if rt.starts_with("http://127.0.0.1:3002")
                            || rt.starts_with("http://127.0.0.1:3004")
                            || (rt.starts_with('/') && !rt.starts_with("//"))
                        {
                            Some(rt)
                        } else {
                            None
                        }
                    });

                    #[allow(unused_variables)]
                    let target = match safe_return {
                        Some(rt) => rt,
                        None => {
                            if user.email == "admin@teebtech.com" {
                                "http://127.0.0.1:3004".to_string()
                            } else {
                                "http://127.0.0.1:3002".to_string()
                            }
                        }
                    };

                    #[cfg(feature = "hydrate")]
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href(&target);
                    }
                }
                Err(err) => {
                    if is_ar() {
                        error.set("البريد الإلكتروني أو كلمة المرور غير صحيحة. يرجى التحقق من البيانات.".to_string());
                    } else {
                        error.set(err.to_string());
                    }
                }
            }
        }
    });

    let handle_login = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        let em = email.get().trim().to_string();
        let pw = password.get().trim().to_string();

        if em.is_empty() || pw.is_empty() {
            if is_ar() {
                error.set("يرجى إدخال البريد الإلكتروني وكلمة المرور.".to_string());
            } else {
                error.set("Please enter your email and password.".to_string());
            }
            return;
        }

        if !em.contains('@') || !em.contains('.') {
            if is_ar() {
                error.set("يرجى إدخال عنوان بريد إلكتروني صحيح.".to_string());
            } else {
                error.set("Please enter a valid email address.".to_string());
            }
            return;
        }

        loading.set(true);
        error.set(String::new());
        login_action.dispatch((em, pw));
    };

    let handle_forgot_submit = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        let fe = forgot_email.get().trim().to_string();
        if fe.is_empty() || !fe.contains('@') || !fe.contains('.') {
            if is_ar() {
                forgot_error.set("يرجى إدخال عنوان بريد إلكتروني مهني صحيح.".to_string());
            } else {
                forgot_error.set("Please enter a valid work email address.".to_string());
            }
            return;
        }
        forgot_error.set(String::new());
        forgot_submitted.set(true);
    };

    view! {
        <Title text=move || if is_ar() { "تسجيل الدخول — تيب تك" } else { "Sign In — TeebTech" }/>
        <Meta name="description" content=move || if is_ar() {
            "سجّل الدخول إلى بوابة تيب تك لمتابعة المشاريع، وإدارة مقترحات العمل، والتواصل المباشر مع فريقك الهندسي المخصص."
        } else {
            "Sign in to your TeebTech portal to track projects, manage proposals, and communicate with your dedicated team."
        }/>

        <div class="min-h-screen bg-slate-50 flex">
            // Left panel (Brand & Value Proposition)
            <div class="hidden lg:flex flex-col justify-between w-1/2 hero-grid p-12 text-white bg-navy-900 border-r border-navy-800 rtl:border-r-0 rtl:border-l">
                <div class="flex items-center justify-between">
                    <a href="/" class="flex items-center gap-2.5 text-decoration-none group" aria-label="TeebTech Home">
                        <img
                            src="/images/logo.png"
                            alt="TeebTech Logo"
                            class="w-8 h-8 object-contain flex-shrink-0 drop-shadow-sm group-hover:scale-105 transition-transform"
                        />
                        <span class="font-bold font-sans text-lg tracking-tight leading-none">
                            <span class="text-white">"Teeb"</span>
                            <span class="bg-gradient-to-r from-sky-400 via-cyan-400 to-blue-500 bg-clip-text text-transparent ml-0.5">"Tech"</span>
                        </span>
                    </a>
                    <div class="bg-navy-800/80 rounded-lg p-1 border border-navy-700/60">
                        <LanguageSwitcher />
                    </div>
                </div>

                <div class="max-w-md">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-brand-500/10 border border-brand-400/20 text-brand-300 text-xs font-medium mb-6">
                        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                        {move || if is_ar() { "بوابة المشاريع المؤسسية الموحدة" } else { "Unified Enterprise Portal" }}
                    </div>
                    <h2 class="font-display text-3xl xl:text-4xl font-semibold text-white leading-tight mb-4">
                        {move || if is_ar() {
                            "مشاريعك، مقترحاتك، ومؤشرات الإنجاز — جميعها في منصة موحدة."
                        } else {
                            "Your projects, proposals, and progress — all in one place."
                        }}
                    </h2>
                    <p class="text-slate-400 text-sm leading-relaxed mb-8">
                        {move || if is_ar() {
                            "احصل على شفافية كاملة ولحظية لمراحل هندسة البرمجيات، ومؤشرات اتفاقية مستوى الخدمة، والفوترة، والتواصل المستمر مع فريقك."
                        } else {
                            "Gain real-time transparency into your software engineering milestones, SLA metrics, billing, and communication."
                        }}
                    </p>

                    <div class="space-y-4">
                        {[
                            ("checkCircle", ("Real-time project tracking and milestone visibility", "تتبع فوري لمراحل المشروع ومؤشرات الإنجاز")),
                            ("shield", ("Secure ISO 27001 document management and e-signatures", "إدارة وثائق آمنة وتوقيعات إلكترونية وفق ISO 27001")),
                            ("messageSquare", ("Direct async communication with your dedicated team", "تواصل غير متزامن مباشر مع فريقك الهندسي المخصص")),
                            ("fileText", ("Invoice history, payment status, and automated billing", "سجل الفواتير وحالات السداد وجدولة الدفعات مؤتمتة")),
                        ].iter().map(|(icon_name, (en, ar))| {
                            view! {
                                <div class="flex items-center gap-3 text-sm text-slate-300">
                                    <div class="w-6 h-6 rounded-full bg-emerald-500/10 flex items-center justify-center flex-shrink-0">
                                        <Icon name=icon_name.to_string() size=14 class="text-emerald-400 flex-shrink-0".to_string() />
                                    </div>
                                    <span>{move || if is_ar() { *ar } else { *en }}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>

                <div class="flex items-center justify-between text-xs text-slate-400 pt-6 border-t border-navy-800">
                    <p>{move || if is_ar() { "© ٢٠٢٦ شركة تيب تك المحدودة. جميع الحقوق محفوظة." } else { "© 2026 TeebTech Ltd. All rights reserved." }}</p>
                    <div class="flex gap-4">
                        <a href="/about" class="hover:text-slate-200 transition-colors">
                            {move || if is_ar() { "عن الشركة" } else { "About" }}
                        </a>
                        <a href="/contact" class="hover:text-slate-200 transition-colors">
                            {move || if is_ar() { "الدعم الفني" } else { "Support" }}
                        </a>
                    </div>
                </div>
            </div>

            // Right panel (Login Form)
            <div class="flex-1 flex flex-col justify-center items-center p-6 sm:p-10 relative">
                <div class="absolute top-6 right-6 rtl:right-auto rtl:left-6 lg:hidden">
                    <LanguageSwitcher />
                </div>

                <div class="w-full max-w-sm">
                    // Mobile logo
                    <a href="/" class="lg:hidden flex items-center gap-2.5 mb-8 text-decoration-none group" aria-label="TeebTech Home">
                        <img
                            src="/images/logo.png"
                            alt="TeebTech Logo"
                            class="w-8 h-8 object-contain flex-shrink-0 drop-shadow-sm group-hover:scale-105 transition-transform"
                        />
                        <span class="font-bold font-sans text-lg tracking-tight leading-none">
                            <span class="text-slate-900">"Teeb"</span>
                            <span class="bg-gradient-to-r from-sky-500 via-cyan-500 to-blue-600 bg-clip-text text-transparent ml-0.5">"Tech"</span>
                        </span>
                    </a>

                    <h1 class="font-display text-2xl font-semibold text-slate-900 mb-1">
                        {move || if is_ar() { "مرحبًا بك مجددًا" } else { "Welcome back" }}
                    </h1>
                    <p class="text-sm text-slate-500 mb-6">
                        {move || if is_ar() { "سجّل الدخول إلى بوابة العملاء أو فريق العمل" } else { "Sign in to your client or staff portal" }}
                    </p>

                    {move || if !error.get().is_empty() {
                        view! {
                            <div class="p-3 mb-5 rounded-lg bg-rose-50 border border-rose-200 text-rose-700 text-xs flex items-center justify-between" role="alert">
                                <div class="flex items-center gap-2">
                                    <Icon name="alertCircle".to_string() size=14 class="text-rose-500 flex-shrink-0".to_string() />
                                    <span>{error.get()}</span>
                                </div>
                                <button
                                    type="button"
                                    on:click=move |_| error.set(String::new())
                                    class="text-rose-500 hover:text-rose-700 font-bold ml-2 rtl:ml-0 rtl:mr-2 p-1"
                                    aria-label="Dismiss error"
                                >
                                    "✕"
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div class="hidden"></div> }.into_any()
                    }}

                    // Demo Credentials Helper Card
                    <div class="mb-5 p-3.5 bg-brand-50/60 border border-brand-200/80 rounded-xl text-xs space-y-2">
                        <div class="flex items-center justify-between text-brand-900 font-semibold">
                            <span class="flex items-center gap-1.5">
                                <Icon name="shield".to_string() size=14 class="text-brand-600".to_string() />
                                {move || if is_ar() { "بيانات الاختبار الجاهزة" } else { "Quick Testing Credentials" }}
                            </span>
                            <span class="text-[10px] bg-brand-100 text-brand-700 px-2 py-0.5 rounded-full font-mono">
                                "Pass: Password123!"
                            </span>
                        </div>
                        <div class="flex flex-wrap gap-2 pt-1">
                            <button
                                type="button"
                                on:click=move |_| {
                                    email.set("client@teebtech.com".to_string());
                                    password.set("Password123!".to_string());
                                }
                                class="px-2.5 py-1 bg-white border border-brand-200 rounded-md text-[11px] font-medium text-slate-800 hover:bg-brand-50 hover:border-brand-300 transition-colors cursor-pointer shadow-2xs"
                            >
                                {move || if is_ar() { "👤 عميل تجريبي (client@teebtech.com)" } else { "👤 Fill Client (client@teebtech.com)" }}
                            </button>
                            <button
                                type="button"
                                on:click=move |_| {
                                    email.set("admin@teebtech.com".to_string());
                                    password.set("Password123!".to_string());
                                }
                                class="px-2.5 py-1 bg-white border border-brand-200 rounded-md text-[11px] font-medium text-slate-800 hover:bg-brand-50 hover:border-brand-300 transition-colors cursor-pointer shadow-2xs"
                            >
                                {move || if is_ar() { "⚡ مسؤول النظام (admin@teebtech.com)" } else { "⚡ Fill Admin (admin@teebtech.com)" }}
                            </button>
                        </div>
                    </div>

                    <form on:submit=handle_login class="space-y-4" novalidate=true>
                        <div>
                            <label for="login-email" class="block text-xs font-medium text-slate-700 mb-1">
                                {move || if is_ar() { "البريد الإلكتروني *" } else { "Email Address *" }}
                            </label>
                            <input
                                id="login-email"
                                type="email"
                                autocomplete="email"
                                placeholder=move || if is_ar() { "name@company.com" } else { "you@company.com" }
                                required
                                prop:value=move || email.get()
                                class="w-full px-3.5 py-2.5 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-brand-500 bg-white transition-all shadow-sm"
                                on:input=move |e| email.set(event_target_value(&e))
                            />
                        </div>

                        <div>
                            <div class="flex items-center justify-between mb-1">
                                <label for="login-password" class="text-xs font-medium text-slate-700">
                                    {move || if is_ar() { "كلمة المرور *" } else { "Password *" }}
                                </label>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        forgot_submitted.set(false);
                                        forgot_error.set(String::new());
                                        forgot_email.set(email.get());
                                        show_forgot_modal.set(true);
                                    }
                                    class="text-xs text-brand-600 hover:text-brand-700 hover:underline font-medium cursor-pointer"
                                >
                                    {move || if is_ar() { "نسيت كلمة المرور؟" } else { "Forgot password?" }}
                                </button>
                            </div>
                            <div class="relative">
                                <input
                                    id="login-password"
                                    type=move || if show_password.get() { "text" } else { "password" }
                                    autocomplete="current-password"
                                    placeholder="••••••••"
                                    required
                                    prop:value=move || password.get()
                                    class="w-full pl-3.5 pr-10 rtl:pr-3.5 rtl:pl-10 py-2.5 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-brand-500 bg-white transition-all shadow-sm"
                                    on:input=move |e| password.set(event_target_value(&e))
                                />
                                <button
                                    type="button"
                                    on:click=move |_| show_password.update(|v| *v = !*v)
                                    class="absolute right-3 rtl:right-auto rtl:left-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 p-1 cursor-pointer"
                                    aria-label=move || if show_password.get() {
                                        if is_ar() { "إخفاء كلمة المرور" } else { "Hide password" }
                                    } else {
                                        if is_ar() { "إظهار كلمة المرور" } else { "Show password" }
                                    }
                                >
                                    {move || if show_password.get() {
                                        view! { <Icon name="eyeOff".to_string() size=15 /> }
                                    } else {
                                        view! { <Icon name="eye".to_string() size=15 /> }
                                    }}
                                </button>
                            </div>
                        </div>

                        <div class="flex items-center justify-between py-1">
                            <label class="flex items-center gap-2 cursor-pointer select-none">
                                <input
                                    type="checkbox"
                                    class="rounded border-slate-300 text-brand-600 focus:ring-brand-500 h-4 w-4"
                                    checked=remember_me
                                    on:change=move |e| remember_me.set(event_target_checked(&e))
                                />
                                <span class="text-xs text-slate-600">
                                    {move || if is_ar() { "تذكّر هذا الجهاز" } else { "Remember this device" }}
                                </span>
                            </label>
                        </div>

                        <button
                            type="submit"
                            disabled=loading
                            class="w-full py-3 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 active:bg-brand-800 transition-colors cursor-pointer shadow-sm flex items-center justify-center gap-2 disabled:opacity-70 disabled:cursor-not-allowed"
                        >
                            {move || if loading.get() {
                                view! {
                                    <span class="inline-flex items-center gap-2">
                                        <span class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></span>
                                        {if is_ar() { "جاري الدخول..." } else { "Signing In..." }}
                                    </span>
                                }.into_any()
                            } else {
                                view! { <span>{if is_ar() { "تسجيل الدخول" } else { "Sign In" }}</span> }.into_any()
                            }}
                        </button>
                    </form>

                    // Cross-portal switcher
                    <div class="mt-8 pt-6 border-t border-slate-200">
                        <p class="text-xs text-slate-500 text-center mb-3 font-medium">
                            {move || if is_ar() { "تسجيل الدخول إلى بوابة أخرى" } else { "Sign in to a different portal" }}
                        </p>
                        <div class="grid grid-cols-2 gap-2.5">
                            <a
                                href="http://127.0.0.1:3004"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="flex items-center justify-center gap-1.5 px-3 py-2.5 border border-slate-300 text-slate-700 bg-white rounded-lg text-xs font-medium hover:bg-slate-50 hover:border-slate-400 transition-colors shadow-xs"
                            >
                                <Icon name="shield".to_string() size=13 class="text-slate-500".to_string() />
                                <span>{move || if is_ar() { "بوابة الإدارة" } else { "Admin Portal" }}</span>
                            </a>
                            <a
                                href="http://127.0.0.1:3002"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="flex items-center justify-center gap-1.5 px-3 py-2.5 border border-slate-300 text-slate-700 bg-white rounded-lg text-xs font-medium hover:bg-slate-50 hover:border-slate-400 transition-colors shadow-xs"
                            >
                                <Icon name="briefcase".to_string() size=13 class="text-slate-500".to_string() />
                                <span>{move || if is_ar() { "بوابة العملاء" } else { "Client Portal" }}</span>
                            </a>
                        </div>
                    </div>

                    <p class="mt-8 text-center text-xs text-slate-500">
                        {move || if is_ar() { "لست عميلاً بعد؟ " } else { "Not a client yet? " }}
                        <a href="/quote" class="text-brand-600 hover:text-brand-700 hover:underline font-semibold">
                            {move || if is_ar() { "اطلب عرض فني وسعري" } else { "Request a proposal" }}
                        </a>
                    </p>
                </div>
            </div>

            // Forgot Password Modal
            {move || if show_forgot_modal.get() {
                view! {
                    <div
                        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-xs animate-fade-in cursor-pointer"
                        role="dialog"
                        aria-modal="true"
                        aria-labelledby="forgot-modal-title"
                        on:click=move |_| show_forgot_modal.set(false)
                    >
                        <div
                            class="bg-white rounded-2xl shadow-2xl max-w-md w-full p-6 sm:p-8 border border-slate-200 relative animate-scale-up cursor-default"
                            on:click=move |e| e.stop_propagation()
                        >
                            <button
                                type="button"
                                on:click=move |_| show_forgot_modal.set(false)
                                class="absolute top-4 right-4 rtl:right-auto rtl:left-4 text-slate-400 hover:text-slate-600 p-2 rounded-lg hover:bg-slate-100 transition-colors cursor-pointer"
                                aria-label="Close modal"
                            >
                                <Icon name="x".to_string() size=18 />
                            </button>

                            <div class="w-12 h-12 rounded-xl bg-brand-50 text-brand-600 flex items-center justify-center mb-4">
                                <Icon name="shield".to_string() size=24 />
                            </div>

                            <h3 id="forgot-modal-title" class="font-display text-xl font-semibold text-slate-900 mb-2">
                                {if is_ar() { "إعادة تعيين كلمة المرور" } else { "Reset your password" }}
                            </h3>

                            {move || if forgot_submitted.get() {
                                view! {
                                    <div class="py-2">
                                        <div class="p-4 rounded-xl bg-emerald-50 border border-emerald-200 text-emerald-800 text-sm mb-6 flex items-start gap-3">
                                            <Icon name="checkCircle".to_string() size=18 class="text-emerald-600 flex-shrink-0 mt-0.5".to_string() />
                                            <div>
                                                <p class="font-medium">
                                                    {if is_ar() { "تم إرسال رابط التعيين" } else { "Password reset link sent" }}
                                                </p>
                                                <p class="text-xs text-emerald-700 mt-1 leading-relaxed">
                                                    {if is_ar() { "إذا كان هناك حساب مسجل بعنوان " } else { "If an account exists for " }}
                                                    <strong dir="ltr">{forgot_email.get()}</strong>
                                                    {if is_ar() { "، ستتلقى تعليمات إعادة تعيين كلمة المرور خلال دقيقتين." } else { ", you will receive password reset instructions within 2 minutes." }}
                                                </p>
                                            </div>
                                        </div>
                                        <button
                                            type="button"
                                            on:click=move |_| show_forgot_modal.set(false)
                                            class="w-full py-2.5 bg-slate-900 text-white rounded-lg text-sm font-semibold hover:bg-slate-800 transition-colors"
                                        >
                                            {if is_ar() { "العودة لتسجيل الدخول" } else { "Return to Sign In" }}
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div>
                                        <p class="text-sm text-slate-500 mb-5 leading-relaxed">
                                            {if is_ar() {
                                                "أدخل بريدك الإلكتروني المهني المرتبط بحسابك في بوابة تيب تك. سنرسل لك رابطاً آمناً لإعادة تعيين كلمة المرور."
                                            } else {
                                                "Enter your corporate email address associated with your TeebTech portal account. We'll send you a secure one-time reset link."
                                            }}
                                        </p>

                                        {move || if !forgot_error.get().is_empty() {
                                            view! {
                                                <div class="p-3 mb-4 rounded-lg bg-rose-50 border border-rose-200 text-rose-700 text-xs flex items-center gap-2">
                                                    <Icon name="alertCircle".to_string() size=14 class="text-rose-500 flex-shrink-0".to_string() />
                                                    <span>{forgot_error.get()}</span>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div class="hidden"></div> }.into_any()
                                        }}

                                        <form on:submit=handle_forgot_submit class="space-y-4">
                                            <div>
                                                <label for="forgot-input-email" class="block text-xs font-medium text-slate-700 mb-1">
                                                    {if is_ar() { "البريد الإلكتروني المهني" } else { "Work Email" }}
                                                </label>
                                                <input
                                                    id="forgot-input-email"
                                                    type="email"
                                                    required
                                                    placeholder="you@company.com"
                                                    prop:value=move || forgot_email.get()
                                                    class="w-full px-3.5 py-2.5 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white"
                                                    on:input=move |e| forgot_email.set(event_target_value(&e))
                                                />
                                            </div>
                                            <div class="flex gap-3 pt-2">
                                                <button
                                                    type="button"
                                                    on:click=move |_| show_forgot_modal.set(false)
                                                    class="flex-1 py-2.5 border border-slate-300 text-slate-700 rounded-lg text-sm font-medium hover:bg-slate-50 transition-colors"
                                                >
                                                    {if is_ar() { "إلغاء" } else { "Cancel" }}
                                                </button>
                                                <button
                                                    type="submit"
                                                    class="flex-1 py-2.5 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 transition-colors shadow-sm"
                                                >
                                                    {if is_ar() { "إرسال رابط التعيين" } else { "Send Reset Link" }}
                                                </button>
                                            </div>
                                        </form>
                                    </div>
                                }.into_any()
                            }}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}
        </div>
    }
}

