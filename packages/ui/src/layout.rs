use crate::components::Icon;
use crate::i18n::{use_locale, LanguageSwitcher};
use leptos::prelude::*;
use leptos_router::components::A;

// ─── PUBLIC LAYOUT ────────────────────────────────────────────────────────────

const SERVICE_LINKS: &[(&'static str, &'static str, &'static str, &'static str, &'static str)] = &[
    ("Software Development", "تطوير البرمجيات", "/services#software", "Custom web, mobile & enterprise apps", "تطبيقات الويب والموبايل والأنظمة المؤسسية"),
    ("Cloud & DevOps", "السحابة وهندسة العمليات", "/services#cloud", "AWS, Azure, GCP architecture & CI/CD", "معماريات AWS و Azure و GCP وخطوط CI/CD"),
    ("Cybersecurity", "الأمن السيبراني", "/services#security", "Pentesting, compliance & SOC", "اختبارات الاختراق وأطر الامتثال ومراقبة SOC"),
    ("AI & Data Engineering", "الذكاء الاصطناعي والبيانات", "/services#ai", "ML, analytics & intelligent automation", "نماذج تعلم الآلة والتحليلات والأتمتة الذكية"),
    ("IT Consulting", "الاستشارات التقنية", "/services#consulting", "Strategic technology advisory", "استشارات استراتيجية للتكنولوجيا والتحول الرقمي"),
    ("Managed IT", "خدمات الدعم المدارة", "/services#managed", "24/7 monitoring & support", "مراقبة البنية التحتية ودعم فني على مدار الساعة"),
    ("ERP & CRM", "أنظمة ERP و CRM", "/services#erp", "SAP, Salesforce, Odoo implementations", "تنفيذ وتكامل SAP و Salesforce و Odoo"),
    ("UI/UX Design", "تصميم واجهات المستخدم", "/services#design", "Research-driven design systems", "أنظمة تصميم مبنية على دراسة سلوك المستخدم"),
    ("Digital Marketing", "التسويق الرقمي", "/services#marketing", "SEO, PPC & content strategy", "تحسين محركات البحث SEO واستراتيجيات المحتوى"),
    ("QA & Testing", "ضبط واختبار الجودة", "/services#qa", "Automated & manual testing", "حزم الاختبارات الآلية واليدوية المتقدمة"),
];

/// Public marketing and platform shell layout matching reference UI.
#[component]
pub fn PublicLayout(
    #[prop(optional, into)] current_page: Option<String>,
    children: Children,
) -> impl IntoView {
    let locale_ctx = use_locale();
    let page_str = current_page.unwrap_or_else(|| "home".to_string());
    let mobile_open = RwSignal::new(false);
    let services_open = RwSignal::new(false);
    let legal_modal_tab = RwSignal::new(Option::<&'static str>::None);
    let scrolled = RwSignal::new(false);

    #[cfg(feature = "hydrate")]
    {
        let _ = window_event_listener(leptos::ev::scroll, move |_| {
            if let Some(w) = web_sys::window() {
                let current = w.scroll_y().unwrap_or(0.0);
                scrolled.set(current > 12.0);
            }
        });

        let _ = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" {
                if legal_modal_tab.get().is_some() {
                    legal_modal_tab.set(None);
                }
                if mobile_open.get() {
                    mobile_open.set(false);
                }
                if services_open.get() {
                    services_open.set(false);
                }
            }
        });
    }

    let is_current = {
        let p = page_str.clone();
        move |target: &str| p == target
    };

    view! {
        <div class="min-h-screen flex flex-col bg-slate-50 text-slate-900 antialiased font-sans">
            {/* Skip to Content Accessibility Link */}
            <a
                href="#main-content"
                class="sr-only focus:not-sr-only focus:fixed focus:top-3 focus:start-3 focus:z-50 focus:px-4 focus:py-2 focus:bg-brand-600 focus:text-white focus:rounded-md focus:shadow-lg focus:outline-none focus:ring-2 focus:ring-brand-400 font-medium text-sm"
            >
                {move || if locale_ctx.is_arabic() { "انتقل إلى المحتوى الرئيسي" } else { "Skip to main content" }}
            </a>

            {/* Top Navigation Header */}
            <header class=move || format!(
                "fixed top-0 left-0 right-0 z-40 transition-all duration-200 {}",
                if scrolled.get() {
                    "bg-white/95 backdrop-blur-sm shadow-sm border-b border-slate-200"
                } else {
                    "bg-white border-b border-slate-200"
                }
            )>
                <div class="max-w-7xl mx-auto px-4 lg:px-8">
                    <div class="flex items-center h-14 justify-between gap-6">
                        {/* Brand Logo */}
                        <a href="/" class="flex items-center gap-2.5 flex-shrink-0 text-decoration-none group" aria-label="TeebTech Homepage">
                            <img
                                src="/images/logo.png"
                                alt="TeebTech Logo"
                                width="32"
                                height="32"
                                class="w-8 h-8 object-contain flex-shrink-0 transition-transform duration-200 group-hover:scale-105 drop-shadow-sm"
                            />
                            <span class="font-bold font-sans text-lg tracking-tight leading-none">
                                <span class="text-slate-900">"Teeb"</span>
                                <span class="bg-gradient-to-r from-sky-500 via-cyan-500 to-blue-600 bg-clip-text text-transparent ms-0.5">"Tech"</span>
                            </span>
                        </a>

                        {/* Desktop Navigation Links */}
                        <nav class="hidden lg:flex items-center gap-1 flex-1" aria-label="Main Navigation">
                            {/* Services Dropdown */}
                            <div
                                class="relative"
                                on:mouseenter=move |_| services_open.set(true)
                                on:mouseleave=move |_| services_open.set(false)
                                on:focusin=move |_| services_open.set(true)
                            >
                                <a
                                    href="/services"
                                    class=format!(
                                        "flex items-center gap-1 px-3 py-1.5 text-sm rounded-md transition-colors {}",
                                        if is_current("services") { "text-brand-600 font-semibold bg-brand-50" } else { "text-slate-600 hover:text-slate-900 hover:bg-slate-50" }
                                    )
                                    aria-haspopup="true"
                                    aria-expanded=move || services_open.get().to_string()
                                >
                                    <span>{move || if locale_ctx.is_arabic() { "الخدمات" } else { "Services" }}</span>
                                    <Icon name="chevronDown".to_string() size=12 class="text-slate-400 ms-0.5".to_string() />
                                </a>

                                <div class=move || format!(
                                    "absolute top-full start-0 pt-1.5 transition-all duration-150 {}",
                                    if services_open.get() { "opacity-100 visible" } else { "opacity-0 invisible pointer-events-none" }
                                )>
                                    <div class="bg-white border border-slate-200 rounded-xl shadow-xl p-3 w-[520px] max-w-[90vw] grid grid-cols-2 gap-1">
                                        {SERVICE_LINKS.iter().copied().map(|(label_en, label_ar, href, desc_en, desc_ar)| {
                                            view! {
                                                <a
                                                    href=href
                                                    on:click=move |_| services_open.set(false)
                                                    class="text-start p-3 rounded-lg hover:bg-slate-50 transition-colors group block"
                                                >
                                                    <p class="text-sm font-medium text-slate-800 group-hover:text-brand-600">
                                                        {move || if locale_ctx.is_arabic() { label_ar } else { label_en }}
                                                    </p>
                                                    <p class="text-xs text-slate-500 mt-0.5">
                                                        {move || if locale_ctx.is_arabic() { desc_ar } else { desc_en }}
                                                    </p>
                                                </a>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            </div>

                            <a
                                href="/case-studies"
                                class=format!(
                                    "px-3 py-1.5 text-sm rounded-md transition-colors {}",
                                    if is_current("work") { "text-brand-600 font-semibold bg-brand-50" } else { "text-slate-600 hover:text-slate-900 hover:bg-slate-50" }
                                )
                            >
                                {move || if locale_ctx.is_arabic() { "الأعمال" } else { "Work" }}
                            </a>
                            <a
                                href="/about"
                                class=format!(
                                    "px-3 py-1.5 text-sm rounded-md transition-colors {}",
                                    if is_current("about") { "text-brand-600 font-semibold bg-brand-50" } else { "text-slate-600 hover:text-slate-900 hover:bg-slate-50" }
                                )
                            >
                                {move || if locale_ctx.is_arabic() { "عن تيب تك" } else { "About" }}
                            </a>
                            <a
                                href="/careers"
                                class=format!(
                                    "px-3 py-1.5 text-sm rounded-md transition-colors {}",
                                    if is_current("careers") { "text-brand-600 font-semibold bg-brand-50" } else { "text-slate-600 hover:text-slate-900 hover:bg-slate-50" }
                                )
                            >
                                {move || if locale_ctx.is_arabic() { "الوظائف" } else { "Careers" }}
                            </a>
                            <a
                                href="/contact"
                                class=format!(
                                    "px-3 py-1.5 text-sm rounded-md transition-colors {}",
                                    if is_current("contact") { "text-brand-600 font-semibold bg-brand-50" } else { "text-slate-600 hover:text-slate-900 hover:bg-slate-50" }
                                )
                            >
                                {move || if locale_ctx.is_arabic() { "تواصل معنا" } else { "Contact" }}
                            </a>
                        </nav>

                        {/* Right CTAs */}
                        <div class="hidden lg:flex items-center gap-2.5 ms-auto">
                            <LanguageSwitcher />

                            <a
                                href="/login"
                                class="inline-flex items-center justify-center font-medium rounded-md text-xs px-3.5 py-1.5 text-slate-700 hover:text-slate-900 hover:bg-slate-100 transition-colors"
                            >
                                {move || if locale_ctx.is_arabic() { "تسجيل الدخول" } else { "Sign In" }}
                            </a>
                            <a
                                href="/quote"
                                class="inline-flex items-center justify-center font-medium rounded-md text-xs px-3.5 py-1.5 bg-brand-600 text-white hover:bg-brand-700 active:bg-brand-800 shadow-sm border border-brand-700 transition-colors gap-1"
                            >
                                <span>{move || if locale_ctx.is_arabic() { "طلب عرض أسعار" } else { "Get a Quote" }}</span>
                                <Icon name="arrowRight".to_string() size=12 class="rtl:rotate-180".to_string() />
                            </a>
                        </div>

                        {/* Mobile Hamburger Button */}
                        <button
                            class="lg:hidden ms-auto p-2 text-slate-600 hover:text-slate-900 rounded-md cursor-pointer focus:outline-none focus:ring-2 focus:ring-brand-500"
                            on:click=move |_| mobile_open.update(|v| *v = !*v)
                            aria-expanded=move || mobile_open.get().to_string()
                            aria-label="Toggle navigation menu"
                        >
                            <Icon name=if mobile_open.get() { "x".to_string() } else { "menu".to_string() } size=20 />
                        </button>
                    </div>
                </div>

                {/* Mobile Navigation Drawer */}
                {move || if mobile_open.get() {
                    view! {
                        <div class="lg:hidden border-t border-slate-200 bg-white px-4 py-4 space-y-2 shadow-lg max-h-[85vh] overflow-y-auto">
                            <div class="pb-2.5 mb-2 border-b border-slate-100 flex items-center justify-between">
                                <span class="text-xs font-semibold text-slate-500">
                                    {move || if locale_ctx.is_arabic() { "اللغة" } else { "Language" }}
                                </span>
                                <LanguageSwitcher />
                            </div>

                            <div>
                                <a
                                    href="/services"
                                    on:click=move |_| mobile_open.set(false)
                                    class="block px-3 py-2 text-sm font-semibold text-slate-900 hover:bg-slate-50 rounded-md text-start"
                                >
                                    {move || if locale_ctx.is_arabic() { "الخدمات" } else { "Services" }}
                                </a>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-1 ps-3 pe-1 py-1">
                                    {SERVICE_LINKS.iter().copied().map(|(label_en, label_ar, href, _, _)| {
                                        view! {
                                            <a
                                                href=href
                                                on:click=move |_| mobile_open.set(false)
                                                class="px-2.5 py-1.5 text-xs text-slate-600 hover:text-brand-600 hover:bg-slate-50 rounded-md block transition-colors text-start"
                                            >
                                                {move || if locale_ctx.is_arabic() { label_ar } else { label_en }}
                                            </a>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                            <a
                                href="/case-studies"
                                on:click=move |_| mobile_open.set(false)
                                class="block px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md transition-colors text-start"
                            >
                                {move || if locale_ctx.is_arabic() { "الأعمال ودراسات الحالة" } else { "Work & Case Studies" }}
                            </a>
                            <a
                                href="/about"
                                on:click=move |_| mobile_open.set(false)
                                class="block px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md transition-colors text-start"
                            >
                                {move || if locale_ctx.is_arabic() { "عن تيب تك" } else { "About TeebTech" }}
                            </a>
                            <a
                                href="/careers"
                                on:click=move |_| mobile_open.set(false)
                                class="block px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md transition-colors text-start"
                            >
                                {move || if locale_ctx.is_arabic() { "الوظائف الشاغرة" } else { "Careers (Open Roles)" }}
                            </a>
                            <a
                                href="/contact"
                                on:click=move |_| mobile_open.set(false)
                                class="block px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md transition-colors text-start"
                            >
                                {move || if locale_ctx.is_arabic() { "تواصل معنا والمكاتب" } else { "Contact & Offices" }}
                            </a>
                            <div class="pt-3 border-t border-slate-100 flex gap-2">
                                <a
                                    href="/login"
                                    on:click=move |_| mobile_open.set(false)
                                    class="flex-1 text-center py-2.5 text-xs font-semibold rounded-md border border-slate-300 text-slate-700 bg-white hover:bg-slate-50"
                                >
                                    {move || if locale_ctx.is_arabic() { "تسجيل الدخول" } else { "Sign In" }}
                                </a>
                                <a
                                    href="/quote"
                                    on:click=move |_| mobile_open.set(false)
                                    class="flex-1 text-center py-2.5 text-xs font-semibold rounded-md bg-brand-600 text-white hover:bg-brand-700 shadow-sm"
                                >
                                    {move || if locale_ctx.is_arabic() { "طلب عرض أسعار" } else { "Get a Quote" }}
                                </a>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="hidden"></div> }.into_any()
                }}
            </header>

            {/* Main Application Content */}
            <main id="main-content" class="flex-1 pt-14 focus:outline-none" tabindex="-1">
                {children()}
            </main>

            {/* Comprehensive Footer */}
            <footer class="bg-navy-900 text-white">
                <div class="max-w-7xl mx-auto px-4 lg:px-8 py-14">
                    <div class="grid grid-cols-2 lg:grid-cols-6 gap-8">
                        {/* Brand Column */}
                        <div class="col-span-2">
                            <div class="flex items-center gap-2.5 mb-4">
                                <img
                                    src="/images/logo.png"
                                    alt="TeebTech Logo"
                                    width="32"
                                    height="32"
                                    class="w-8 h-8 object-contain flex-shrink-0 drop-shadow-sm"
                                />
                                <span class="font-bold font-sans text-xl tracking-tight leading-none">
                                    <span class="text-white">"Teeb"</span>
                                    <span class="bg-gradient-to-r from-sky-400 via-cyan-400 to-blue-500 bg-clip-text text-transparent ms-0.5">"Tech"</span>
                                </span>
                            </div>
                            <p class="text-sm text-slate-400 leading-relaxed max-w-xs text-start">
                                {move || if locale_ctx.is_arabic() {
                                    "هندسة حلول تقنية وبرمجيات مؤسسية تتوسع مع طموحك. موثوقون من قِبل أكثر من 200 مؤسسة وشركة عالمياً."
                                } else {
                                    "Engineering enterprise-grade technology solutions that scale with your ambition. Trusted by 200+ organizations worldwide."
                                }}
                            </p>
                            <div class="flex items-center gap-3 mt-5">
                                <a href="/" class="w-8 h-8 rounded-md bg-navy-800 flex items-center justify-center text-slate-400 hover:text-white hover:bg-navy-700 transition-colors" aria-label="TeebTech Home">
                                    <Icon name="globe".to_string() size=14 />
                                </a>
                                <a href="mailto:hello@teebtech.com" class="w-8 h-8 rounded-md bg-navy-800 flex items-center justify-center text-slate-400 hover:text-white hover:bg-navy-700 transition-colors" aria-label="Email Us">
                                    <Icon name="mail".to_string() size=14 />
                                </a>
                                <a href="/contact" class="w-8 h-8 rounded-md bg-navy-800 flex items-center justify-center text-slate-400 hover:text-white hover:bg-navy-700 transition-colors" aria-label="Contact Page">
                                    <Icon name="chat".to_string() size=14 />
                                </a>
                            </div>
                        </div>

                        {/* Services Column */}
                        <div>
                            <p class="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">
                                {move || if locale_ctx.is_arabic() { "الخدمات" } else { "Services" }}
                            </p>
                            <ul class="space-y-2 text-sm text-slate-400">
                                <li><a href="/services#software" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "تطوير البرمجيات" } else { "Software Dev" }}</a></li>
                                <li><a href="/services#cloud" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "السحابة والعمليات" } else { "Cloud & DevOps" }}</a></li>
                                <li><a href="/services#security" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "الأمن السيبراني" } else { "Cybersecurity" }}</a></li>
                                <li><a href="/services#ai" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "الذكاء الاصطناعي" } else { "AI & Data" }}</a></li>
                                <li><a href="/services#consulting" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "الاستشارات" } else { "IT Consulting" }}</a></li>
                                <li><a href="/services#managed" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "الخدمات المدارة" } else { "Managed IT" }}</a></li>
                            </ul>
                        </div>

                        {/* Industries Column */}
                        <div>
                            <p class="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">
                                {move || if locale_ctx.is_arabic() { "القطاعات" } else { "Industries" }}
                            </p>
                            <ul class="space-y-2 text-sm text-slate-400">
                                <li><a href="/case-studies" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "المالية والمصارف" } else { "Finance & Banking" }}</a></li>
                                <li><a href="/case-studies" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "الرعاية الصحية" } else { "Healthcare" }}</a></li>
                                <li><a href="/case-studies" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "التجارة الإلكترونية" } else { "Retail & E-commerce" }}</a></li>
                                <li><a href="/case-studies" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "الصناعة" } else { "Manufacturing" }}</a></li>
                                <li><a href="/case-studies" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "القطاع الحكومي" } else { "Government" }}</a></li>
                                <li><a href="/case-studies" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "التعليم" } else { "Education" }}</a></li>
                            </ul>
                        </div>

                        {/* Company Column */}
                        <div>
                            <p class="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">
                                {move || if locale_ctx.is_arabic() { "الشركة" } else { "Company" }}
                            </p>
                            <ul class="space-y-2 text-sm text-slate-400">
                                <li><a href="/about" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "من نحن" } else { "About Us" }}</a></li>
                                <li><a href="/careers" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "الوظائف" } else { "Careers" }}</a></li>
                                <li><a href="/case-studies" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "دراسات الحالة" } else { "Case Studies" }}</a></li>
                                <li><a href="/contact" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "تواصل مع المبيعات" } else { "Contact Sales" }}</a></li>
                                <li><a href="/quote" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "طلب عرض أسعار" } else { "Get a Quote" }}</a></li>
                                <li><a href="/login" class="hover:text-white transition-colors">{move || if locale_ctx.is_arabic() { "بوابة العملاء" } else { "Client Portal" }}</a></li>
                            </ul>
                        </div>

                        {/* Legal Column */}
                        <div>
                            <p class="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">
                                {move || if locale_ctx.is_arabic() { "الشؤون القانونية" } else { "Legal & Compliance" }}
                            </p>
                            <ul class="space-y-2 text-sm text-slate-400">
                                <li>
                                    <button
                                        type="button"
                                        on:click=move |_| legal_modal_tab.set(Some("privacy"))
                                        class="hover:text-white transition-colors text-start cursor-pointer"
                                    >
                                        {move || if locale_ctx.is_arabic() { "سياسة الخصوصية" } else { "Privacy Policy" }}
                                    </button>
                                </li>
                                <li>
                                    <button
                                        type="button"
                                        on:click=move |_| legal_modal_tab.set(Some("terms"))
                                        class="hover:text-white transition-colors text-start cursor-pointer"
                                    >
                                        {move || if locale_ctx.is_arabic() { "شروط الخدمة" } else { "Terms of Service" }}
                                    </button>
                                </li>
                                <li>
                                    <button
                                        type="button"
                                        on:click=move |_| legal_modal_tab.set(Some("cookies"))
                                        class="hover:text-white transition-colors text-start cursor-pointer"
                                    >
                                        {move || if locale_ctx.is_arabic() { "ملفات تعريف الارتباط" } else { "Cookie Policy" }}
                                    </button>
                                </li>
                                <li>
                                    <button
                                        type="button"
                                        on:click=move |_| legal_modal_tab.set(Some("sla"))
                                        class="hover:text-white transition-colors text-start cursor-pointer"
                                    >
                                        {move || if locale_ctx.is_arabic() { "اتفاقية مستوى الخدمة SLA" } else { "SLA Commitments" }}
                                    </button>
                                </li>
                                <li>
                                    <button
                                        type="button"
                                        on:click=move |_| legal_modal_tab.set(Some("gdpr"))
                                        class="hover:text-white transition-colors text-start cursor-pointer"
                                    >
                                        {move || if locale_ctx.is_arabic() { "الامتثال لـ GDPR" } else { "GDPR Compliance" }}
                                    </button>
                                </li>
                                <li>
                                    <button
                                        type="button"
                                        on:click=move |_| legal_modal_tab.set(Some("security"))
                                        class="hover:text-white transition-colors text-start cursor-pointer"
                                    >
                                        {move || if locale_ctx.is_arabic() { "الأمن ومعيار ISO 27001" } else { "Security & ISO 27001" }}
                                    </button>
                                </li>
                            </ul>
                        </div>
                    </div>

                    {/* Bottom Status Bar */}
                    <div class="mt-10 pt-6 border-t border-navy-800 flex flex-col sm:flex-row items-center justify-between gap-3">
                        <p class="text-xs text-slate-400">
                            {move || if locale_ctx.is_arabic() { "© 2026 شركة تيب تك المحدودة. جميع الحقوق محفوظة." } else { "© 2026 TeebTech Ltd. All rights reserved." }}
                        </p>
                        <div class="flex items-center gap-2">
                            <div class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></div>
                            <span class="text-xs text-slate-400">
                                {move || if locale_ctx.is_arabic() { "جميع الأنظمة تعمل بكفاءة • جاهزية 99.99%" } else { "All systems operational • 99.99% uptime" }}
                            </span>
                        </div>
                    </div>
                </div>
            </footer>

            {/* Interactive Legal Policy Modal */}
            {move || if let Some(tab) = legal_modal_tab.get() {
                let is_ar = locale_ctx.is_arabic();
                view! {
                    <div
                        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-navy-950/70 backdrop-blur-sm cursor-pointer"
                        role="dialog"
                        aria-modal="true"
                        aria-labelledby="legal-modal-title"
                        on:click=move |_| legal_modal_tab.set(None)
                    >
                        <div
                            class="bg-white rounded-xl shadow-2xl max-w-2xl w-full max-h-[85vh] flex flex-col overflow-hidden border border-slate-200 cursor-default text-start"
                            on:click=move |e| e.stop_propagation()
                        >
                            {/* Modal Header */}
                            <div class="flex items-center justify-between px-6 py-4 border-b border-slate-200 bg-slate-50">
                                <div class="flex items-center gap-2">
                                    <Icon name="shield".to_string() size=18 class="text-brand-600".to_string() />
                                    <h3 id="legal-modal-title" class="font-display font-semibold text-lg text-slate-900">
                                        {match (tab, is_ar) {
                                            ("privacy", true) => "سياسة الخصوصية العالمية لشركة تيب تك",
                                            ("privacy", false) => "TeebTech Global Privacy Policy",
                                            ("terms", true) => "شروط تقديم الخدمات والتعاقد",
                                            ("terms", false) => "Terms of Service & Engagement",
                                            ("cookies", true) => "سياسة ملفات تعريف الارتباط والجلسات",
                                            ("cookies", false) => "Cookie & Tracking Policy",
                                            ("sla", true) => "اتفاقية مستوى الخدمة والجاهزية (SLA)",
                                            ("sla", false) => "Service Level Agreement (SLA)",
                                            ("gdpr", true) => "الامتثال للائحة حماية البيانات العامة (GDPR)",
                                            ("gdpr", false) => "GDPR & International Data Protection",
                                            (_, true) => "معايير الأمن المؤسسي وشهادة ISO 27001",
                                            (_, false) => "Enterprise Security & ISO 27001 Posture",
                                        }}
                                    </h3>
                                </div>
                                <button
                                    on:click=move |_| legal_modal_tab.set(None)
                                    class="p-1 rounded-md text-slate-400 hover:text-slate-600 hover:bg-slate-200 transition-colors"
                                    aria-label="Close modal"
                                >
                                    <Icon name="x".to_string() size=18 />
                                </button>
                            </div>

                            {/* Modal Body */}
                            <div class="p-6 overflow-y-auto space-y-4 text-sm text-slate-600 leading-relaxed">
                                {match (tab, is_ar) {
                                    ("privacy", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"تاريخ السريان: 1 يناير 2026"</p>
                                            <p>"تلتزم شركة تيب تك المحدودة بحماية بيانات مؤسستك وبياناتك الشخصية بأعلى المعايير الأمنية. تسري هذه السياسة عبر كافة منصاتنا وتطبيقاتنا وخدماتنا الاستشارية والسحابية."</p>
                                            <p>"نحن لا نقوم مطلقاً ببيع أو تداول بيانات العملاء مع أي أطراف تجارية. وتخضع كافة قواعد البيانات والاتصالات لتشفير صارم بمعايير TLS 1.3 و AES-256."</p>
                                        </div>
                                    }.into_any(),
                                    ("privacy", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Effective Date: January 1, 2026"</p>
                                            <p>"TeebTech Ltd. ('TeebTech', 'we', 'our') is committed to protecting your organizational and personal data. This privacy policy applies across all web platforms, client portals, APIs, and cloud services."</p>
                                            <p>"We never broker, monetize, or sell client information. All databases and network transmissions adhere to TLS 1.3 and AES-256 encryption standards."</p>
                                        </div>
                                    }.into_any(),
                                    ("terms", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"اتفاقية الخدمات الهندسية والتقنية الرئيسية"</p>
                                            <p>"تعتبر كافة البرمجيات والأنظمة والمعماريات المصممة خصيصاً للعميل ملكية فكرية حصرية له فور سداد الالتزامات المالية المتفق عليها."</p>
                                            <p>"يلتزم الطرفان باتفاقيات سرية متبادلة لحماية الشيفرة المصدرية والبيانات الحساسة."</p>
                                        </div>
                                    }.into_any(),
                                    ("terms", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Master Technology Services Agreement"</p>
                                            <p>"All bespoke software engineering, system architecture, and cloud infrastructure delivered by TeebTech become the exclusive intellectual property of the client upon invoice settlement."</p>
                                        </div>
                                    }.into_any(),
                                    ("cookies", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"معايير ملفات تعريف الارتباط الضرورية"</p>
                                            <p>"نستخدم فقط ملفات تعريف الارتباط الآمنة والمشفرة والضرورية لإدارة الجلسات وحماية المستخدمين من هجمات CSRF. لا نستخدم أدوات تعقب إعلانية خارجية."</p>
                                        </div>
                                    }.into_any(),
                                    ("cookies", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Strictly Necessary Session Storage"</p>
                                            <p>"We only utilize secure, HttpOnly authentication cookies necessary to protect tenant sessions against CSRF and session hijacking. No third-party ad trackers are loaded."</p>
                                        </div>
                                    }.into_any(),
                                    ("sla", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"ضمانات اتفاقية مستوى الخدمة (SLA)"</p>
                                            <p>"نضمن جاهزية بنسبة 99.99% عبر مراكز بيانات متعددة المناطق، مع استجابة فورية للحوادث الحرجة خلال أقل من 15 دقيقة على مدار الساعة."</p>
                                        </div>
                                    }.into_any(),
                                    ("sla", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Service Level Agreement (SLA)"</p>
                                            <p>"Our SLA guarantees 99.99% multi-region uptime, sub-15-minute response times for critical incidents, and automated disaster recovery failover."</p>
                                        </div>
                                    }.into_any(),
                                    ("gdpr", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"الامتثال للأنظمة والمعايير الدولية لحماية البيانات"</p>
                                            <p>"نلتزم ببنود معالجة البيانات القياسية لحماية خصوصية المستخدمين وحق الوصول والحذف والتعديل عبر التواصل مع: data-privacy@teebtech.com."</p>
                                        </div>
                                    }.into_any(),
                                    ("gdpr", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"GDPR & Cross-Border Compliance"</p>
                                            <p>"We provide standard contractual clauses (SCCs) and robust Data Processing Addendums. Data subject requests can be submitted to data-privacy@teebtech.com."</p>
                                        </div>
                                    }.into_any(),
                                    (_, true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"ضوابط الأمن السيبراني ومعايير ISO 27001 / SOC 2"</p>
                                            <p>"عمليات معتمدة بشهادات ISO/IEC 27001:2022 و SOC 2 Type II، وتدقيق دوري مستمر للثغرات الأمنية مع تطبيق صارم لمبدأ الثقة الصفرية Zero-Trust."</p>
                                        </div>
                                    }.into_any(),
                                    (_, false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Enterprise Security & ISO 27001 Controls"</p>
                                            <p>"Certified ISO/IEC 27001:2022 and SOC 2 Type II operations, continuous SAST/DAST vulnerability scanning, and strict Zero-Trust role-based access control."</p>
                                        </div>
                                    }.into_any(),
                                }}
                            </div>

                            {/* Modal Footer */}
                            <div class="px-6 py-3 border-t border-slate-200 bg-slate-50 flex justify-end">
                                <button
                                    on:click=move |_| legal_modal_tab.set(None)
                                    class="px-4 py-2 text-xs font-semibold rounded-md bg-brand-600 text-white hover:bg-brand-700 transition-colors"
                                >
                                    {move || if locale_ctx.is_arabic() { "فهمت وموافق" } else { "I Understand" }}
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}
        </div>
    }
}

// ─── PORTAL LAYOUT ────────────────────────────────────────────────────────────

/// Portal navigation item descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortalNavItem {
    pub label: String,
    pub label_ar: String,
    pub href: String,
    pub icon: String,
    pub active: bool,
    pub badge: Option<String>,
}

/// Portal workspace shell layout (Client Portal & Admin Portal) matching reference UI.
#[component]
pub fn PortalLayout(
    #[prop(into)] portal_name: String,
    #[prop(optional, into)] user_name: Option<String>,
    #[prop(optional, into)] user_email: Option<String>,
    #[prop(optional, into)] user_role: Option<String>,
    #[prop(optional)] is_admin: Option<bool>,
    #[prop(optional, into)] nav_items: Option<Vec<PortalNavItem>>,
    #[prop(optional, into)] on_logout: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let admin_mode = is_admin.unwrap_or(false);
    let u_name = user_name.unwrap_or_else(|| "Guest User".to_string());
    let u_email = user_email.unwrap_or_else(|| "guest@teebtech.com".to_string());
    let u_role = user_role.unwrap_or_else(|| "Member".to_string());
    let mobile_open = RwSignal::new(false);

    let sidebar_bg = if admin_mode {
        "bg-navy-900 border-r border-navy-800 text-slate-400"
    } else {
        "bg-white border-r border-slate-200 text-slate-600"
    };

    let active_nav_class = if admin_mode {
        "bg-navy-800 text-white font-semibold shadow-xs"
    } else {
        "bg-brand-50 text-brand-700 border-s-2 border-brand-600 font-semibold shadow-xs"
    };

    let items = nav_items.unwrap_or_default();
    let p_name_m = portal_name.clone();
    let p_name_top = portal_name.clone();

    view! {
        <div class="min-h-screen flex bg-slate-50 text-slate-900 antialiased font-sans">
            {/* Desktop Sidebar */}
            <aside class=format!("w-64 flex-shrink-0 flex flex-col justify-between p-4 min-h-screen hidden md:flex sticky top-0 h-screen overflow-y-auto {}", sidebar_bg)>
                <div class="space-y-6">
                    {/* Portal Header */}
                    <div class="flex items-center justify-between px-2 py-1">
                        <div class="flex items-center gap-2.5">
                            <img
                                src="/images/logo.png"
                                alt="TeebTech Logo"
                                class="w-8 h-8 object-contain flex-shrink-0 drop-shadow-sm"
                            />
                            <div>
                                <span class="font-bold text-sm block font-sans tracking-tight leading-tight">
                                    <span class=if admin_mode { "text-white" } else { "text-slate-900" }>"Teeb"</span>
                                    <span class="bg-gradient-to-r from-sky-400 via-cyan-400 to-blue-500 bg-clip-text text-transparent ms-0.5">"Tech"</span>
                                </span>
                                <span class="text-[10px] text-slate-400 uppercase tracking-wider block font-mono">
                                    {portal_name.clone()}
                                </span>
                            </div>
                        </div>
                    </div>

                    {/* Navigation Items */}
                    <nav class="space-y-1">
                        <p class="px-2 text-[10px] font-semibold text-slate-400 uppercase tracking-wider mb-2">
                            {if admin_mode { "Platform Ops" } else { "Workspace Nav" }}
                        </p>
                        {if items.is_empty() {
                            view! {
                                <div class="space-y-1">
                                    <A
                                        href="/"
                                        {..}
                                        class=format!("flex items-center gap-2.5 px-3 py-2 text-xs rounded-md transition-colors {}", active_nav_class)
                                    >
                                        <Icon name="home".to_string() size=14 />
                                        <span>"Overview"</span>
                                    </A>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-1">
                                    {items.clone().into_iter().map(|item| {
                                        let cls = if item.active {
                                            active_nav_class.to_string()
                                        } else if admin_mode {
                                            "text-slate-400 hover:text-white hover:bg-navy-800 transition-colors".to_string()
                                        } else {
                                            "text-slate-600 hover:text-slate-900 hover:bg-slate-100 transition-colors".to_string()
                                        };
                                        view! {
                                            <A
                                                href=item.href
                                                {..}
                                                class=format!("flex items-center justify-between px-3 py-2 text-xs rounded-md {}", cls)
                                            >
                                                <div class="flex items-center gap-2.5">
                                                    <Icon name=item.icon size=14 />
                                                    <span>{item.label}</span>
                                                </div>
                                                {item.badge.map(|b| view! {
                                                    <span class="px-1.5 py-0.5 text-[9px] rounded-full bg-brand-600 text-white font-mono">
                                                        {b}
                                                    </span>
                                                })}
                                            </A>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }}
                    </nav>
                </div>

                {/* Sidebar User Footer */}
                <div class=format!("p-3 rounded-lg border space-y-2.5 {}", if admin_mode { "bg-navy-950/60 border-navy-800" } else { "bg-slate-50 border-slate-200" })>
                    <div class="flex items-center justify-between">
                        <div class="flex items-center gap-2 overflow-hidden">
                            <div class="w-7 h-7 rounded-full bg-brand-100 text-brand-700 flex items-center justify-center font-bold text-xs flex-shrink-0 font-mono">
                                {u_name.chars().next().unwrap_or('U').to_string()}
                            </div>
                            <div class="overflow-hidden">
                                <p class=format!("text-xs font-semibold truncate {}", if admin_mode { "text-white" } else { "text-slate-900" })>
                                    {u_name.clone()}
                                </p>
                                <p class="text-[10px] text-slate-400 truncate font-mono">
                                    {u_email.clone()}
                                </p>
                            </div>
                        </div>
                    </div>
                    {on_logout.map(|cb| view! {
                        <button
                            type="button"
                            on:click=move |_| cb.run(())
                            class="w-full flex items-center justify-center gap-1.5 px-2 py-1.5 rounded-md text-[11px] font-medium text-rose-600 hover:bg-rose-50 hover:text-rose-700 transition-colors cursor-pointer border border-rose-200/50"
                        >
                            <Icon name="logout".to_string() size=12 />
                            <span>"Sign Out"</span>
                        </button>
                    })}
                </div>
            </aside>

            {/* Mobile Drawer (if open) */}
            {move || if mobile_open.get() {
                let items_m = items.clone();
                let p_name = p_name_m.clone();
                view! {
                    <div class="fixed inset-0 z-50 flex md:hidden" role="dialog" aria-modal="true">
                        <div class="fixed inset-0 bg-slate-900/50 backdrop-blur-xs" on:click=move |_| mobile_open.set(false)></div>
                        <aside class=format!("relative w-72 max-w-xs flex flex-col justify-between p-4 min-h-screen shadow-2xl z-10 {}", sidebar_bg)>
                            <div class="space-y-6">
                                <div class="flex items-center justify-between px-2 py-1">
                                    <div class="flex items-center gap-2">
                                        <img src="/images/logo.png" alt="Logo" class="w-7 h-7 object-contain" />
                                        <span class="font-bold text-sm">{p_name}</span>
                                    </div>
                                    <button
                                        type="button"
                                        on:click=move |_| mobile_open.set(false)
                                        class="p-1 rounded text-slate-400 hover:text-slate-600"
                                    >
                                        <Icon name="x".to_string() size=18 />
                                    </button>
                                </div>
                                <nav class="space-y-1">
                                    {items_m.into_iter().map(|item| {
                                        let cls = if item.active {
                                            active_nav_class.to_string()
                                        } else {
                                            "text-slate-600 hover:text-slate-900 hover:bg-slate-100 transition-colors".to_string()
                                        };
                                        view! {
                                            <A
                                                href=item.href
                                                {..}
                                                class=format!("flex items-center justify-between px-3 py-2.5 text-xs rounded-md {}", cls)
                                                on:click=move |_| mobile_open.set(false)
                                            >
                                                <div class="flex items-center gap-2.5">
                                                    <Icon name=item.icon size=14 />
                                                    <span>{item.label}</span>
                                                </div>
                                            </A>
                                        }
                                    }).collect_view()}
                                </nav>
                            </div>
                        </aside>
                    </div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}

            {/* Main Content Area */}
            <div class="flex-1 flex flex-col min-w-0">
                {/* Topbar */}
                <header class="h-14 bg-white border-b border-slate-200 px-4 sm:px-6 flex items-center justify-between gap-4 sticky top-0 z-30">
                    <div class="flex items-center gap-3">
                        <button
                            type="button"
                            on:click=move |_| mobile_open.update(|v| *v = !*v)
                            class="md:hidden p-1.5 text-slate-500 hover:text-slate-800 rounded-md hover:bg-slate-100 cursor-pointer"
                            aria-label="Open sidebar"
                        >
                            <Icon name="menu".to_string() size=18 />
                        </button>
                        <h1 class="font-display font-semibold text-slate-900 text-sm sm:text-base tracking-tight truncate">
                            {p_name_top}
                        </h1>
                        <span class="inline-flex items-center px-2 py-0.5 rounded text-[10px] font-semibold bg-brand-50 text-brand-700 border border-brand-200 font-mono">
                            {u_role}
                        </span>
                    </div>

                    <div class="flex items-center gap-2 sm:gap-3">
                        <div class="hidden sm:flex items-center text-xs text-slate-500 font-mono bg-slate-50 border border-slate-200 px-2.5 py-1 rounded-md">
                            <span>"User: " {u_name}</span>
                        </div>
                        {on_logout.map(|cb| view! {
                            <button
                                type="button"
                                on:click=move |_| cb.run(())
                                class="hidden sm:inline-flex items-center gap-1 px-2.5 py-1 text-xs font-medium text-slate-600 hover:text-rose-600 rounded-md border border-slate-200 hover:border-rose-200 transition-colors cursor-pointer"
                            >
                                <Icon name="logout".to_string() size=12 />
                                <span>"Sign Out"</span>
                            </button>
                        })}
                    </div>
                </header>

                {/* Body Canvas */}
                <main class="flex-1 p-4 sm:p-6 lg:p-8 max-w-7xl w-full mx-auto space-y-6">
                    {children()}
                </main>
            </div>
        </div>
    }
}

/// Backward-compatible AppLayout alias for existing imports.
#[component]
pub fn AppLayout(
    #[prop(optional, into)] title: Option<String>,
    children: Children,
) -> impl IntoView {
    let current_title = title.unwrap_or_default();
    view! {
        <PublicLayout current_page=current_title>
            {children()}
        </PublicLayout>
    }
}
