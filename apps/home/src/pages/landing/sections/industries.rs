//! # Industries Section
//!
//! Matches UI reference Home.tsx Industries with full Arabic and English support:
//! White background, SectionHeading, 12 interactive industry pill tags.

use crate::i18n::use_locale;
use leptos::prelude::*;

#[component]
pub fn IndustriesSection() -> impl IntoView {
    let locale_ctx = use_locale();

    let industries = vec![
        ("Finance & Banking", "المالية والمصارف"),
        ("Healthcare", "الرعاية الصحية"),
        ("Retail & E-commerce", "التجارة الإلكترونية"),
        ("Manufacturing", "الصناعة والإنتاج"),
        ("Government", "القطاع الحكومي"),
        ("Education", "التعليم والتدريب"),
        ("Telecommunications", "الاتصالات"),
        ("Energy & Utilities", "الطاقة والمرافق"),
        ("Logistics", "الخدمات اللوجستية"),
        ("Real Estate", "القطاع العقاري"),
        ("Legal & Compliance", "الشؤون القانونية والامتثال"),
        ("Media & Entertainment", "الإعلام والترفيه"),
    ];

    view! {
        <section id="industries" class="bg-white py-16 lg:py-20">
            <div class="max-w-7xl mx-auto px-4 lg:px-8">
                <div class="mb-10">
                    <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                        {move || if locale_ctx.is_arabic() { "القطاعات" } else { "Industries" }}
                    </p>
                    <h2 class="font-display text-3xl lg:text-4xl font-semibold text-slate-900 leading-tight">
                        {move || if locale_ctx.is_arabic() { "خبرات تخصصية عبر مختلف القطاعات" } else { "Domain Expertise Across Sectors" }}
                    </h2>
                    <p class="mt-3 text-slate-500 text-base leading-relaxed max-w-2xl">
                        {move || if locale_ctx.is_arabic() {
                            "معرفة قطاعية عميقة تضمن تحولاً رقمياً استراتيجياً يتجاوز مجرد التنفيذ التقني العادي."
                        } else {
                            "Deep vertical knowledge that separates technology implementation from technology transformation."
                        }}
                    </p>
                </div>

                <div class="flex flex-wrap gap-2">
                    {industries.into_iter().map(|(en, ar)| {
                        view! {
                            <a
                                href="/case-studies"
                                class="px-3 py-1.5 border border-slate-200 rounded-full text-sm text-slate-600 hover:border-brand-300 hover:text-brand-700 hover:bg-brand-50 cursor-pointer transition-colors"
                            >
                                {move || if locale_ctx.is_arabic() { ar } else { en }}
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
