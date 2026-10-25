//! # Testimonials Section
//!
//! Matches UI reference Home.tsx Testimonials with full Arabic and English support:
//! bg-slate-50, centered SectionHeading, 3 quote cards with 5 star icons, italic quotes, avatars.

use crate::i18n::use_locale;
use leptos::prelude::*;

#[component]
pub fn TestimonialsSection() -> impl IntoView {
    let locale_ctx = use_locale();

    let testimonials = vec![
        (
            "Adaeze Okonkwo",
            "CTO, FirstNile Bank",
            "كبير مسؤولي التقنية، بنك النيل الأول",
            "TeebTech transformed our legacy infrastructure into a modern, resilient platform. Their team's depth across cloud, security, and engineering is unmatched in the region.",
            "قامت تيب تك بتحويل بنيتنا التحتية القديمة إلى منصة سحابية حديثة وفائقة المرونة. خبرة فريقهم في السحابة والأمن والهندسة لا مثيل لها في المنطقة.",
            "AO",
        ),
        (
            "James Thornberry",
            "VP Engineering, MedCore Group",
            "نائب رئيس الهندسة، مجموعة ميدكور الطبية",
            "From scoping to deployment, TeebTech operated like an extension of our internal team. The AI platform they delivered has genuinely changed how our clinicians work.",
            "من دراسة المتطلبات حتى النشر، عملت تيب تك كامتداد حقيقي لفريقنا الداخلي. منصة الذكاء الاصطناعي التي قدموها أحدثت نقلة نوعية حقيقية في كفاءة أطبائنا.",
            "JT",
        ),
        (
            "Yemi Fashola-Roberts",
            "CEO, PanAfrica Retail",
            "الرئيس التنفيذي، بان أفريكا للتجزئة",
            "We went from a fragmented tech stack to a unified commerce platform in under 9 months. TeebTech delivered on every milestone, every time.",
            "انتقلنا من منظومات برمجية متباينة إلى منصة تجارة رقمية موحدة ومتطورة خلال أقل من 9 أشهر، مع وفاء تام بكافة المخرجات في مواعيدها.",
            "YF",
        ),
    ];

    view! {
        <section id="testimonials" class="bg-slate-50 py-16 lg:py-20">
            <div class="max-w-7xl mx-auto px-4 lg:px-8">
                <div class="text-center mb-10">
                    <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                        {move || if locale_ctx.is_arabic() { "آراء الشركاء" } else { "Client Feedback" }}
                    </p>
                    <h2 class="font-display text-3xl lg:text-4xl font-semibold text-slate-900 leading-tight">
                        {move || if locale_ctx.is_arabic() { "ماذا يقول قادة التقنية عن شراكتنا" } else { "What Our Clients Say" }}
                    </h2>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {testimonials.into_iter().map(|(name, role_en, role_ar, quote_en, quote_ar, avatar)| {
                        view! {
                            <div class="bg-white border border-slate-200 rounded-xl p-6 text-start">
                                // Stars
                                <div class="flex gap-0.5 mb-3 text-amber-400">
                                    {(0..5).map(|_| view! {
                                        <svg xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="1">
                                            <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z" />
                                        </svg>
                                    }).collect_view()}
                                </div>

                                // Quote
                                <p class="text-sm text-slate-700 leading-relaxed italic mb-5">
                                    {move || if locale_ctx.is_arabic() { format!("«{}»", quote_ar) } else { format!("“{}”", quote_en) }}
                                </p>

                                // Author
                                <div class="flex items-center gap-3">
                                    <div class="w-9 h-9 rounded-full bg-navy-900 flex items-center justify-center text-white text-xs font-semibold">
                                        {avatar}
                                    </div>
                                    <div>
                                        <p class="text-sm font-semibold text-slate-900">{name}</p>
                                        <p class="text-xs text-slate-500">
                                            {move || if locale_ctx.is_arabic() { role_ar } else { role_en }}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
