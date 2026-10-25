//! # Process Section (How We Deliver)
//!
//! Matches UI reference Home.tsx Process with full Arabic and English support:
//! White background, SectionHeading, 5-step numbered delivery framework with connector line.

use crate::i18n::use_locale;
use leptos::prelude::*;

#[component]
pub fn ProcessSection() -> impl IntoView {
    let locale_ctx = use_locale();

    let steps = vec![
        ("01", "Discover", "الاستكشاف والتحليل", "Deep-dive stakeholder workshops, technical audits, and competitive analysis to define the real problem.", "ورش عمل معمقة مع أصحاب المصلحة، وتدقيق للبنية الحالية لتحديد المتطلبات بدقة متناهية."),
        ("02", "Plan", "التخطيط والهندسة", "Detailed scoping, architecture blueprints, resource allocation, and a transparent project roadmap.", "تحديد نطاق العمل المفصل، ومخططات البنية التحتية، وخارطة طريق زمنية واضحة وشفافة."),
        ("03", "Build", "التطوير السريع", "Agile sprints with weekly demos, continuous integration, and rigorous code review.", "دورات تطوير رشيقة (Agile) مع استعراض أسبوعي للإنجاز، ومراجعة صارمة للشيفرة البرمجية."),
        ("04", "Deploy", "النشر والتدشين", "Blue-green deployments, performance benchmarking, security hardening, and user acceptance testing.", "إطلاق آمن بنظام النشر المزدوج، واختبارات ضغط وأداء، وتحصين أمني شامل قبل التدشين."),
        ("05", "Support", "الدعم والتحسين", "SLA-backed monitoring, proactive maintenance, training, and continuous improvement.", "مراقبة مستمرة مدعومة باتفاقيات SLA، وصيانة استباقية وتطوير مستمر للأداء."),
    ];

    view! {
        <section id="process" class="bg-white py-16 lg:py-20">
            <div class="max-w-7xl mx-auto px-4 lg:px-8">
                <div class="mb-10">
                    <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                        {move || if locale_ctx.is_arabic() { "منهجية العمل" } else { "Our Process" }}
                    </p>
                    <h2 class="font-display text-3xl lg:text-4xl font-semibold text-slate-900 leading-tight">
                        {move || if locale_ctx.is_arabic() { "كيف ننجز مشاريعنا الهندسية" } else { "How We Deliver" }}
                    </h2>
                    <p class="mt-3 text-slate-500 text-base leading-relaxed max-w-2xl">
                        {move || if locale_ctx.is_arabic() {
                            "إطار عمل شفاف وموثوق أثبت نجاحه في أكثر من 500 مشروع تقني عالي الأهمية."
                        } else {
                            "A transparent, repeatable delivery framework that's served over 500 successful projects."
                        }}
                    </p>
                </div>

                <div class="relative">
                    // Connector line
                    <div class="absolute top-8 start-0 end-0 h-px bg-slate-100 hidden lg:block"></div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-6 relative z-10">
                        {steps.into_iter().map(|(num, title_en, title_ar, desc_en, desc_ar)| {
                            view! {
                                <div>
                                    <div class="w-14 h-14 bg-white border-2 border-slate-200 rounded-full flex items-center justify-center mb-4">
                                        <span class="font-mono text-sm font-semibold text-brand-600">{num}</span>
                                    </div>
                                    <p class="text-sm font-semibold text-slate-900 mb-1.5">
                                        {move || if locale_ctx.is_arabic() { title_ar } else { title_en }}
                                    </p>
                                    <p class="text-xs text-slate-500 leading-relaxed">
                                        {move || if locale_ctx.is_arabic() { desc_ar } else { desc_en }}
                                    </p>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </section>
    }
}
