//! # CTA Section
//!
//! Matches UI reference Home.tsx CTA with full Arabic and English support:
//! hero-grid background, centered text, italic "Today.", proposal and sales buttons, guarantee text.

use crate::i18n::use_locale;
use leptos::prelude::*;

#[component]
pub fn CtaSection() -> impl IntoView {
    let locale_ctx = use_locale();

    view! {
        <section id="cta" class="hero-grid py-16 lg:py-20">
            <div class="max-w-7xl mx-auto px-4 lg:px-8 text-center">
                <p class="text-xs font-semibold tracking-widest text-brand-400 uppercase mb-3">
                    {move || if locale_ctx.is_arabic() { "هل أنت مستعد للانطلاق؟" } else { "Ready to start?" }}
                </p>
                <h2 class="font-display text-3xl lg:text-4xl font-semibold text-white mb-4">
                    {move || if locale_ctx.is_arabic() {
                        view! {
                            <span>"طوّر منظومتك التقنية. "</span>
                            <span class="italic text-brand-400">"اليوم."</span>
                        }.into_any()
                    } else {
                        view! {
                            <span>"Transform Your Technology. "</span>
                            <span class="italic text-brand-400">"Today."</span>
                        }.into_any()
                    }}
                </h2>
                <p class="text-slate-400 text-base max-w-xl mx-auto mb-8">
                    {move || if locale_ctx.is_arabic() {
                        "احصل على استشارة تقنية متخصصة مع مهندسينا واستلم مقترح عمل تفصيلي وعرض أسعار خلال 48 ساعة."
                    } else {
                        "Get a free consultation with a senior engineer and receive a detailed proposal within 48 hours."
                    }}
                </p>
                <div class="flex items-center justify-center gap-3 flex-wrap">
                    <a
                        href="/quote"
                        class="inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 h-11 px-6 text-sm gap-2 bg-brand-600 text-white hover:bg-brand-700 shadow-sm border border-brand-700"
                    >
                        <span>{move || if locale_ctx.is_arabic() { "اطلب عرض أسعار مخصص" } else { "Request a Proposal" }}</span>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" class="rtl:rotate-180">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                        </svg>
                    </a>
                    <a
                        href="/contact"
                        class="inline-flex items-center justify-center font-medium rounded-md transition-colors duration-100 h-11 px-6 text-sm gap-2 border border-slate-600 text-slate-300 hover:bg-white/10 hover:text-white bg-transparent"
                    >
                        {move || if locale_ctx.is_arabic() { "تواصل مع المبيعات" } else { "Contact Sales" }}
                    </a>
                </div>
                <p class="mt-5 text-xs text-slate-500">
                    {move || if locale_ctx.is_arabic() {
                        "بدون أي التزام مسبق. رد مهني موثوق خلال 24 ساعة."
                    } else {
                        "No commitment required. Response within 24 hours."
                    }}
                </p>
            </div>
        </section>
    }
}
