//! # Careers Page
//!
//! Matches UI Careers reference:
//! Open engineering roles, company culture perks, interactive job application modal dialog,
//! and career inquiry options.

use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use design_system::i18n::Locale;
use crate::i18n::context::use_locale;
use ui::components::{Badge, Icon};
use ui::layout::PublicLayout;

struct JobOpening {
    id: &'static str,
    title_en: &'static str,
    title_ar: &'static str,
    dept_en: &'static str,
    dept_ar: &'static str,
    loc_en: &'static str,
    loc_ar: &'static str,
    type_en: &'static str,
    type_ar: &'static str,
    desc_en: &'static str,
    desc_ar: &'static str,
}

const OPENINGS: &[JobOpening] = &[
    JobOpening {
        id: "dist-sys",
        title_en: "Senior Distributed Systems Engineer (Rust / Go)",
        title_ar: "كبير مهندسي النظم الموزعة (Rust / Go)",
        dept_en: "Core Engineering",
        dept_ar: "الهندسة الأساسية",
        loc_en: "Remote / Lagos / London",
        loc_ar: "عن بعد / لاغوس / لندن",
        type_en: "Full-time",
        type_ar: "دوام كامل",
        desc_en: "Design and build low-latency transactional systems and microservices for tier-1 financial and infrastructure clients.",
        desc_ar: "تصميم وبناء نظم معاملات فائقة السرعة ومنخفضة الاستجابة وخدمات مصغرة لمؤسسات مالية وبنى تحتية كبرى.",
    },
    JobOpening {
        id: "cloud-arch",
        title_en: "Staff Cloud & DevOps Architect (AWS / K8s)",
        title_ar: "خبير معماري للسحابة و DevOps (AWS / K8s)",
        dept_en: "Infrastructure & Platform",
        dept_ar: "البنية التحتية والمنصات",
        loc_en: "Remote / Toronto / Dubai",
        loc_ar: "عن بعد / تورونتو / دبي",
        type_en: "Full-time",
        type_ar: "دوام كامل",
        desc_en: "Lead multi-region Kubernetes deployments, automated FinOps pipelines, and enterprise disaster recovery architectures.",
        desc_ar: "قيادة نشر مجموعات Kubernetes متعددة المناطق، وأتمتة مسارات تحسين التكاليف (FinOps)، وهندسة التعافي المؤسسي من الكوارث.",
    },
    JobOpening {
        id: "cyber-sec",
        title_en: "Principal Cybersecurity Engineer",
        title_ar: "كبير مهندسي الأمن السيبراني والامتثال",
        dept_en: "Security Operations",
        dept_ar: "العمليات الأمنية (SOC)",
        loc_en: "Remote / London",
        loc_ar: "عن بعد / لندن",
        type_en: "Full-time",
        type_ar: "دوام كامل",
        desc_en: "Conduct high-stakes penetration tests, design SOC infrastructures, and implement Zero-Trust frameworks for regulated enterprises.",
        desc_ar: "تنفيذ اختبارات الاختراق المتقدمة، وتصميم بنى مراكز العمليات الأمنية (SOC)، وتطبيق أطر Zero-Trust للمؤسسات المنظمة.",
    },
    JobOpening {
        id: "ai-res",
        title_en: "Senior AI / ML Research Engineer",
        title_ar: "باحث ومهندس أول في الذكاء الاصطناعي وتعلم الآلة",
        dept_en: "AI & Data Engineering",
        dept_ar: "هندسة الذكاء الاصطناعي والبيانات",
        loc_en: "Remote",
        loc_ar: "عن بعد",
        type_en: "Full-time",
        type_ar: "دوام كامل",
        desc_en: "Deploy fine-tuned LLM pipelines, real-time analytics engines, and robust MLOps workflows in production.",
        desc_ar: "نشر مسارات النماذج اللغوية الدقيقة (LLMs)، ومحركات التحليلات الفورية، وأطر عمل MLOps المتقدمة في بيئات الإنتاج الفعلية.",
    },
];

#[component]
pub fn CareersPage() -> impl IntoView {
    let locale_ctx = use_locale();
    let is_ar = move || locale_ctx.get() == Locale::Ar;

    let selected_job = RwSignal::new(Option::<&'static str>::None);

    // Application Form state
    let candidate_name = RwSignal::new(String::new());
    let candidate_email = RwSignal::new(String::new());
    let candidate_url = RwSignal::new(String::new());
    let candidate_note = RwSignal::new(String::new());

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::ev::keydown;
        leptos::prelude::window_event_listener(keydown, move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" && selected_job.get().is_some() {
                selected_job.set(None);
            }
        });
    }

    let name_err = RwSignal::new(Option::<(&'static str, &'static str)>::None);
    let email_err = RwSignal::new(Option::<(&'static str, &'static str)>::None);
    let url_err = RwSignal::new(Option::<(&'static str, &'static str)>::None);
    let is_submitted = RwSignal::new(false);
    let is_submitting = RwSignal::new(false);

    let open_modal = move |job_id: &'static str| {
        selected_job.set(Some(job_id));
        is_submitted.set(false);
        candidate_name.set(String::new());
        candidate_email.set(String::new());
        candidate_url.set(String::new());
        candidate_note.set(String::new());
        name_err.set(None);
        email_err.set(None);
        url_err.set(None);
    };

    let handle_apply_submit = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        let mut has_error = false;

        let name = candidate_name.get().trim().to_string();
        if name.is_empty() {
            name_err.set(Some(("Please provide your full name", "يرجى تقديم اسمك الكامل")));
            has_error = true;
        } else {
            name_err.set(None);
        }

        let em = candidate_email.get().trim().to_string();
        if em.is_empty() || !em.contains('@') || !em.contains('.') {
            email_err.set(Some(("Please provide a valid email address", "يرجى تقديم بريد إلكتروني صالح")));
            has_error = true;
        } else {
            email_err.set(None);
        }

        let url = candidate_url.get().trim().to_string();
        if url.is_empty() {
            url_err.set(Some(("Please provide your LinkedIn, GitHub, or portfolio URL", "يرجى توفير رابط حساب LinkedIn أو GitHub أو معرض أعمالك")));
            has_error = true;
        } else {
            url_err.set(None);
        }

        if has_error {
            return;
        }

        is_submitting.set(true);
        is_submitting.set(false);
        is_submitted.set(true);
    };

    view! {
        <Title text=move || if is_ar() { "الوظائف والفرص الهندسية — تيب تك" } else { "Careers — TeebTech" }/>
        <Meta name="description" content=move || if is_ar() {
            "انضم إلى نخبة المهندسين في تيب تك. استكشف الفرص الوظيفية الدولية والعمل عن بعد في تطوير البرمجيات، السحابة، الأمن السيبراني، والذكاء الاصطناعي."
        } else {
            "Join TeebTech's world-class engineering team. Explore remote and international career opportunities in software, cloud, security, and AI."
        }/>

        <PublicLayout current_page="careers".to_string()>
            <div>
                // Hero
                <section class="bg-white border-b border-slate-200 py-14 lg:py-18">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <div class="max-w-2xl">
                            <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-3">
                                {move || if is_ar() { "الوظائف والفرص في تيب تك" } else { "Careers at TeebTech" }}
                            </p>
                            <h1 class="font-display text-4xl lg:text-5xl font-semibold text-slate-900 leading-tight mb-4">
                                {move || if is_ar() {
                                    "ابنِ تقنيات نوعية مؤثرة مع نخبة من المهندسين العالميين."
                                } else {
                                    "Build technology that matters with world-class peers."
                                }}
                            </h1>
                            <p class="text-slate-500 text-base lg:text-lg leading-relaxed">
                                {move || if is_ar() {
                                    "نحن فريق عالمي يضم أكثر من ١٨٠ من كبار المهندسين والمصممين ومعماري النظم عبر ٢٢ جنسية. بلا بيروقراطية معقدة — فقط التميز الهندسي والأثر الملموس."
                                } else {
                                    "We are a global team of 180+ senior engineers, designers, and systems architects spanning 22 nationalities. No corporate bureaucracy — just engineering excellence and meaningful impact."
                                }}
                            </p>
                        </div>
                    </div>
                </section>

                // Openings Section
                <section class="bg-slate-50 py-12 lg:py-16">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <div class="flex items-center justify-between mb-8 flex-wrap gap-4">
                            <div>
                                <h2 class="font-display text-2xl font-semibold text-slate-900">
                                    {move || if is_ar() { "الوظائف الشاغرة" } else { "Open Positions" }}
                                </h2>
                                <p class="text-sm text-slate-500 mt-1">
                                    {move || if is_ar() { "استكشف الفرص المتاحة لتقديم أفضل إنجازاتك المهنية." } else { "Explore current opportunities to do the best work of your career." }}
                                </p>
                            </div>
                            <Badge variant="navy".to_string()>{move || if is_ar() { "٤ شواغر متاحة" } else { "4 Open Roles" }}</Badge>
                        </div>

                        <div class="space-y-4">
                            {OPENINGS.iter().map(|job| {
                                let j_id = job.id;
                                view! {
                                    <div class="bg-white border border-slate-200 rounded-xl p-6 sm:p-7 hover:shadow-lg hover:border-brand-300 transition-all">
                                        <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-6">
                                            <div class="flex-1">
                                                <div class="flex items-center gap-2 mb-2 flex-wrap">
                                                    <span class="text-xs font-semibold text-brand-600 uppercase tracking-wider">
                                                        {move || if is_ar() { job.dept_ar } else { job.dept_en }}
                                                    </span>
                                                    <span class="text-slate-300">"•"</span>
                                                    <span class="text-xs text-slate-500 font-medium">
                                                        {move || if is_ar() { job.loc_ar } else { job.loc_en }}
                                                    </span>
                                                    <span class="text-slate-300">"•"</span>
                                                    <Badge variant="neutral".to_string()>
                                                        {move || if is_ar() { job.type_ar } else { job.type_en }}
                                                    </Badge>
                                                </div>
                                                <h3 class="text-lg font-bold text-slate-900">
                                                    {move || if is_ar() { job.title_ar } else { job.title_en }}
                                                </h3>
                                                <p class="text-sm text-slate-600 mt-2 max-w-3xl leading-relaxed">
                                                    {move || if is_ar() { job.desc_ar } else { job.desc_en }}
                                                </p>
                                            </div>
                                            <button
                                                type="button"
                                                on:click=move |_| open_modal(j_id)
                                                class="inline-flex items-center justify-center font-medium rounded-lg text-sm px-5 py-2.5 bg-brand-600 text-white hover:bg-brand-700 transition-colors flex-shrink-0 gap-2 cursor-pointer shadow-sm focus:outline-none focus:ring-2 focus:ring-brand-500"
                                            >
                                                <span>{move || if is_ar() { "قدّم الآن" } else { "Apply Now" }}</span>
                                                <Icon name="arrowRight".to_string() size=14 class="rtl:rotate-180".to_string() />
                                            </button>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </section>

                // Interactive Application Modal Dialog
                {move || selected_job.get().map(|job_id| {
                    let job_opt = OPENINGS.iter().find(|j| j.id == job_id);
                    let title = job_opt.map(|j| if is_ar() { j.title_ar } else { j.title_en }).unwrap_or(job_id);
                    view! {
                        <div
                            class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 bg-slate-950/70 backdrop-blur-xs animate-fade-in overflow-y-auto cursor-pointer"
                            role="dialog"
                            aria-modal="true"
                            aria-labelledby="career-modal-title"
                            on:click=move |_| selected_job.set(None)
                        >
                            <div
                                class="bg-white rounded-2xl shadow-2xl max-w-lg w-full p-6 sm:p-8 border border-slate-200 relative my-auto animate-scale-up cursor-default"
                                on:click=move |e| e.stop_propagation()
                            >
                                <button
                                    type="button"
                                    on:click=move |_| selected_job.set(None)
                                    class="absolute top-4 right-4 rtl:right-auto rtl:left-4 text-slate-400 hover:text-slate-600 p-2 rounded-lg hover:bg-slate-100 transition-colors cursor-pointer"
                                    aria-label="Close application form"
                                >
                                    <Icon name="x".to_string() size=18 />
                                </button>

                                {move || if is_submitted.get() {
                                    view! {
                                        <div class="text-center py-6">
                                            <div class="w-16 h-16 rounded-full bg-emerald-100 text-emerald-600 flex items-center justify-center mx-auto mb-4">
                                                <Icon name="checkCircle".to_string() size=32 />
                                            </div>
                                            <h3 class="font-display text-2xl font-bold text-slate-900 mb-2">
                                                {if is_ar() { "تم استلام طلب التوظيف بنجاح!" } else { "Application Received!" }}
                                            </h3>
                                            <p class="text-sm text-slate-600 mb-6 max-w-sm mx-auto leading-relaxed">
                                                {if is_ar() { "شكرًا لتقديمك على وظيفة " } else { "Thank you for applying for the " }}
                                                <strong>{title}</strong>
                                                {if is_ar() { ". سيقوم فريق استقطاب الكفاءات الهندسية لدينا بمراجعة ملفك والرد خلال ٤٨ ساعة عمل." } else { " position. Our talent acquisition engineering team will review your profile and respond within 48 business hours." }}
                                            </p>
                                            <button
                                                type="button"
                                                on:click=move |_| selected_job.set(None)
                                                class="px-6 py-2.5 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 transition-colors cursor-pointer"
                                            >
                                                {if is_ar() { "إغلاق" } else { "Done" }}
                                            </button>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div>
                                            <div class="mb-5">
                                                <span class="text-xs font-semibold text-brand-600 uppercase tracking-wider">
                                                    {if is_ar() { "طلب تقديم على وظيفة" } else { "Job Application" }}
                                                </span>
                                                <h3 id="career-modal-title" class="font-display text-xl font-bold text-slate-900 mt-1">
                                                    {title}
                                                </h3>
                                            </div>

                                            <form on:submit=handle_apply_submit class="space-y-4" novalidate=true>
                                                <div>
                                                    <label for="apply-name" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "الاسم الكامل *" } else { "Full Name *" }}
                                                    </label>
                                                    <input
                                                        id="apply-name"
                                                        type="text"
                                                        placeholder=if is_ar() { "مثال: مريم العلي" } else { "Jane Doe" }
                                                        required
                                                        prop:value=move || candidate_name.get()
                                                        class=move || format!(
                                                            "w-full px-3.5 py-2 text-sm border rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white {}",
                                                            if name_err.get().is_some() { "border-red-500 bg-red-50/20" } else { "border-slate-300" }
                                                        )
                                                        on:input=move |e| {
                                                            name_err.set(None);
                                                            candidate_name.set(event_target_value(&e));
                                                        }
                                                    />
                                                    {move || name_err.get().map(|(en, ar)| view! {
                                                        <p class="text-xs text-red-600 mt-1 flex items-center gap-1">
                                                            <Icon name="alertCircle".to_string() size=11 />
                                                            {if is_ar() { ar } else { en }}
                                                        </p>
                                                    })}
                                                </div>

                                                <div>
                                                    <label for="apply-email" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "البريد الإلكتروني *" } else { "Email Address *" }}
                                                    </label>
                                                    <input
                                                        id="apply-email"
                                                        type="email"
                                                        placeholder=if is_ar() { "name@example.com" } else { "jane@example.com" }
                                                        required
                                                        prop:value=move || candidate_email.get()
                                                        class=move || format!(
                                                            "w-full px-3.5 py-2 text-sm border rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white {}",
                                                            if email_err.get().is_some() { "border-red-500 bg-red-50/20" } else { "border-slate-300" }
                                                        )
                                                        on:input=move |e| {
                                                            email_err.set(None);
                                                            candidate_email.set(event_target_value(&e));
                                                        }
                                                    />
                                                    {move || email_err.get().map(|(en, ar)| view! {
                                                        <p class="text-xs text-red-600 mt-1 flex items-center gap-1">
                                                            <Icon name="alertCircle".to_string() size=11 />
                                                            {if is_ar() { ar } else { en }}
                                                        </p>
                                                    })}
                                                </div>

                                                <div>
                                                    <label for="apply-url" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "رابط LinkedIn أو GitHub أو معرض الأعمال *" } else { "LinkedIn, GitHub, or Portfolio URL *" }}
                                                    </label>
                                                    <input
                                                        id="apply-url"
                                                        type="url"
                                                        placeholder="https://linkedin.com/in/username"
                                                        dir="ltr"
                                                        required
                                                        prop:value=move || candidate_url.get()
                                                        class=move || format!(
                                                            "w-full px-3.5 py-2 text-sm border rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white {}",
                                                            if url_err.get().is_some() { "border-red-500 bg-red-50/20" } else { "border-slate-300" }
                                                        )
                                                        on:input=move |e| {
                                                            url_err.set(None);
                                                            candidate_url.set(event_target_value(&e));
                                                        }
                                                    />
                                                    {move || url_err.get().map(|(en, ar)| view! {
                                                        <p class="text-xs text-red-600 mt-1 flex items-center gap-1">
                                                            <Icon name="alertCircle".to_string() size=11 />
                                                            {if is_ar() { ar } else { en }}
                                                        </p>
                                                    })}
                                                </div>

                                                <div>
                                                    <label for="apply-note" class="block text-xs font-medium text-slate-700 mb-1">
                                                        {if is_ar() { "نبذة مختصرة / لماذا تيب تك؟ (اختياري)" } else { "Cover Note / Why TeebTech? (optional)" }}
                                                    </label>
                                                    <textarea
                                                        id="apply-note"
                                                        rows="3"
                                                        placeholder=if is_ar() {
                                                            "شاركنا بأبرز المشاريع التي قمت ببنائها أو سبب اهتمامك بهذه الفرصة..."
                                                        } else {
                                                            "Share what projects you've built or why you're excited about this role..."
                                                        }
                                                        prop:value=move || candidate_note.get()
                                                        class="w-full px-3.5 py-2 text-sm border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-500 bg-white"
                                                        on:input=move |e| candidate_note.set(event_target_value(&e))
                                                    >{move || candidate_note.get()}</textarea>
                                                </div>

                                                <div class="flex gap-3 pt-3">
                                                    <button
                                                        type="button"
                                                        on:click=move |_| selected_job.set(None)
                                                        class="flex-1 py-2.5 border border-slate-300 text-slate-700 rounded-lg text-sm font-medium hover:bg-slate-50 transition-colors cursor-pointer"
                                                    >
                                                        {if is_ar() { "إلغاء" } else { "Cancel" }}
                                                    </button>
                                                    <button
                                                        type="submit"
                                                        disabled=is_submitting
                                                        class="flex-1 py-2.5 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 transition-colors shadow-sm cursor-pointer disabled:opacity-50"
                                                    >
                                                        {move || if is_submitting.get() {
                                                            if is_ar() { "جاري الإرسال..." } else { "Submitting..." }
                                                        } else {
                                                            if is_ar() { "إرسال الطلب" } else { "Submit Application" }
                                                        }}
                                                    </button>
                                                </div>
                                            </form>
                                        </div>
                                    }.into_any()
                                }}
                            </div>
                        </div>
                    }
                })}
            </div>
        </PublicLayout>
    }
}

