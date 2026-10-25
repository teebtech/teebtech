//! # Stats Bar
//!
//! Matches UI reference Home.tsx Stats section with full Arabic and English support:
//! White background, 4-column grid, icon + value + label.

use crate::i18n::use_locale;
use leptos::prelude::*;

#[component]
pub fn StatsBar() -> impl IntoView {
    let locale_ctx = use_locale();

    let stats = vec![
        ("200+", "+200", "Clients worldwide", "عميل حول العالم", "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"),
        ("500+", "+500", "Projects delivered", "مشروع منجز بنجاح", "M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"),
        ("98%", "98%", "Client retention rate", "معدل استبقاء العملاء", "M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"),
        ("12+", "+12", "Industries served", "قطاعاً نخدمه", "M12 2a10 10 0 100 20 10 10 0 000-20zm0 0v0M2 12h0m10-10v0M22 12h0M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10 15.3 15.3 0 01-4-10A15.3 15.3 0 0112 2z"),
    ];

    view! {
        <section id="stats" class="bg-white border-b border-slate-200">
            <div class="max-w-7xl mx-auto px-4 lg:px-8 py-8">
                <div class="grid grid-cols-2 lg:grid-cols-4 gap-6">
                    {stats.into_iter().map(|(val_en, val_ar, lbl_en, lbl_ar, icon_path)| {
                        view! {
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 bg-brand-50 rounded-lg flex items-center justify-center text-brand-600 flex-shrink-0">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75">
                                        <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                                    </svg>
                                </div>
                                <div>
                                    <p class="font-display text-2xl font-semibold text-slate-900">
                                        {move || if locale_ctx.is_arabic() { val_ar } else { val_en }}
                                    </p>
                                    <p class="text-xs text-slate-500">
                                        {move || if locale_ctx.is_arabic() { lbl_ar } else { lbl_en }}
                                    </p>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
