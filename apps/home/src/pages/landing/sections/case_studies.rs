//! # Case Studies Section
//!
//! Matches UI reference Home.tsx Case Studies with full Arabic and English support:
//! bg-slate-50, SectionHeading, 3-column cards with Unsplash image, industry badge, result pill, tags.

use crate::data::case_studies::get_case_studies;
use crate::i18n::use_locale;
use leptos::prelude::*;

#[component]
pub fn CaseStudiesSection() -> impl IntoView {
    let locale_ctx = use_locale();
    let case_studies = get_case_studies();

    view! {
        <section id="work" class="bg-slate-50 py-16 lg:py-20">
            <div class="max-w-7xl mx-auto px-4 lg:px-8">
                <div class="flex items-end justify-between gap-4 mb-10">
                    <div>
                        <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                            {move || if locale_ctx.is_arabic() { "دراسات الحالة" } else { "Case Studies" }}
                        </p>
                        <h2 class="font-display text-3xl lg:text-4xl font-semibold text-slate-900 leading-tight">
                            {move || if locale_ctx.is_arabic() { "إنجازات تصنع فارقاً حقيقياً لعملائنا" } else { "Work That Moves the Needle" }}
                        </h2>
                    </div>
                    <a href="/case-studies" class="flex-shrink-0 hidden sm:flex inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 h-8 px-3 text-xs gap-1.5 border border-slate-300 text-slate-700 hover:bg-slate-50 bg-white">
                        <span>{move || if locale_ctx.is_arabic() { "جميع المشاريع" } else { "All Projects" }}</span>
                        <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" class="rtl:rotate-180">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                        </svg>
                    </a>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {case_studies.into_iter().map(|cs| {
                        view! {
                            <a
                                href="/case-studies"
                                class="group text-start bg-white border border-slate-200 rounded-xl overflow-hidden hover:shadow-lg hover:border-slate-300 transition-all duration-150 block"
                            >
                                <div class="relative h-44 bg-slate-100 overflow-hidden">
                                    <img
                                        src=cs.image
                                        alt=move || cs.localized_title(locale_ctx.get())
                                        class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                                        loading="lazy"
                                    />
                                    <div class="absolute top-3 start-3">
                                        <span class="inline-flex items-center gap-1 font-medium rounded-full px-2 py-0.5 text-xs bg-navy-900 text-white border border-navy-800">
                                            {move || cs.localized_industry(locale_ctx.get())}
                                        </span>
                                    </div>
                                </div>
                                <div class="p-5">
                                    <p class="text-sm font-semibold text-slate-900 leading-snug mb-2">
                                        {move || cs.localized_title(locale_ctx.get())}
                                    </p>
                                    <p class="text-xs text-emerald-700 font-medium bg-emerald-50 inline-block px-2 py-0.5 rounded mb-3">
                                        {move || cs.localized_result(locale_ctx.get())}
                                    </p>
                                    <div class="flex flex-wrap gap-1">
                                        {move || cs.localized_tags(locale_ctx.get()).iter().map(|t| {
                                            let tag_str = *t;
                                            view! {
                                                <span class="inline-flex items-center gap-1 font-medium rounded-full px-2 py-0.5 text-xs bg-slate-100 text-slate-600 border border-slate-200">
                                                    {tag_str}
                                                </span>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
