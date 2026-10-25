//! # Quote Page (Request a Proposal)
//!
//! Matches UI reference Quote.tsx:
//! 4-step wizard with step indicators:
//! Step 1: Your Info
//! Step 2: Services Needed (multi-select grid with timeline and budget)
//! Step 3: Project Details (description, goals, existing stack)
//! Step 4: Review & Submit
//! Submission confirmation screen with reference ID.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use design_system::i18n::Locale;
use crate::i18n::context::use_locale;
use ui::components::Icon;
use ui::layout::PublicLayout;

struct ServiceOption {
    id: &'static str,
    name_en: &'static str,
    name_ar: &'static str,
}

const SERVICES_LIST: &[ServiceOption] = &[
    ServiceOption { id: "software", name_en: "Software Development", name_ar: "تطوير البرمجيات والنظم" },
    ServiceOption { id: "cloud", name_en: "Cloud & DevOps", name_ar: "السحابة وهندسة DevOps" },
    ServiceOption { id: "security", name_en: "Cybersecurity", name_ar: "الأمن السيبراني والامتثال" },
    ServiceOption { id: "ai", name_en: "AI & Data", name_ar: "الذكاء الاصطناعي والبيانات" },
    ServiceOption { id: "consulting", name_en: "IT Consulting", name_ar: "الاستشارات المعمارية والتقنية" },
    ServiceOption { id: "managed", name_en: "Managed IT", name_ar: "الخدمات التقنية المدارة" },
    ServiceOption { id: "erp", name_en: "ERP & CRM", name_ar: "أنظمة ERP و CRM المؤسسية" },
    ServiceOption { id: "design", name_en: "UI/UX Design", name_ar: "تصميم الواجهات وتجربة المستخدم" },
    ServiceOption { id: "marketing", name_en: "Digital Marketing", name_ar: "التسويق الرقمي والنمو" },
    ServiceOption { id: "qa", name_en: "QA & Testing", name_ar: "ضمان الجودة واختبار الأنظمة" },
];

#[component]
pub fn QuotePage() -> impl IntoView {
    let locale_ctx = use_locale();
    let is_ar = move || locale_ctx.get() == Locale::Ar;

    let step = RwSignal::new(1usize);
    let submitted = RwSignal::new(false);

    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let company = RwSignal::new(String::new());
    let role = RwSignal::new(String::new());
    let phone = RwSignal::new(String::new());
    let selected_services = RwSignal::new(Vec::<&'static str>::new());
    let timeline = RwSignal::new(String::new());
    let budget = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let goals = RwSignal::new(String::new());
    let existing = RwSignal::new(String::new());

    // Validation signals
    let name_error = RwSignal::new(Option::<(&'static str, &'static str)>::None);
    let email_error = RwSignal::new(Option::<(&'static str, &'static str)>::None);
    let services_error = RwSignal::new(Option::<(&'static str, &'static str)>::None);
    let description_error = RwSignal::new(Option::<(&'static str, &'static str)>::None);

    let toggle_service = move |svc_id: &'static str| {
        services_error.set(None);
        selected_services.update(|list| {
            if list.contains(&svc_id) {
                list.retain(|&x| x != svc_id);
            } else {
                list.push(svc_id);
            }
        });
    };

    let advance_step_1 = move || {
        let mut valid = true;
        if name.get().trim().is_empty() {
            name_error.set(Some(("Please enter your full name.", "يرجى إدخال اسمك الكامل.")));
            valid = false;
        } else {
            name_error.set(None);
        }

        let em = email.get();
        if em.trim().is_empty() {
            email_error.set(Some(("Please enter your work email.", "يرجى إدخال بريدك الإلكتروني المهني.")));
            valid = false;
        } else if !em.contains('@') || !em.contains('.') || em.len() < 5 {
            email_error.set(Some(("Please enter a valid email address (e.g. name@company.com).", "يرجى إدخال بريد إلكتروني صحيح (مثال: name@company.com).")));
            valid = false;
        } else {
            email_error.set(None);
        }

        if valid {
            step.set(2);
        }
    };

    let advance_step_2 = move || {
        if selected_services.get().is_empty() {
            services_error.set(Some(("Please select at least one service category.", "يرجى اختيار مجال خدمة واحد على الأقل.")));
        } else {
            services_error.set(None);
            step.set(3);
        }
    };

    let advance_step_3 = move || {
        if description.get().trim().len() < 10 {
            description_error.set(Some(("Please provide at least a brief 10-character description of your project.", "يرجى تقديم وصف موجز للمشروع (١٠ أحرف على الأقل).")));
        } else {
            description_error.set(None);
            step.set(4);
        }
    };

    let reset_form = move || {
        step.set(1);
        submitted.set(false);
        name.set(String::new());
        email.set(String::new());
        company.set(String::new());
        role.set(String::new());
        phone.set(String::new());
        selected_services.set(Vec::new());
        timeline.set(String::new());
        budget.set(String::new());
        description.set(String::new());
        goals.set(String::new());
        existing.set(String::new());
        name_error.set(None);
        email_error.set(None);
        services_error.set(None);
        description_error.set(None);
    };

    view! {
        <Title text=move || if is_ar() { "طلب عرض فني وسعري — تيب تك" } else { "Request a Proposal — TeebTech" }/>
        <Meta name="description" content=move || if is_ar() {
            "اطلب عرضاً هندسياً ومعمارياً مخصصاً لتطوير البرمجيات أو البنية التحتية السحابية أو الذكاء الاصطناعي أو الأمن السيبراني من تيب تك."
        } else {
            "Request a detailed engineering proposal and architecture scope for custom software development, cloud infrastructure, AI, or cybersecurity from TeebTech."
        }/>
        <PublicLayout current_page="quote".to_string()>
            <div>
                // Hero
                <section class="bg-white border-b border-slate-200 py-10 lg:py-12">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <div class="max-w-lg">
                            <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                                {move || if is_ar() { "طلب عرض فني وسعري" } else { "Request a Proposal" }}
                            </p>
                            <h1 class="font-display text-3xl sm:text-4xl font-semibold text-slate-900 mb-2">
                                {move || if is_ar() { "أخبرنا بتفاصيل مشروعك" } else { "Tell us about your project" }}
                            </h1>
                            <p class="text-slate-500 text-sm leading-relaxed">
                                {move || if is_ar() {
                                    "تحصل الطلبات المفصلة على عروض هندسية ومعمارية مخصصة تشمل النطاق وجداول التسليم — خلال ٤٨ ساعة عمل."
                                } else {
                                    "Detailed submissions receive custom, engineering-grade proposals with architectural scopes — within 48 hours."
                                }}
                            </p>
                        </div>
                    </div>
                </section>

                <section class="bg-slate-50 py-10">
                    <div class="max-w-3xl mx-auto px-4 lg:px-8">
                        // Step Indicator
                        <div class="flex items-center gap-2 sm:gap-3 mb-8" role="navigation" aria-label="Proposal Wizard Steps">
                            {[1, 2, 3, 4].iter().map(|s| {
                                let cur_s = *s;
                                let is_active = move || step.get() == cur_s;
                                view! {
                                    <div
                                        class=move || format!(
                                            "flex items-center gap-1.5 sm:gap-2 {}",
                                            if is_active() {
                                                "text-brand-600 font-semibold"
                                            } else if step.get() > cur_s {
                                                "text-emerald-600"
                                            } else {
                                                "text-slate-400"
                                            }
                                        )
                                        aria-current=move || if is_active() { "step" } else { "false" }
                                    >
                                        <div class=move || format!(
                                            "w-7 h-7 rounded-full flex items-center justify-center text-xs font-semibold {}",
                                            if is_active() {
                                                "bg-brand-600 text-white shadow-sm"
                                            } else if step.get() > cur_s {
                                                "bg-emerald-500 text-white"
                                            } else {
                                                "bg-slate-200 text-slate-500"
                                            }
                                        )>
                                            {move || if step.get() > cur_s {
                                                view! { <Icon name="check".to_string() size=12 /> }.into_any()
                                            } else {
                                                view! { <span>{cur_s}</span> }.into_any()
                                            }}
                                        </div>
                                        <span class="text-xs font-medium hidden md:block">
                                            {move || match cur_s {
                                                1 => if is_ar() { "بياناتك" } else { "Your Info" },
                                                2 => if is_ar() { "الخدمات" } else { "Services" },
                                                3 => if is_ar() { "النطاق" } else { "Scope" },
                                                _ => if is_ar() { "المراجعة" } else { "Review" },
                                            }}
                                        </span>
                                    </div>
                                    {if cur_s < 4 {
                                        view! {
                                            <div class=move || format!(
                                                "flex-1 h-px {}",
                                                if step.get() > cur_s { "bg-emerald-400" } else { "bg-slate-200" }
                                            )></div>
                                        }.into_any()
                                    } else {
                                        view! { <span class="hidden"></span> }.into_any()
                                    }}
                                }
                            }).collect_view()}
                        </div>

                        // Step Container
                        <div class="bg-white border border-slate-200 rounded-xl p-6 lg:p-8 shadow-sm">
                            {move || if submitted.get() {
                                view! {
                                    <div class="text-center py-8">
                                        <div class="w-14 h-14 bg-emerald-100 rounded-full flex items-center justify-center mx-auto mb-4">
                                            <Icon name="checkCircle".to_string() size=24 class="text-emerald-600".to_string() />
                                        </div>
                                        <span class="inline-block px-3 py-1 bg-emerald-50 text-emerald-700 text-xs font-semibold rounded-full border border-emerald-200 mb-3" dir="ltr">
                                            "Submission Confirmed • Ref: #TT-2026-PROP"
                                        </span>
                                        <h2 class="font-display text-2xl font-semibold text-slate-900 mb-2">
                                            {if is_ar() { "تم استلام طلب العرض بنجاح" } else { "Proposal Request Received" }}
                                        </h2>
                                        <p class="text-slate-600 mb-1 leading-relaxed">
                                            {if is_ar() { "شكرًا لك، " } else { "Thank you, " }}
                                            <strong class="text-slate-900">{name.get()}</strong>
                                            {if is_ar() { ". لقد تم تحويل متطلبات مشروعك إلى مكتب الحلول الهندسية." } else { ". Your project requirements have been routed to our enterprise solutions desk." }}
                                        </p>
                                        <p class="text-slate-500 text-sm mb-7 max-w-md mx-auto leading-relaxed">
                                            {if is_ar() {
                                                "سيقوم أحد كبار مهندسي النظم بدراسة المواصفات التقنية والتواصل معك على "
                                            } else {
                                                "A senior solutions architect will review your technical specifications and contact you at "
                                            }}
                                            <strong class="text-brand-700" dir="ltr">{email.get()}</strong>
                                            {if is_ar() {
                                                " خلال ٤٨ ساعة عمل مع تقدير أولي ومعماري."
                                            } else {
                                                " within 48 business hours with an initial estimation."
                                            }}
                                        </p>
                                        <div class="flex items-center justify-center gap-3 flex-wrap">
                                            <a
                                                href="/"
                                                class="inline-flex items-center justify-center font-medium rounded-md text-sm px-5 py-2.5 bg-brand-600 text-white hover:bg-brand-700 transition-colors"
                                            >
                                                {if is_ar() { "العودة للرئيسية" } else { "Return to Home" }}
                                            </a>
                                            <button
                                                type="button"
                                                on:click=move |_| reset_form()
                                                class="inline-flex items-center justify-center font-medium rounded-md text-sm px-5 py-2.5 border border-slate-300 text-slate-700 hover:bg-slate-50 transition-colors cursor-pointer"
                                            >
                                                {if is_ar() { "تقديم طلب آخر" } else { "Submit Another Request" }}
                                            </button>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                match step.get() {
                                    1 => view! {
                                        <div class="space-y-4">
                                            <div>
                                                <h2 class="text-base font-semibold text-slate-900">
                                                    {if is_ar() { "بيانات التواصل والمؤسسة" } else { "Your Information" }}
                                                </h2>
                                                <p class="text-xs text-slate-500 mt-0.5">
                                                    {if is_ar() { "يرجى التعريف بنفسك وبالمؤسسة التي تمثلها." } else { "Please introduce yourself and your organization." }}
                                                </p>
                                            </div>

                                            <div class="grid sm:grid-cols-2 gap-4">
                                                <div>
                                                    <label for="quote-name" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "الاسم الكامل *" } else { "Full Name *" }}
                                                    </label>
                                                    <input
                                                        id="quote-name"
                                                        type="text"
                                                        placeholder=if is_ar() { "مثال: مريم العلي" } else { "Jane Doe" }
                                                        prop:value=move || name.get()
                                                        class=move || format!(
                                                            "w-full px-3 py-2 text-sm border rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 {}",
                                                            if name_error.get().is_some() { "border-red-500 bg-red-50/30" } else { "border-slate-300" }
                                                        )
                                                        on:input=move |e| {
                                                            name_error.set(None);
                                                            name.set(event_target_value(&e));
                                                        }
                                                    />
                                                    {move || name_error.get().map(|(en, ar)| view! {
                                                        <p class="text-xs text-red-600 mt-1 flex items-center gap-1">
                                                            <Icon name="alertCircle".to_string() size=11 />
                                                            {if is_ar() { ar } else { en }}
                                                        </p>
                                                    })}
                                                </div>
                                                <div>
                                                    <label for="quote-email" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "البريد الإلكتروني المهني *" } else { "Work Email *" }}
                                                    </label>
                                                    <input
                                                        id="quote-email"
                                                        type="email"
                                                        placeholder=if is_ar() { "name@organization.com" } else { "jane@organization.com" }
                                                        prop:value=move || email.get()
                                                        class=move || format!(
                                                            "w-full px-3 py-2 text-sm border rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 {}",
                                                            if email_error.get().is_some() { "border-red-500 bg-red-50/30" } else { "border-slate-300" }
                                                        )
                                                        on:input=move |e| {
                                                            email_error.set(None);
                                                            email.set(event_target_value(&e));
                                                        }
                                                    />
                                                    {move || email_error.get().map(|(en, ar)| view! {
                                                        <p class="text-xs text-red-600 mt-1 flex items-center gap-1">
                                                            <Icon name="alertCircle".to_string() size=11 />
                                                            {if is_ar() { ar } else { en }}
                                                        </p>
                                                    })}
                                                </div>
                                            </div>

                                            <div class="grid sm:grid-cols-2 gap-4">
                                                <div>
                                                    <label for="quote-company" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "الشركة / المؤسسة" } else { "Company / Organization" }}
                                                    </label>
                                                    <input
                                                        id="quote-company"
                                                        type="text"
                                                        placeholder=if is_ar() { "شركة التقنية العالمية" } else { "Acme Enterprises" }
                                                        prop:value=move || company.get()
                                                        class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500"
                                                        on:input=move |e| company.set(event_target_value(&e))
                                                    />
                                                </div>
                                                <div>
                                                    <label for="quote-role" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "المسمى الوظيفي" } else { "Your Role" }}
                                                    </label>
                                                    <input
                                                        id="quote-role"
                                                        type="text"
                                                        placeholder=if is_ar() { "الرئيس التنفيذي للتقنية، مدير الهندسة..." } else { "CTO, VP Engineering, Lead..." }
                                                        prop:value=move || role.get()
                                                        class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500"
                                                        on:input=move |e| role.set(event_target_value(&e))
                                                    />
                                                </div>
                                            </div>

                                            <div>
                                                <label for="quote-phone" class="block text-xs font-medium text-slate-700 mb-1">
                                                    {if is_ar() { "رقم الهاتف (اختياري)" } else { "Phone Number (optional)" }}
                                                </label>
                                                <input
                                                    id="quote-phone"
                                                    type="tel"
                                                    placeholder="+971 4 000 0000"
                                                    dir="ltr"
                                                    prop:value=move || phone.get()
                                                    class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500"
                                                    on:input=move |e| phone.set(event_target_value(&e))
                                                />
                                            </div>

                                            <button
                                                type="button"
                                                on:click=move |_| advance_step_1()
                                                class="w-full py-3 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 transition-colors flex items-center justify-center gap-2 cursor-pointer shadow-sm mt-4"
                                            >
                                                <span>{if is_ar() { "المتابعة لاختيار الخدمات" } else { "Continue to Services" }}</span>
                                                <Icon name="arrowRight".to_string() size=14 class="rtl:rotate-180".to_string() />
                                            </button>
                                        </div>
                                    }.into_any(),

                                    2 => view! {
                                        <div>
                                            <div class="mb-4">
                                                <h2 class="text-base font-semibold text-slate-900">
                                                    {if is_ar() { "الخدمات والمجالات المطلوبة" } else { "Services Needed" }}
                                                </h2>
                                                <p class="text-xs text-slate-500 mt-0.5">
                                                    {if is_ar() { "حدد جميع القدرات والمجالات التي يتطلبها مشروعك." } else { "Select all capabilities your project requires." }}
                                                </p>
                                            </div>

                                            {move || services_error.get().map(|(en, ar)| view! {
                                                <div class="p-3 mb-4 rounded-lg bg-red-50 border border-red-200 text-red-700 text-xs flex items-center gap-2">
                                                    <Icon name="alertCircle".to_string() size=14 class="text-red-500".to_string() />
                                                    <span>{if is_ar() { ar } else { en }}</span>
                                                </div>
                                            })}

                                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 mb-6" role="group" aria-label="Services selection">
                                                {SERVICES_LIST.iter().map(|svc| {
                                                    let svc_id = svc.id;
                                                    let is_selected = move || selected_services.get().contains(&svc_id);
                                                    view! {
                                                        <button
                                                            type="button"
                                                            role="checkbox"
                                                            aria-checked=move || if is_selected() { "true" } else { "false" }
                                                            on:click=move |_| toggle_service(svc_id)
                                                            class=move || format!(
                                                                "flex items-center gap-3 p-3 rounded-lg border text-sm text-start transition-colors cursor-pointer {}",
                                                                if is_selected() {
                                                                    "border-brand-500 bg-brand-50 text-brand-700 font-medium"
                                                                } else {
                                                                    "border-slate-200 text-slate-700 hover:border-slate-300 hover:bg-slate-50"
                                                                }
                                                            )
                                                        >
                                                            <div class=move || format!(
                                                                "w-4 h-4 rounded flex-shrink-0 border flex items-center justify-center {}",
                                                                if is_selected() { "border-brand-500 bg-brand-600" } else { "border-slate-300 bg-white" }
                                                            )>
                                                                {move || if is_selected() {
                                                                    view! { <Icon name="check".to_string() size=10 class="text-white".to_string() /> }.into_any()
                                                                } else {
                                                                    view! { <span class="hidden"></span> }.into_any()
                                                                }}
                                                            </div>
                                                            <span>{move || if is_ar() { svc.name_ar } else { svc.name_en }}</span>
                                                        </button>
                                                    }
                                                }).collect_view()}
                                            </div>

                                            <div class="grid sm:grid-cols-2 gap-4 mb-6">
                                                <div>
                                                    <label for="quote-timeline" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "الجدول الزمني المستهدف" } else { "Desired Timeline" }}
                                                    </label>
                                                    <select
                                                        id="quote-timeline"
                                                        prop:value=move || timeline.get()
                                                        class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white"
                                                        on:change=move |e| timeline.set(event_target_value(&e))
                                                    >
                                                        <option value="">{if is_ar() { "اختر الجدول الزمني..." } else { "Select timeline..." }}</option>
                                                        <option value="Immediate (< 1 month)">{if is_ar() { "فوري (أقل من شهر)" } else { "Immediate (< 1 month)" }}</option>
                                                        <option value="1–3 months">{if is_ar() { "١ – ٣ أشهر" } else { "1–3 months" }}</option>
                                                        <option value="3–6 months">{if is_ar() { "٣ – ٦ أشهر" } else { "3–6 months" }}</option>
                                                        <option value="6+ months">{if is_ar() { "٦ أشهر فأكثر" } else { "6+ months" }}</option>
                                                        <option value="Ongoing Advisory">{if is_ar() { "استشارات وشراكة مستمرة" } else { "Ongoing Advisory" }}</option>
                                                    </select>
                                                </div>
                                                <div>
                                                    <label for="quote-budget" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "الميزانية التقديرية" } else { "Estimated Budget" }}
                                                    </label>
                                                    <select
                                                        id="quote-budget"
                                                        prop:value=move || budget.get()
                                                        class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white"
                                                        on:change=move |e| budget.set(event_target_value(&e))
                                                    >
                                                        <option value="">{if is_ar() { "اختر نطاق الميزانية..." } else { "Select budget range..." }}</option>
                                                        <option value="Under $25,000">{if is_ar() { "أقل من ٢٥,٠٠٠ دولار" } else { "Under $25,000" }}</option>
                                                        <option value="$25,000 – $100,000">{if is_ar() { "٢٥,٠٠٠ – ١٠٠,٠٠٠ دولار" } else { "$25,000 – $100,000" }}</option>
                                                        <option value="$100,000 – $500,000">{if is_ar() { "١٠٠,٠٠٠ – ٥٠٠,٠٠٠ دولار" } else { "$100,000 – $500,000" }}</option>
                                                        <option value="$500,000+">{if is_ar() { "أكثر من ٥٠٠,٠٠٠ دولار" } else { "$500,000+" }}</option>
                                                        <option value="Flexible / Needs Scoping">{if is_ar() { "مرنة / تتطلب دراسة النطاق" } else { "Flexible / Needs Scoping" }}</option>
                                                    </select>
                                                </div>
                                            </div>

                                            <div class="flex gap-3">
                                                <button
                                                    type="button"
                                                    on:click=move |_| step.set(1)
                                                    class="px-4 py-2 border border-slate-300 text-slate-700 rounded-lg text-sm hover:bg-slate-50 cursor-pointer"
                                                >
                                                    {if is_ar() { "السابق" } else { "Back" }}
                                                </button>
                                                <button
                                                    type="button"
                                                    on:click=move |_| advance_step_2()
                                                    class="flex-1 py-3 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 transition-colors flex items-center justify-center gap-2 cursor-pointer shadow-sm"
                                                >
                                                    <span>{if is_ar() { "المتابعة لتفاصيل النطاق" } else { "Continue to Scope" }}</span>
                                                    <Icon name="arrowRight".to_string() size=14 class="rtl:rotate-180".to_string() />
                                                </button>
                                            </div>
                                        </div>
                                    }.into_any(),

                                    3 => view! {
                                        <div class="space-y-4">
                                            <div>
                                                <h2 class="text-base font-semibold text-slate-900">
                                                    {if is_ar() { "تفاصيل المشروع والأهداف المستهدفة" } else { "Project Details & Goals" }}
                                                </h2>
                                                <p class="text-xs text-slate-500 mt-0.5">
                                                    {if is_ar() { "ساعد مهندسي الحلول لدينا في فهم سياق مشروعك وتطلعاتك." } else { "Help our solution architects understand your goals and context." }}
                                                </p>
                                            </div>

                                            <div>
                                                <label for="quote-desc" class="block text-xs font-medium text-slate-700 mb-1">
                                                    {if is_ar() { "وصف المشروع *" } else { "Project Description *" }}
                                                </label>
                                                <textarea
                                                    id="quote-desc"
                                                    rows="4"
                                                    placeholder=if is_ar() {
                                                        "ما التحدي الذي تسعى لحله؟ ما هي الأنظمة التي ترغب في بنائها أو ترقيتها أو تأمينها؟"
                                                    } else {
                                                        "What challenge are you solving? What systems need to be built, migrated, or secured?"
                                                    }
                                                    prop:value=move || description.get()
                                                    class=move || format!(
                                                        "w-full px-3 py-2 text-sm border rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 {}",
                                                        if description_error.get().is_some() { "border-red-500 bg-red-50/30" } else { "border-slate-300" }
                                                    )
                                                    on:input=move |e| {
                                                        description_error.set(None);
                                                        description.set(event_target_value(&e));
                                                    }
                                                >{move || description.get()}</textarea>
                                                {move || description_error.get().map(|(en, ar)| view! {
                                                    <p class="text-xs text-red-600 mt-1 flex items-center gap-1">
                                                        <Icon name="alertCircle".to_string() size=11 />
                                                        {if is_ar() { ar } else { en }}
                                                    </p>
                                                })}
                                            </div>

                                            <div>
                                                <label for="quote-goals" class="block text-xs font-medium text-slate-700 mb-1">
                                                    {if is_ar() { "معايير النجاح والنتائج المستهدفة" } else { "Target Outcomes / Success Criteria" }}
                                                </label>
                                                <textarea
                                                    id="quote-goals"
                                                    rows="2"
                                                    placeholder=if is_ar() {
                                                        "مثال: تقليل زمن الاستجابة بنسبة ٤٠٪، تحقيق امتثال SOC 2، التوسع لخدمة ٥٠٠ ألف مستخدم..."
                                                    } else {
                                                        "E.g. reduce latency by 40%, achieve SOC 2 compliance, scale to 500k users..."
                                                    }
                                                    prop:value=move || goals.get()
                                                    class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500"
                                                    on:input=move |e| goals.set(event_target_value(&e))
                                                >{move || goals.get()}</textarea>
                                            </div>

                                            <div>
                                                <label for="quote-existing" class="block text-xs font-medium text-slate-700 mb-1">
                                                    {if is_ar() { "البنية التحتية والتقنيات الحالية (اختياري)" } else { "Current Infrastructure & Stack (optional)" }}
                                                </label>
                                                <textarea
                                                    id="quote-existing"
                                                    rows="2"
                                                    placeholder=if is_ar() {
                                                        "مثال: AWS EKS, PostgreSQL, React, تطبيق Java قديم..."
                                                    } else {
                                                        "E.g. AWS EKS, PostgreSQL, React, legacy monolithic Java app..."
                                                    }
                                                    prop:value=move || existing.get()
                                                    class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500"
                                                    on:input=move |e| existing.set(event_target_value(&e))
                                                >{move || existing.get()}</textarea>
                                            </div>

                                            <div class="flex gap-3">
                                                <button
                                                    type="button"
                                                    on:click=move |_| step.set(2)
                                                    class="px-4 py-2 border border-slate-300 text-slate-700 rounded-lg text-sm hover:bg-slate-50 cursor-pointer"
                                                >
                                                    {if is_ar() { "السابق" } else { "Back" }}
                                                </button>
                                                <button
                                                    type="button"
                                                    on:click=move |_| advance_step_3()
                                                    class="flex-1 py-3 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 transition-colors flex items-center justify-center gap-2 cursor-pointer shadow-sm"
                                                >
                                                    <span>{if is_ar() { "مراجعة تفاصيل الطلب" } else { "Review Proposal Request" }}</span>
                                                    <Icon name="arrowRight".to_string() size=14 class="rtl:rotate-180".to_string() />
                                                </button>
                                            </div>
                                        </div>
                                    }.into_any(),

                                    _ => view! {
                                        <div class="space-y-5">
                                            <div>
                                                <h2 class="text-base font-semibold text-slate-900">
                                                    {if is_ar() { "الخطوة الرابعة: مراجعة طلبك وتأكيده" } else { "Step 4: Review Your Request" }}
                                                </h2>
                                                <p class="text-xs text-slate-500 mt-0.5">
                                                    {if is_ar() { "يرجى التحقق من صحة البيانات قبل إرسالها إلى فريقنا الهندسي." } else { "Please confirm your submission details before dispatching to our engineering desk." }}
                                                </p>
                                            </div>

                                            <div class="bg-slate-50 border border-slate-200 rounded-lg p-4 space-y-3 text-sm">
                                                <div class="grid sm:grid-cols-2 gap-2 border-b border-slate-200 pb-3">
                                                    <div>
                                                        <span class="text-xs text-slate-500">{if is_ar() { "الاسم:" } else { "Name:" }}</span>
                                                        <p class="font-medium text-slate-800">{name.get()}</p>
                                                    </div>
                                                    <div>
                                                        <span class="text-xs text-slate-500">{if is_ar() { "البريد الإلكتروني:" } else { "Work Email:" }}</span>
                                                        <p class="font-medium text-slate-800" dir="ltr">{email.get()}</p>
                                                    </div>
                                                    <div>
                                                        <span class="text-xs text-slate-500">{if is_ar() { "الشركة:" } else { "Company:" }}</span>
                                                        <p class="font-medium text-slate-800">
                                                            {if company.get().is_empty() {
                                                                if is_ar() { "غير محدد".to_string() } else { "Not specified".to_string() }
                                                            } else {
                                                                company.get()
                                                            }}
                                                        </p>
                                                    </div>
                                                    <div>
                                                        <span class="text-xs text-slate-500">{if is_ar() { "الجدول الزمني والميزانية:" } else { "Timeline & Budget:" }}</span>
                                                        <p class="font-medium text-slate-800">
                                                            {
                                                                let tl = if timeline.get().is_empty() {
                                                                    if is_ar() { "مرنة".to_string() } else { "Flexible".to_string() }
                                                                } else {
                                                                    timeline.get()
                                                                };
                                                                let bg = if budget.get().is_empty() {
                                                                    if is_ar() { "تحدد لاحقاً".to_string() } else { "To be estimated".to_string() }
                                                                } else {
                                                                    budget.get()
                                                                };
                                                                format!("{tl} • {bg}")
                                                            }
                                                        </p>
                                                    </div>
                                                </div>

                                                <div>
                                                    <span class="text-xs text-slate-500">{if is_ar() { "الخدمات المختارة:" } else { "Selected Services:" }}</span>
                                                    <div class="flex flex-wrap gap-1.5 mt-1">
                                                        {selected_services.get().into_iter().map(|svc_id| {
                                                            let svc_name = SERVICES_LIST.iter()
                                                                .find(|s| s.id == svc_id)
                                                                .map(|s| if is_ar() { s.name_ar } else { s.name_en })
                                                                .unwrap_or(svc_id);
                                                            view! {
                                                                <span class="px-2 py-0.5 bg-brand-50 text-brand-700 rounded text-xs font-medium border border-brand-200">
                                                                    {svc_name}
                                                                </span>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>

                                                <div>
                                                    <span class="text-xs text-slate-500">{if is_ar() { "ملخص المشروع:" } else { "Project Summary:" }}</span>
                                                    <p class="text-xs text-slate-700 mt-1 line-clamp-3 italic">"{description.get()}"</p>
                                                </div>
                                            </div>

                                            <div class="flex gap-3 pt-2">
                                                <button
                                                    type="button"
                                                    on:click=move |_| step.set(3)
                                                    class="px-4 py-2 border border-slate-300 text-slate-700 rounded-lg text-sm hover:bg-slate-50 cursor-pointer"
                                                >
                                                    {if is_ar() { "السابق" } else { "Back" }}
                                                </button>
                                                <button
                                                    type="button"
                                                    on:click=move |_| submitted.set(true)
                                                    class="flex-1 py-3 bg-emerald-600 text-white rounded-lg text-sm font-semibold hover:bg-emerald-700 transition-colors flex items-center justify-center gap-2 cursor-pointer shadow-sm"
                                                >
                                                    <span>{if is_ar() { "تأكيد وإرسال طلب العرض" } else { "Confirm & Submit Proposal Request" }}</span>
                                                    <Icon name="check".to_string() size=14 />
                                                </button>
                                            </div>
                                        </div>
                                    }.into_any(),
                                }
                            }}
                        </div>
                    </div>
                </section>
            </div>
        </PublicLayout>
    }
}

