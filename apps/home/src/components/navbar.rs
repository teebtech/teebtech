//! # Navbar Component
//!
//! Matches UI reference PublicLayout.tsx Header with full bilingual (English & Arabic RTL/LTR) support:
//! White fixed header (h-14), TT logo, Services dropdown (520px mega menu),
//! Work, About, Careers, Contact links, Language Switcher, Sign In + Get a Quote CTAs,
//! and responsive mobile drawer.

use crate::i18n::{use_locale, LanguageSwitcher};
use leptos::prelude::*;

struct ServiceChild {
    label_en: &'static str,
    label_ar: &'static str,
    href: &'static str,
    desc_en: &'static str,
    desc_ar: &'static str,
}

#[component]
pub fn Navbar() -> impl IntoView {
    let locale_ctx = use_locale();
    let mobile_open = RwSignal::new(false);
    let dropdown_open = RwSignal::new(false);
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
                if mobile_open.get() {
                    mobile_open.set(false);
                }
                if dropdown_open.get() {
                    dropdown_open.set(false);
                }
            }
        });
    }

    let service_children = vec![
        ServiceChild {
            label_en: "Software Development",
            label_ar: "تطوير البرمجيات",
            href: "/services#software",
            desc_en: "Custom web, mobile & enterprise apps",
            desc_ar: "تطبيقات الويب والموبايل والأنظمة المؤسسية",
        },
        ServiceChild {
            label_en: "Cloud & DevOps",
            label_ar: "السحابة وهندسة العمليات",
            href: "/services#cloud",
            desc_en: "AWS, Azure, GCP architecture & CI/CD",
            desc_ar: "معماريات AWS و Azure و GCP وخطوط CI/CD",
        },
        ServiceChild {
            label_en: "Cybersecurity",
            label_ar: "الأمن السيبراني",
            href: "/services#security",
            desc_en: "Pentesting, compliance & SOC",
            desc_ar: "اختبارات الاختراق وأطر الامتثال ومراقبة SOC",
        },
        ServiceChild {
            label_en: "AI & Data Engineering",
            label_ar: "الذكاء الاصطناعي والبيانات",
            href: "/services#ai",
            desc_en: "ML, analytics & intelligent automation",
            desc_ar: "نماذج تعلم الآلة والتحليلات والأتمتة الذكية",
        },
        ServiceChild {
            label_en: "IT Consulting",
            label_ar: "الاستشارات التقنية",
            href: "/services#consulting",
            desc_en: "Strategic technology advisory",
            desc_ar: "استشارات استراتيجية للتكنولوجيا والتحول الرقمي",
        },
        ServiceChild {
            label_en: "Managed IT",
            label_ar: "خدمات الدعم المدارة",
            href: "/services#managed",
            desc_en: "24/7 monitoring & support",
            desc_ar: "مراقبة البنية التحتية ودعم فني على مدار الساعة",
        },
        ServiceChild {
            label_en: "ERP & CRM",
            label_ar: "أنظمة ERP و CRM",
            href: "/services#erp",
            desc_en: "SAP, Salesforce, Odoo implementations",
            desc_ar: "تنفيذ وتكامل SAP و Salesforce و Odoo",
        },
        ServiceChild {
            label_en: "UI/UX Design",
            label_ar: "تصميم واجهات المستخدم",
            href: "/services#design",
            desc_en: "Research-driven design systems",
            desc_ar: "أنظمة تصميم مبنية على دراسة سلوك المستخدم",
        },
        ServiceChild {
            label_en: "Digital Marketing",
            label_ar: "التسويق الرقمي",
            href: "/services#marketing",
            desc_en: "SEO, PPC & content strategy",
            desc_ar: "تحسين محركات البحث SEO واستراتيجيات المحتوى",
        },
        ServiceChild {
            label_en: "QA & Testing",
            label_ar: "ضبط واختبار الجودة",
            href: "/services#qa",
            desc_en: "Automated & manual testing",
            desc_ar: "حزم الاختبارات الآلية واليدوية المتقدمة",
        },
    ];

    let other_links = vec![
        ("Work", "الأعمال", "/case-studies"),
        ("About", "عن تيب تك", "/about"),
        ("Careers", "الوظائف", "/careers"),
        ("Contact", "تواصل معنا", "/contact"),
    ];

    view! {
        // Skip-to-content accessibility link
        <a
            href="#main-content"
            class="sr-only focus:not-sr-only focus:fixed focus:top-3 focus:start-3 focus:z-50 focus:px-4 focus:py-2 focus:bg-brand-600 focus:text-white focus:rounded-md focus:shadow-lg focus:outline-none focus:ring-2 focus:ring-brand-400 font-medium text-sm"
        >
            {move || if locale_ctx.is_arabic() { "انتقل إلى المحتوى الرئيسي" } else { "Skip to main content" }}
        </a>

        <header
            class=move || format!(
                "fixed top-0 left-0 right-0 z-40 transition-all duration-200 {}",
                if scrolled.get() {
                    "bg-white/95 backdrop-blur-sm shadow-sm border-b border-slate-200"
                } else {
                    "bg-white border-b border-slate-200"
                }
            )
        >
            <div class="max-w-7xl mx-auto px-4 lg:px-8">
                <div class="flex items-center h-14 gap-6">
                    // Logo
                    <a href="/" class="flex items-center gap-2.5 flex-shrink-0 group text-decoration-none" aria-label="TeebTech Homepage">
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

                    // Desktop nav
                    <nav class="hidden lg:flex items-center gap-0.5 flex-1" aria-label="Main Navigation">
                        // Services Dropdown
                        <div
                            class="relative"
                            on:mouseenter=move |_| dropdown_open.set(true)
                            on:mouseleave=move |_| dropdown_open.set(false)
                        >
                            <a
                                href="/services"
                                class="flex items-center gap-0.5 px-3 py-1.5 text-sm text-slate-600 hover:text-slate-900 rounded-md hover:bg-slate-50 transition-colors"
                                aria-haspopup="true"
                                aria-expanded=move || dropdown_open.get().to_string()
                            >
                                <span>{move || if locale_ctx.is_arabic() { "الخدمات" } else { "Services" }}</span>
                                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" class="text-slate-400 ms-0.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
                                </svg>
                            </a>

                            // Mega menu
                            <div
                                class=move || format!(
                                    "absolute top-full start-0 pt-1.5 {}",
                                    if dropdown_open.get() { "block" } else { "hidden" }
                                )
                            >
                                <div class="bg-white border border-slate-200 rounded-xl shadow-xl p-3 w-[520px] max-w-[90vw] grid grid-cols-2 gap-1">
                                    {service_children.into_iter().map(|child| {
                                        view! {
                                            <a
                                                href=child.href
                                                on:click=move |_| dropdown_open.set(false)
                                                class="text-start p-3 rounded-lg hover:bg-slate-50 transition-colors group block"
                                            >
                                                <p class="text-sm font-medium text-slate-800 group-hover:text-brand-600">
                                                    {move || if locale_ctx.is_arabic() { child.label_ar } else { child.label_en }}
                                                </p>
                                                <p class="text-xs text-slate-500 mt-0.5">
                                                    {move || if locale_ctx.is_arabic() { child.desc_ar } else { child.desc_en }}
                                                </p>
                                            </a>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </div>

                        // Other nav links
                        {other_links.into_iter().map(|(label_en, label_ar, href)| {
                            view! {
                                <a
                                    href=href
                                    class="flex items-center gap-0.5 px-3 py-1.5 text-sm text-slate-600 hover:text-slate-900 rounded-md hover:bg-slate-50 transition-colors"
                                >
                                    {move || if locale_ctx.is_arabic() { label_ar } else { label_en }}
                                </a>
                            }
                        }).collect_view()}
                    </nav>

                    // Right CTAs (Language Switcher + Sign In + Get a Quote)
                    <div class="hidden lg:flex items-center gap-2 ms-auto">
                        <LanguageSwitcher />

                        <a
                            href="/login"
                            class="inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 h-8 px-3 text-xs text-slate-600 hover:bg-slate-100 active:bg-slate-200"
                        >
                            {move || if locale_ctx.is_arabic() { "تسجيل الدخول" } else { "Sign In" }}
                        </a>
                        <a
                            href="/quote"
                            class="inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 h-8 px-3 text-xs bg-brand-600 text-white hover:bg-brand-700 active:bg-brand-800 shadow-sm border border-brand-700 gap-1"
                        >
                            <span>{move || if locale_ctx.is_arabic() { "طلب عرض أسعار" } else { "Get a Quote" }}</span>
                            <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" class="rtl:rotate-180">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                            </svg>
                        </a>
                    </div>

                    // Mobile hamburger
                    <button
                        class="lg:hidden ms-auto p-2 text-slate-600 hover:text-slate-900 cursor-pointer focus:outline-none focus:ring-2 focus:ring-brand-500 rounded-md"
                        on:click=move |_| mobile_open.update(|v| *v = !*v)
                        aria-expanded=move || mobile_open.get().to_string()
                        aria-label="Toggle navigation menu"
                    >
                        {move || if mobile_open.get() {
                            view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
                                </svg>
                            }.into_any()
                        }}
                    </button>
                </div>
            </div>

            // Mobile menu drawer
            <div
                class=move || format!(
                    "lg:hidden border-t border-slate-200 bg-white px-4 py-4 space-y-1 max-h-[80vh] overflow-y-auto {}",
                    if mobile_open.get() { "block" } else { "hidden" }
                )
            >
                <div class="pb-3 mb-2 border-b border-slate-100 flex items-center justify-between">
                    <span class="text-xs font-semibold text-slate-500">
                        {move || if locale_ctx.is_arabic() { "اللغة" } else { "Language" }}
                    </span>
                    <LanguageSwitcher />
                </div>

                <a
                    href="/services"
                    on:click=move |_| mobile_open.set(false)
                    class="w-full text-start px-3 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md block"
                >
                    {move || if locale_ctx.is_arabic() { "الخدمات الهندسية" } else { "Services" }}
                </a>
                <a
                    href="/case-studies"
                    on:click=move |_| mobile_open.set(false)
                    class="w-full text-start px-3 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md block"
                >
                    {move || if locale_ctx.is_arabic() { "الأعمال ودراسات الحالة" } else { "Work & Case Studies" }}
                </a>
                <a
                    href="/about"
                    on:click=move |_| mobile_open.set(false)
                    class="w-full text-start px-3 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md block"
                >
                    {move || if locale_ctx.is_arabic() { "عن تيب تك" } else { "About TeebTech" }}
                </a>
                <a
                    href="/careers"
                    on:click=move |_| mobile_open.set(false)
                    class="w-full text-start px-3 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md block"
                >
                    {move || if locale_ctx.is_arabic() { "الوظائف الشاغرة" } else { "Careers (Open Roles)" }}
                </a>
                <a
                    href="/contact"
                    on:click=move |_| mobile_open.set(false)
                    class="w-full text-start px-3 py-2.5 text-sm font-medium text-slate-700 hover:bg-slate-50 rounded-md block"
                >
                    {move || if locale_ctx.is_arabic() { "تواصل معنا والمكاتب" } else { "Contact & Offices" }}
                </a>
                <div class="pt-3 border-t border-slate-100 flex gap-2">
                    <a
                        href="/login"
                        on:click=move |_| mobile_open.set(false)
                        class="flex-1 inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 h-8 px-3 text-xs border border-slate-300 text-slate-700 hover:bg-slate-50 bg-white"
                    >
                        {move || if locale_ctx.is_arabic() { "تسجيل الدخول" } else { "Sign In" }}
                    </a>
                    <a
                        href="/quote"
                        on:click=move |_| mobile_open.set(false)
                        class="flex-1 inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 h-8 px-3 text-xs bg-brand-600 text-white hover:bg-brand-700 shadow-sm border border-brand-700"
                    >
                        {move || if locale_ctx.is_arabic() { "طلب عرض أسعار" } else { "Get a Quote" }}
                    </a>
                </div>
            </div>
        </header>
    }
}
