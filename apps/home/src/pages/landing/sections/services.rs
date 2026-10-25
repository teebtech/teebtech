//! # Services Section
//!
//! Matches UI reference Home.tsx Services with full Arabic and English support:
//! bg-slate-50, SectionHeading, "All Services" button, 12 cards in responsive grid.

use crate::data::services::get_services;
use crate::i18n::use_locale;
use leptos::prelude::*;

/// Inline SVG icon by name — matches the reference ui.tsx Icon component paths.
fn icon_svg(name: &str) -> &'static str {
    match name {
        "code" => "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4",
        "server" => "M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01",
        "shield" => "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
        "cpu" => "M9 3v2m6-2v2M9 19v2m6-2v2M3 9h2m-2 6h2m14-6h2m-2 6h2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z",
        "briefcase" => "M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
        "settings" => "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065zM15 12a3 3 0 11-6 0 3 3 0 016 0z",
        "database" => "M12 2C7 2 3 3.34 3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5c0-1.66-4-3-9-3zM3 12c0 1.66 4 3 9 3s9-1.34 9-3",
        "layers" => "M12 2l10 6.5-10 6.5L2 8.5 12 2zM2 15.5L12 22l10-6.5M2 11.5l10 6.5 10-6.5",
        "trending" => "M13 7h8m0 0v8m0-8l-8 8-4-4-6 6",
        "activity" => "M22 12h-4l-3 9L9 3l-3 9H2",
        "globe" => "M12 2a10 10 0 100 20 10 10 0 000-20zm0 0M2 12h20M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10 15.3 15.3 0 01-4-10A15.3 15.3 0 0112 2z",
        "star" => "M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z",
        _ => "",
    }
}

#[component]
pub fn ServicesSection() -> impl IntoView {
    let locale_ctx = use_locale();
    let services = get_services();

    view! {
        <section id="services" class="bg-slate-50 py-16 lg:py-20">
            <div class="max-w-7xl mx-auto px-4 lg:px-8">
                // Header
                <div class="flex items-end justify-between gap-4 mb-10">
                    <div>
                        <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                            {move || if locale_ctx.is_arabic() { "مجالات خبراتنا" } else { "What We Do" }}
                        </p>
                        <h2 class="font-display text-3xl lg:text-4xl font-semibold text-slate-900 leading-tight">
                            {move || if locale_ctx.is_arabic() { "حلول وخدمات تقنية وهندسية شاملة" } else { "Comprehensive Technology Services" }}
                        </h2>
                        <p class="mt-3 text-slate-500 text-base leading-relaxed max-w-2xl">
                            {move || if locale_ctx.is_arabic() { "من التخطيط والاستراتيجية إلى التنفيذ والتشغيل، نغطي كل متطلبات منظومتك التقنية." } else { "From strategy to execution, we cover every layer of your technology needs." }}
                        </p>
                    </div>
                    <a href="/services" class="flex-shrink-0 hidden sm:flex inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 h-8 px-3 text-xs gap-1.5 border border-slate-300 text-slate-700 hover:bg-slate-50 bg-white">
                        <span>{move || if locale_ctx.is_arabic() { "كافة الخدمات" } else { "All Services" }}</span>
                        <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" class="rtl:rotate-180">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                        </svg>
                    </a>
                </div>

                // Service cards grid
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    {services.into_iter().map(|svc| {
                        let path = icon_svg(svc.icon);
                        let target_hash = match svc.icon {
                            "code" => "/services#software",
                            "server" => "/services#cloud",
                            "shield" => "/services#security",
                            "cpu" => "/services#ai",
                            "briefcase" => "/services#consulting",
                            "settings" => "/services#managed",
                            "database" => "/services#erp",
                            "layers" => "/services#design",
                            "trending" => "/services#marketing",
                            "activity" => "/services#qa",
                            _ => "/services",
                        };
                        view! {
                            <a href=target_hash class="group text-start bg-white border border-slate-200 rounded-lg p-5 hover:shadow-md hover:border-brand-200 transition-all duration-150 block">
                                <div class="w-8 h-8 bg-slate-100 group-hover:bg-brand-50 rounded-md flex items-center justify-center text-slate-500 group-hover:text-brand-600 transition-colors mb-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75">
                                        <path stroke-linecap="round" stroke-linejoin="round" d=path />
                                    </svg>
                                </div>
                                <p class="text-sm font-semibold text-slate-900 mb-1">
                                    {move || svc.localized_title(locale_ctx.get())}
                                </p>
                                <p class="text-xs text-slate-500 leading-relaxed">
                                    {move || svc.localized_desc(locale_ctx.get())}
                                </p>
                                <div class="mt-3 flex items-center gap-1 text-xs text-brand-600 opacity-0 group-hover:opacity-100 transition-opacity">
                                    <span>{move || if locale_ctx.is_arabic() { "اكتشف المزيد" } else { "Learn more" }}</span>
                                    <svg xmlns="http://www.w3.org/2000/svg" width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" class="rtl:rotate-180">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                                    </svg>
                                </div>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
