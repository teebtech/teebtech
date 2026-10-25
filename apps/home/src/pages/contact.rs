//! # Contact Page
//!
//! Matches UI reference Contact.tsx:
//! Interactive form with validation, service & budget selects, office cards (Lagos, London, Dubai, Toronto), 24/7 emergency card.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use design_system::i18n::Locale;
use crate::i18n::context::use_locale;
use ui::components::Icon;
use ui::layout::PublicLayout;

struct OfficeInfo {
    city_en: &'static str,
    city_ar: &'static str,
    country_en: &'static str,
    country_ar: &'static str,
    address_en: &'static str,
    address_ar: &'static str,
    phone: &'static str,
    email: &'static str,
}

const OFFICES: &[OfficeInfo] = &[
    OfficeInfo {
        city_en: "Lagos",
        city_ar: "لاغوس",
        country_en: "Nigeria",
        country_ar: "نيجيريا",
        address_en: "14 Adetokunbo Ademola St, Victoria Island",
        address_ar: "١٤ شارع أديتوكونبو أديمولا، فيكتوريا آيلاند",
        phone: "+234 1 461 9000",
        email: "lagos@teebtech.com",
    },
    OfficeInfo {
        city_en: "London",
        city_ar: "لندن",
        country_en: "United Kingdom",
        country_ar: "المملكة المتحدة",
        address_en: "20 Finsbury Street, London EC2Y 9AQ",
        address_ar: "٢٠ شارع فينسبوري، لندن EC2Y 9AQ",
        phone: "+44 20 7946 0300",
        email: "london@teebtech.com",
    },
    OfficeInfo {
        city_en: "Dubai",
        city_ar: "دبي",
        country_en: "UAE",
        country_ar: "الإمارات العربية المتحدة",
        address_en: "DIFC Gate Building, Level 12",
        address_ar: "مركز دبي المالي العالمي، مبنى البوابة، الطابق ١٢",
        phone: "+971 4 420 0800",
        email: "dubai@teebtech.com",
    },
    OfficeInfo {
        city_en: "Toronto",
        city_ar: "تورونتو",
        country_en: "Canada",
        country_ar: "كندا",
        address_en: "100 King Street West, Suite 5600",
        address_ar: "١٠٠ كينغ ستريت ويست، جناح ٥٦٠٠",
        phone: "+1 416 933 4200",
        email: "toronto@teebtech.com",
    },
];

#[component]
pub fn ContactPage() -> impl IntoView {
    let locale_ctx = use_locale();
    let is_ar = move || locale_ctx.get() == Locale::Ar;

    let submitted = RwSignal::new(false);
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let company = RwSignal::new(String::new());
    let service = RwSignal::new(String::new());
    let budget = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());

    let name_error = RwSignal::new(Option::<(&'static str, &'static str)>::None);
    let email_error = RwSignal::new(Option::<(&'static str, &'static str)>::None);
    let message_error = RwSignal::new(Option::<(&'static str, &'static str)>::None);

    let handle_submit = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
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
            email_error.set(Some(("Please enter a valid email address (e.g. name@company.com).", "يرجى إدخال عنوان بريد إلكتروني صحيح (مثال: name@company.com).")));
            valid = false;
        } else {
            email_error.set(None);
        }

        if message.get().trim().len() < 10 {
            message_error.set(Some(("Please provide at least a brief 10-character message.", "يرجى تقديم تفاصيل كافية (١٠ أحرف على الأقل).")));
            valid = false;
        } else {
            message_error.set(None);
        }

        if valid {
            submitted.set(true);
        }
    };

    let reset_form = move || {
        submitted.set(false);
        name.set(String::new());
        email.set(String::new());
        company.set(String::new());
        service.set(String::new());
        budget.set(String::new());
        message.set(String::new());
        name_error.set(None);
        email_error.set(None);
        message_error.set(None);
    };

    view! {
        <Title text=move || if is_ar() { "اتصل بنا والمكاتب العالمية — تيب تك" } else { "Contact Us & Global Offices — TeebTech" }/>
        <Meta name="description" content=move || if is_ar() {
            "تواصل مع كبار مهندسي ومعماري الحلول في تيب تك عبر مكاتبنا في دبي، لندن، لاغوس، وتورونتو لحلول البرمجيات والسحابة والأمن السيبراني."
        } else {
            "Get in touch with TeebTech senior engineering architects in Lagos, London, Dubai, and Toronto for enterprise software, cloud, and security solutions."
        }/>
        <PublicLayout current_page="contact".to_string()>
            <div>
                // Hero
                <section class="bg-white border-b border-slate-200 py-12 lg:py-14">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <div class="max-w-xl">
                            <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                                {move || if is_ar() { "تواصل معنا" } else { "Contact" }}
                            </p>
                            <h1 class="font-display text-3xl sm:text-4xl font-semibold text-slate-900 mb-3">
                                {move || if is_ar() { "تواصل مع خبرائنا" } else { "Get in touch" }}
                            </h1>
                            <p class="text-slate-500 text-sm leading-relaxed">
                                {move || if is_ar() {
                                    "أخبرنا عن مشروعك أو التحدي التقني الذي تواجهه. سيقوم مهندس حلول خبير بمراجعة طلبك والرد خلال يوم عمل واحد."
                                } else {
                                    "Tell us about your project or technical challenge. A senior solutions architect will review your inquiry and respond within one business day."
                                }}
                            </p>
                        </div>
                    </div>
                </section>

                <section class="bg-slate-50 py-12">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <div class="grid grid-cols-1 lg:grid-cols-5 gap-10">
                            // Form Column
                            <div class="lg:col-span-3">
                                <div class="bg-white border border-slate-200 rounded-xl p-6 lg:p-8 shadow-sm">
                                    <h2 class="text-base font-semibold text-slate-900 mb-1">
                                        {move || if is_ar() { "أرسل لنا رسالة" } else { "Send us a message" }}
                                    </h2>
                                    <p class="text-xs text-slate-500 mb-6">
                                        {move || if is_ar() {
                                            "يتم التعامل مع جميع الاستفسارات وفق معايير اتفاقية عدم الإفصاح المتبادلة (NDA)."
                                        } else {
                                            "All inquiries are handled under reciprocal NDA standards."
                                        }}
                                    </p>

                                    {move || if submitted.get() {
                                        view! {
                                            <div class="p-6 rounded-xl bg-emerald-50 border border-emerald-200 text-emerald-900 text-center space-y-4">
                                                <div class="w-12 h-12 bg-emerald-100 rounded-full flex items-center justify-center mx-auto text-emerald-600">
                                                    <Icon name="checkCircle".to_string() size=22 />
                                                </div>
                                                <div>
                                                    <h3 class="font-semibold text-lg text-emerald-900 mb-1">
                                                        {if is_ar() { "تم إرسال الرسالة بنجاح" } else { "Message Dispatched" }}
                                                    </h3>
                                                    <p class="text-xs text-emerald-700 max-w-md mx-auto leading-relaxed">
                                                        {if is_ar() { "شكرًا لتواصلك مع تيب تك، " } else { "Thank you for contacting TeebTech, " }}
                                                        <strong class="text-emerald-900">{name.get()}</strong>
                                                        {if is_ar() { ". لقد أرسلنا إشعار تأكيد آلي إلى " } else { ". We have sent an automated confirmation receipt to " }}
                                                        <strong class="text-emerald-900">{email.get()}</strong>
                                                        {if is_ar() { " وسيتواصل معك فريقنا الهندسي خلال ٢٤ ساعة." } else { " and our engineering desk will reply within 24 hours." }}
                                                    </p>
                                                </div>
                                                <div class="pt-2">
                                                    <button
                                                        type="button"
                                                        on:click=move |_| reset_form()
                                                        class="px-4 py-2 bg-emerald-700 text-white rounded-lg text-xs font-semibold hover:bg-emerald-800 transition-colors shadow-sm cursor-pointer"
                                                    >
                                                        {if is_ar() { "إرسال رسالة أخرى" } else { "Send Another Message" }}
                                                    </button>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <form on:submit=handle_submit class="space-y-4">
                                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                                    <div>
                                                        <label for="contact-name" class="block text-xs font-medium text-slate-700 mb-1">
                                                            {if is_ar() { "الاسم الكامل *" } else { "Full Name *" }}
                                                        </label>
                                                        <input
                                                            id="contact-name"
                                                            type="text"
                                                            placeholder=if is_ar() { "مثال: عبد الله الأحمد" } else { "Adaeze Okonkwo" }
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
                                                        <label for="contact-email" class="block text-xs font-medium text-slate-700 mb-1">
                                                            {if is_ar() { "البريد الإلكتروني المهني *" } else { "Work Email *" }}
                                                        </label>
                                                        <input
                                                            id="contact-email"
                                                            type="email"
                                                            placeholder=if is_ar() { "name@company.com" } else { "adaeze@company.com" }
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

                                                <div>
                                                    <label for="contact-company" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "الشركة / المؤسسة" } else { "Company / Organization" }}
                                                    </label>
                                                    <input
                                                        id="contact-company"
                                                        type="text"
                                                        placeholder=if is_ar() { "اسم مؤسستك" } else { "Your organization" }
                                                        prop:value=move || company.get()
                                                        class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500"
                                                        on:input=move |e| company.set(event_target_value(&e))
                                                    />
                                                </div>

                                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                                    <div>
                                                        <label for="contact-service" class="block text-xs font-medium text-slate-700 mb-1">
                                                            {if is_ar() { "مجال الخدمة المطلوب" } else { "Service of Interest" }}
                                                        </label>
                                                        <select
                                                            id="contact-service"
                                                            prop:value=move || service.get()
                                                            class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white"
                                                            on:change=move |e| service.set(event_target_value(&e))
                                                        >
                                                            <option value="">{if is_ar() { "اختر الخدمة المطلوبة..." } else { "Select a service..." }}</option>
                                                            <option value="software">{if is_ar() { "تطوير البرمجيات المخصصة" } else { "Software Development" }}</option>
                                                            <option value="cloud">{if is_ar() { "السحابة وهندسة DevOps" } else { "Cloud & DevOps" }}</option>
                                                            <option value="security">{if is_ar() { "الأمن السيبراني وحماية النظم" } else { "Cybersecurity" }}</option>
                                                            <option value="ai">{if is_ar() { "الذكاء الاصطناعي وهندسة البيانات" } else { "AI & Data Engineering" }}</option>
                                                            <option value="consulting">{if is_ar() { "الاستشارات التقنية والمعمارية" } else { "IT Consulting" }}</option>
                                                            <option value="managed">{if is_ar() { "خدمات تكنولوجيا المعلومات المدارة" } else { "Managed IT Services" }}</option>
                                                            <option value="erp">{if is_ar() { "أنظمة ERP و CRM المؤسسية" } else { "ERP & CRM Solutions" }}</option>
                                                            <option value="design">{if is_ar() { "تصميم الواجهات وتجربة المستخدم (UI/UX)" } else { "UI/UX Design" }}</option>
                                                            <option value="marketing">{if is_ar() { "التسويق الرقمي المعتمد على البيانات" } else { "Digital Marketing" }}</option>
                                                            <option value="qa">{if is_ar() { "ضمان الجودة واختبار البرمجيات" } else { "QA & Testing" }}</option>
                                                            <option value="other">{if is_ar() { "هندسة معمارية مخصصة / أخرى" } else { "Other / Custom Architecture" }}</option>
                                                        </select>
                                                    </div>

                                                    <div>
                                                        <label for="contact-budget" class="block text-xs font-medium text-slate-700 mb-1">
                                                            {if is_ar() { "الميزانية التقديرية" } else { "Estimated Budget" }}
                                                        </label>
                                                        <select
                                                            id="contact-budget"
                                                            prop:value=move || budget.get()
                                                            class="w-full px-3 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white"
                                                            on:change=move |e| budget.set(event_target_value(&e))
                                                        >
                                                            <option value="">{if is_ar() { "اختر نطاق الميزانية..." } else { "Select a budget range..." }}</option>
                                                            <option value="<25k">{if is_ar() { "أقل من ٢٥,٠٠٠ دولار" } else { "Under $25,000" }}</option>
                                                            <option value="25-100k">{if is_ar() { "٢٥,٠٠٠ – ١٠٠,٠٠٠ دولار" } else { "$25,000 – $100,000" }}</option>
                                                            <option value="100-500k">{if is_ar() { "١٠٠,٠٠٠ – ٥٠٠,٠٠٠ دولار" } else { "$100,000 – $500,000" }}</option>
                                                            <option value="500k+">{if is_ar() { "أكثر من ٥٠٠,٠٠٠ دولار" } else { "$500,000+" }}</option>
                                                            <option value="unknown">{if is_ar() { "مرنة / تتطلب تحديد النطاق أولاً" } else { "Flexible / Needs Scoping" }}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div>
                                                    <label for="contact-message" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "وصف المشروع أو التحدي *" } else { "Message / Challenge Description *" }}
                                                    </label>
                                                    <textarea
                                                        id="contact-message"
                                                        rows="5"
                                                        placeholder=if is_ar() {
                                                            "صف أهدافك، والبيئة التقنية الحالية، أو استفساراتك. كيف يبدو النجاح بالنسبة لهذه المبادرة؟"
                                                        } else {
                                                            "Describe your goals, tech stack, or questions. What does success look like for this initiative?"
                                                        }
                                                        prop:value=move || message.get()
                                                        class=move || format!(
                                                            "w-full px-3 py-2 text-sm border rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 {}",
                                                            if message_error.get().is_some() { "border-red-500 bg-red-50/30" } else { "border-slate-300" }
                                                        )
                                                        on:input=move |e| {
                                                            message_error.set(None);
                                                            message.set(event_target_value(&e));
                                                        }
                                                    ></textarea>
                                                    {move || message_error.get().map(|(en, ar)| view! {
                                                        <p class="text-xs text-red-600 mt-1 flex items-center gap-1">
                                                            <Icon name="alertCircle".to_string() size=11 />
                                                            {if is_ar() { ar } else { en }}
                                                        </p>
                                                    })}
                                                </div>

                                                <button
                                                    type="submit"
                                                    class="w-full py-3 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 transition-colors flex items-center justify-center gap-2 cursor-pointer shadow-sm"
                                                >
                                                    <span>{if is_ar() { "إرسال الرسالة" } else { "Send Message" }}</span>
                                                    <Icon name="arrowRight".to_string() size=15 class="rtl:rotate-180".to_string() />
                                                </button>
                                                <p class="text-xs text-slate-400 text-center">
                                                    {if is_ar() {
                                                        "بالإرسال، فإنك توافق على معايير السرية والخصوصية لدينا. لن يتم مشاركة بياناتك مطلقاً مع أطراف ثالثة."
                                                    } else {
                                                        "By submitting, you agree to our confidential handling standards. Never shared with third parties."
                                                    }}
                                                </p>
                                            </form>
                                        }.into_any()
                                    }}
                                </div>
                            </div>

                            // Offices Column
                            <div class="lg:col-span-2 space-y-4">
                                <h2 class="text-sm font-semibold text-slate-700">
                                    {move || if is_ar() { "المواقع والمكاتب العالمية" } else { "Global Locations" }}
                                </h2>
                                {OFFICES.iter().map(|office| {
                                    view! {
                                        <div class="bg-white border border-slate-200 rounded-xl p-5 shadow-sm">
                                            <div class="flex items-center gap-2 mb-2">
                                                <div class="w-7 h-7 bg-slate-100 rounded-md flex items-center justify-center text-slate-600">
                                                    <Icon name="building".to_string() size=13 />
                                                </div>
                                                <div>
                                                    <p class="text-sm font-semibold text-slate-900">{move || if is_ar() { office.city_ar } else { office.city_en }}</p>
                                                    <p class="text-xs text-slate-500">{move || if is_ar() { office.country_ar } else { office.country_en }}</p>
                                                </div>
                                            </div>
                                            <p class="text-xs text-slate-600 mb-2">{move || if is_ar() { office.address_ar } else { office.address_en }}</p>
                                            <div class="space-y-1">
                                                <a href=format!("tel:{}", office.phone) class="flex items-center gap-1.5 text-xs text-slate-600 hover:text-brand-600 ltr:direction-ltr" dir="ltr">
                                                    <Icon name="activity".to_string() size=11 class="text-slate-400".to_string() />
                                                    {office.phone}
                                                </a>
                                                <a href=format!("mailto:{}", office.email) class="flex items-center gap-1.5 text-xs text-brand-600 hover:underline ltr:direction-ltr" dir="ltr">
                                                    <Icon name="mail".to_string() size=11 class="text-brand-400".to_string() />
                                                    {office.email}
                                                </a>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}

                                // 24/7 Support Desk Card
                                <div class="bg-brand-50 border border-brand-200 rounded-xl p-5">
                                    <div class="flex items-center gap-2 mb-2">
                                        <Icon name="shield".to_string() size=16 class="text-brand-600".to_string() />
                                        <p class="text-sm font-semibold text-brand-900">
                                            {move || if is_ar() { "دعم فني واستجابة على مدار الساعة (SLA)" } else { "24/7 SLA Engineering Support" }}
                                        </p>
                                    </div>
                                    <p class="text-xs text-brand-700 leading-relaxed">
                                        {move || if is_ar() {
                                            "للعملاء المؤسسيين بموجب عقود الصيانة والدعم النشطة، يعمل مركز عمليات الشبكة والأمن (NOC/SOC) لدينا على مدار الساعة بلا انقطاع."
                                        } else {
                                            "For enterprise clients under active SLA maintenance, our Network Operations Center (NOC) is staffed around the clock."
                                        }}
                                    </p>
                                    <a href="tel:+18884832255" class="mt-3 inline-flex items-center gap-1.5 text-xs font-semibold text-brand-700 hover:underline" dir="ltr">
                                        <Icon name="activity".to_string() size=12 />
                                        "+1 888 483 2255 (Toll-Free Priority Line)"
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        </PublicLayout>
    }
}

