//! # Case Studies Page
//!
//! Matches UI reference CaseStudies.tsx with full Arabic and English support:
//! Filter tabs by industry, grid of case cards with metrics, tags, durations,
//! interactive deep-dive modal for each case study, and proposal CTA.

use crate::i18n::use_locale;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use ui::components::{Badge, Icon};
use ui::layout::PublicLayout;

struct CaseStudy {
    id: usize,
    industry_en: &'static str,
    industry_ar: &'static str,
    service_en: &'static str,
    service_ar: &'static str,
    title_en: &'static str,
    title_ar: &'static str,
    result_en: &'static str,
    result_ar: &'static str,
    duration_en: &'static str,
    duration_ar: &'static str,
    image: &'static str,
    tags: &'static [&'static str],
    client_en: &'static str,
    client_ar: &'static str,
    challenge_en: &'static str,
    challenge_ar: &'static str,
    solution_en: &'static str,
    solution_ar: &'static str,
    key_metrics_en: &'static [&'static str],
    key_metrics_ar: &'static [&'static str],
}

const ALL_CASES: &[CaseStudy] = &[
    CaseStudy {
        id: 1,
        industry_en: "Finance",
        industry_ar: "المالية",
        service_en: "Cloud & DevOps",
        service_ar: "السحابة والعمليات",
        title_en: "Core Banking Modernization for FirstNile Bank",
        title_ar: "تحديث المنظومة المصرفية الأساسية لبنك النيل الأول",
        result_en: "99.97% uptime, 60% faster transaction processing",
        result_ar: "جاهزية 99.97% ومعالجة أسرع للعمليات بنسبة 60%",
        duration_en: "14 months",
        duration_ar: "14 شهراً",
        image: "https://images.unsplash.com/photo-1560472355-536de3962603?w=600&h=360&fit=crop&auto=format",
        tags: &["AWS", "Kubernetes", "Security"],
        client_en: "FirstNile Commercial Bank",
        client_ar: "بنك النيل الأول التجاري",
        challenge_en: "FirstNile's legacy monolithic mainframe suffered frequent outages during peak month-end salary processing cycles, with deployment lead times exceeding six weeks.",
        challenge_ar: "كانت المنظومة المركزية القديمة لبنك النيل تعاني من انقطاعات متكررة خلال دورات تحويل الرواتب الشهرية، مع بطء شديد في دورة نشر التحديثات استمرت لأسابيع.",
        solution_en: "Engineered an event-driven microservices architecture on AWS EKS with automated zero-downtime canary deployments, Kafka streaming, and end-to-end PCI-DSS compliant encryption.",
        solution_ar: "قمنا بهندسة معمارية خدمات مصغرة موجهة بالأحداث على AWS EKS مع نشر تدريجي مؤتمت بدون أي توقف، واستخدام Kafka لمعالجة البيانات وتشفير مطابق لـ PCI-DSS.",
        key_metrics_en: &[
            "99.97% High Availability SLA maintained",
            "60% reduction in end-to-end transaction latency",
            "Deployment frequency increased from monthly to 12x/day",
            "Zero downtime migrations across 4.2M active accounts",
        ],
        key_metrics_ar: &[
            "الحفاظ على نسبة جاهزية 99.97% وفق اتفاقية SLA",
            "تقليص زمن معالجة المعاملات المصرفية بنسبة 60%",
            "زيادة وتيرة النشر والتحديث من شهرياً إلى 12 مرة يومياً",
            "ترحيل سلس لبيانات 4.2 مليون حساب نشط بدون أي توقف",
        ],
    },
    CaseStudy {
        id: 2,
        industry_en: "Healthcare",
        industry_ar: "الرعاية الصحية",
        service_en: "AI & Data",
        service_ar: "الذكاء الاصطناعي",
        title_en: "AI-Powered Diagnostic Platform for MedCore Group",
        title_ar: "منصة تشخيص مدعومة بالذكاء الاصطناعي لمجموعة ميدكور",
        result_en: "40% reduction in diagnostic turnaround time",
        result_ar: "تقليص زمن التشخيص الطبي بنسبة 40%",
        duration_en: "9 months",
        duration_ar: "9 أشهر",
        image: "https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?w=600&h=360&fit=crop&auto=format",
        tags: &["ML", "HIPAA", "Python"],
        client_en: "MedCore Health Consortium",
        client_ar: "مجموعة ميدكور للرعاية الصحية",
        challenge_en: "Radiologists faced massive backlogs analyzing DICOM scans across 18 regional hospitals, resulting in delayed diagnoses for critical pulmonary conditions.",
        challenge_ar: "واجه أطباء الأشعة تراكمات هائلة في تحليل صور الأشعة السينية عبر 18 مستشفى إقليمياً، مما تسبب في تأخر تشخيص الحالات الصدرية الحرجة.",
        solution_en: "Built a HIPAA-compliant computer vision diagnostic pipeline utilizing fine-tuned neural networks running on secure GPU clusters, integrating directly with legacy PACS systems.",
        solution_ar: "بنينا مسار تشخيص آلي بالرؤية الحاسوبية متوافقاً مع HIPAA يعتمد على شبكات عصبية مدربة تعمل على وحدات GPU مشفرة، وتتكامل مباشرة مع أنظمة PACS الطبية.",
        key_metrics_en: &[
            "40% reduction in triage and turnaround time",
            "96.4% diagnostic concordance with senior radiologists",
            "Over 180,000 patient scans securely processed in year one",
            "Full HIPAA and ISO 27799 clinical data compliance",
        ],
        key_metrics_ar: &[
            "تقليص زمن الفرز والتشخيص السريري بنسبة 40%",
            "تطابق تشخيصي بنسبة 96.4% مع تقارير كبار استشاريي الأشعة",
            "معالجة آمنة لأكثر من 180,000 فحص طبي خلال العام الأول",
            "امتثال كامل لمعايير HIPAA و ISO 27799 لحماية البيانات الطبية",
        ],
    },
    CaseStudy {
        id: 3,
        industry_en: "Retail",
        industry_ar: "التجارة الإلكترونية",
        service_en: "Software Dev",
        service_ar: "تطوير البرمجيات",
        title_en: "Omnichannel Commerce Suite for PanAfrica Retail",
        title_ar: "منصة تجارة التجزئة الموحدة لشركة بان أفريكا",
        result_en: "3.2x revenue growth in 18 months",
        result_ar: "نمو الإيرادات بمعدل 3.2 أضعاف خلال 18 شهراً",
        duration_en: "12 months",
        duration_ar: "12 شهراً",
        image: "https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=600&h=360&fit=crop&auto=format",
        tags: &["React", "ERP", "Mobile"],
        client_en: "PanAfrica Retail Holdings",
        client_ar: "مجموعة بان أفريكا القابضة للتجزئة",
        challenge_en: "Disconnected in-store POS terminals and digital channels caused inventory inaccuracies, ghost stockouts, and fragmented customer loyalty data across 140 retail locations.",
        challenge_ar: "تسبب تباين نقاط البيع في المتاجر وعدم اتصالها بالمنصات الرقمية في أخطاء بالمخزون ونفاد وهمي للمنتجات عبر 140 فرعاً للشركة.",
        solution_en: "Architected a unified headless commerce platform with real-time multi-warehouse inventory synchronization, progressive web apps, and offline-first store operations.",
        solution_ar: "صممنا منصة تجارة رقمية موحدة مع مزامنة لحظية للمخزون عبر المستودعات وتطبيقات ويب تقدمية تدعم العمل بدون إنترنت لنقاط البيع.",
        key_metrics_en: &[
            "3.2x online-to-offline omnichannel revenue growth",
            "Inventory discrepancy reduced from 14% to under 0.3%",
            "Sub-second checkout speeds during Black Friday surges",
            "99.99% POS terminal uptime across all branches",
        ],
        key_metrics_ar: &[
            "نمو الإيرادات الموحدة عبر القنوات بمعدل 3.2 أضعاف",
            "انخفاض نسبة الخطأ في المخزون من 14% إلى أقل من 0.3%",
            "إتمام عمليات الدفع في أجزاء من الثانية خلال مواسم الذروة",
            "جاهزية بنسبة 99.99% لنقاط البيع في كافة الفروع",
        ],
    },
    CaseStudy {
        id: 4,
        industry_en: "Government",
        industry_ar: "القطاع الحكومي",
        service_en: "Cybersecurity",
        service_ar: "الأمن السيبراني",
        title_en: "National Digital ID Infrastructure for NIDCO",
        title_ar: "البنية التحتية للهوية الرقمية الوطنية لـ NIDCO",
        result_en: "ISO 27001 certified, 12M citizens enrolled",
        result_ar: "اعتماد ISO 27001 وتسجيل أكثر من 12 مليون مواطن",
        duration_en: "20 months",
        duration_ar: "20 شهراً",
        image: "https://images.unsplash.com/photo-1529699211952-734e80c4d42b?w=600&h=360&fit=crop&auto=format",
        tags: &["Security", "Biometrics", "Compliance"],
        client_en: "National Identity Commission (NIDCO)",
        client_ar: "الهيئة الوطنية للهوية الرقمية",
        challenge_en: "The national identification system required biometric deduplication and cryptographic verification at population scale while withstanding sophisticated nation-state cyber threats.",
        challenge_ar: "تطلب نظام الهوية الوطنية معالجة البيانات الحيوية والتحقق الرقمي الموثوق على نطاق سكاني واسع مع التصدي للتهديدات السيبرانية المتقدمة.",
        solution_en: "Implemented hardware security modules (HSMs), Zero Trust perimeter architecture, automated penetration testing pipelines, and an ISO 27001 audited Security Operations Center.",
        solution_ar: "طبقنا وحدات التشفير المادية (HSMs) ومعمارية الثقة الصفرية ومسارات اختبار الاختراق المستمرة وإنشاء مركز مراقبة أمنية معتمد وفق ISO 27001.",
        key_metrics_en: &[
            "12+ Million citizens successfully enrolled",
            "Zero security breaches or data exfiltrations detected",
            "Sub-50ms cryptographic signature verification",
            "Full compliance with ISO 27001 and GDPR sovereign guidelines",
        ],
        key_metrics_ar: &[
            "تسجيل أكثر من 12 مليون مواطن بنجاح وأمان",
            "صفر اختراقات أمنية أو تسريبات للبيانات السيادية",
            "التحقق من التوقيع الرقمي في أقل من 50 مللي ثانية",
            "امتثال كامل لمعايير ISO 27001 وسياسات حماية البيانات",
        ],
    },
    CaseStudy {
        id: 5,
        industry_en: "Manufacturing",
        industry_ar: "الصناعة",
        service_en: "ERP & CRM",
        service_ar: "أنظمة ERP",
        title_en: "SAP S/4HANA Rollout for Consolidated Steel Works",
        title_ar: "تطبيق SAP S/4HANA لمصانع الصلب المتحدة",
        result_en: "35% reduction in procurement cycle time",
        result_ar: "تقليص دورة المشتريات والتوريد بنسبة 35%",
        duration_en: "18 months",
        duration_ar: "18 شهراً",
        image: "https://images.unsplash.com/photo-1581091226825-a6a2a5aee158?w=600&h=360&fit=crop&auto=format",
        tags: &["SAP", "ERP", "Integration"],
        client_en: "Consolidated Steel Works Ltd.",
        client_ar: "شركة مصانع الصلب المتحدة المحدودة",
        challenge_en: "Fragmented spreadsheets and regional legacy ERP silos delayed supply chain procurement approvals by an average of 22 days, causing production line bottlenecks.",
        challenge_ar: "أدت الجداول المتفرقة وأنظمة ERP الإقليمية المتقادمة إلى تأخير اعتمادات التوريد بمعدل 22 يوماً، مما خلق اختناقات تشغيلية في خطوط الإنتاج.",
        solution_en: "Executed an enterprise-wide SAP S/4HANA migration integrating IoT sensor feeds from factory smelting lines with real-time automated supplier procurement workflows.",
        solution_ar: "نفذنا ترحيلاً شاملاً إلى SAP S/4HANA مع ربط أجهزة استشعار إنترنت الأشياء في خطوط الصهر بمسارات أتمتة طلبات الموردين في الوقت الفعلي.",
        key_metrics_en: &[
            "35% reduction in overall procurement cycle duration",
            "$4.8M annual working capital savings from optimized inventory",
            "Automated compliance tracking across 6 manufacturing plants",
            "Real-time visibility into scrap rates and production yields",
        ],
        key_metrics_ar: &[
            "تقليص مدة دورة المشتريات بنسبة 35%",
            "توفير 4.8 مليون دولار سنوياً في رأس المال العامل بفضل تحسين المخزون",
            "تتبع آلي للامتثال ومعايير الجودة عبر 6 مصانع متكاملة",
            "رؤية لحظية لمعدلات الإنتاج ونسب الفاقد بدقة متناهية",
        ],
    },
    CaseStudy {
        id: 6,
        industry_en: "Telecom",
        industry_ar: "الاتصالات",
        service_en: "Software Dev",
        service_ar: "تطوير البرمجيات",
        title_en: "Self-Service Portal Rebuild for NovaTel ISP",
        title_ar: "إعادة بناء بوابة الخدمة الذاتية لشركة نوفاتيل",
        result_en: "82% CSAT score, 50% reduction in support tickets",
        result_ar: "نسبة رضا 82% وتقليص تذاكر الدعم بنسبة 50%",
        duration_en: "7 months",
        duration_ar: "7 أشهر",
        image: "https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=600&h=360&fit=crop&auto=format",
        tags: &["React", "Node.js", "UX"],
        client_en: "NovaTel Telecommunications",
        client_ar: "شركة نوفاتيل للاتصالات",
        challenge_en: "Customer service call centers were overwhelmed with routine billing queries and fiber bandwidth upgrade requests, driving customer churn to historic highs.",
        challenge_ar: "كانت مراكز الاتصال والدعم مثقلة باستفسارات الفواتير وطلبات ترقية سرعات الألياف البصرية، مما رفع معدل فقدان العملاء إلى مستويات غير مسبوقة.",
        solution_en: "Designed and built an intuitive self-service subscriber portal and mobile app with one-click payment processing, diagnostic bandwidth tests, and automated plan changes.",
        solution_ar: "صممنا وطورنا بوابة تفاعلية وتطبيقاً للهاتف للخدمة الذاتية يتيح الدفع بضغطة زر وفحص سرعة الاتصال وتغيير باقات الاشتراك آلياً.",
        key_metrics_en: &[
            "50% reduction in inbound support ticket volume",
            "Customer satisfaction score (CSAT) surged from 46% to 82%",
            "Self-service bill payments increased by 140%",
            "Average customer resolution time dropped from 4 hours to 2 minutes",
        ],
        key_metrics_ar: &[
            "انخفاض حجم تذاكر الدعم الواردة بنسبة 50%",
            "ارتفاع مؤشر رضا العملاء CSAT من 46% إلى 82%",
            "زيادة سداد الفواتير عبر الخدمة الذاتية بنسبة 140%",
            "انخفاض متوسط زمن معالجة الطلب من 4 ساعات إلى دقيقتين فقط",
        ],
    },
];

const INDUSTRIES: &[(&'static str, &'static str)] = &[
    ("All", "الكل"),
    ("Finance", "المالية"),
    ("Healthcare", "الرعاية الصحية"),
    ("Retail", "التجارة الإلكترونية"),
    ("Government", "القطاع الحكومي"),
    ("Manufacturing", "الصناعة"),
    ("Telecom", "الاتصالات"),
];

#[component]
pub fn CaseStudiesPage() -> impl IntoView {
    let locale_ctx = use_locale();
    let filter = RwSignal::new("All");
    let selected_case_id = RwSignal::new(Option::<usize>::None);

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::ev::keydown;
        leptos::prelude::window_event_listener(keydown, move |e: web_sys::KeyboardEvent| {
            if e.key() == "Escape" && selected_case_id.get().is_some() {
                selected_case_id.set(None);
            }
        });
    }

    view! {
        <Title text=move || if locale_ctx.is_arabic() {
            "دراسات الحالة وسجل الأعمال — تيب تك"
        } else {
            "Case Studies & Work — TeebTech"
        } />
        <Meta name="description" content=move || if locale_ctx.is_arabic() {
            "استكشف نتائج حقيقية حققتها تيب تك لعملائها في السحابة والذكاء الاصطناعي والأمن السيبراني والبرمجيات المؤسسية."
        } else {
            "Explore real enterprise outcomes delivered by TeebTech across cloud migrations, AI engineering, cybersecurity, and scalable software platforms."
        } />

        <PublicLayout current_page="work".to_string()>
            <div>
                // Hero
                <section class="bg-white border-b border-slate-200 py-12 lg:py-16 text-start">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <div class="max-w-2xl">
                            <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                                {move || if locale_ctx.is_arabic() { "دراسات الحالة" } else { "Case Studies" }}
                            </p>
                            <h1 class="font-display text-4xl lg:text-5xl font-semibold text-slate-900 leading-tight mb-4">
                                {move || if locale_ctx.is_arabic() { "إنجازات تصنع فارقاً حقيقياً" } else { "Work that moves the needle" }}
                            </h1>
                            <p class="text-slate-500 text-base leading-relaxed">
                                {move || if locale_ctx.is_arabic() {
                                    "مشاريع هندسية حقيقية ونتائج مؤسسية قابلة للقياس. تعرف على كيفية حل فرقنا لأعقد التحديات في البنية التحتية والأمن والبرمجيات."
                                } else {
                                    "Real engineering projects, measurable enterprise outcomes. Explore how our senior teams solve complex software, infrastructure, and security challenges."
                                }}
                            </p>
                        </div>
                    </div>
                </section>

                // Content Section
                <section class="bg-slate-50 py-12">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        // Industry Filter Tabs
                        <div class="flex items-center gap-2 flex-wrap mb-8" role="tablist" aria-label="Filter case studies by industry">
                            {INDUSTRIES.iter().map(|(ind_en, ind_ar)| {
                                let ind_str = *ind_en;
                                let is_active = move || filter.get() == ind_str;
                                view! {
                                    <button
                                        type="button"
                                        role="tab"
                                        aria-selected=move || if is_active() { "true" } else { "false" }
                                        on:click=move |_| filter.set(ind_str)
                                        class=move || format!(
                                            "px-4 py-2 rounded-full text-sm font-medium transition-all cursor-pointer shadow-2xs focus:outline-none focus:ring-2 focus:ring-brand-500 {}",
                                            if is_active() {
                                                "bg-brand-600 text-white shadow-sm font-semibold"
                                            } else {
                                                "bg-white border border-slate-200 text-slate-600 hover:border-brand-300 hover:text-brand-700 hover:bg-slate-50"
                                            }
                                        )
                                    >
                                        {move || if locale_ctx.is_arabic() { *ind_ar } else { *ind_en }}
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        // Case Studies Grid
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            {move || {
                                let cur_filter = filter.get();
                                let filtered: Vec<&CaseStudy> = ALL_CASES.iter().filter(|c| cur_filter == "All" || c.industry_en == cur_filter).collect();

                                if filtered.is_empty() {
                                    view! {
                                        <div class="col-span-full py-16 text-center bg-white rounded-2xl border border-slate-200 p-8">
                                            <div class="w-12 h-12 rounded-full bg-slate-100 flex items-center justify-center mx-auto mb-4 text-slate-400">
                                                <Icon name="search".to_string() size=20 />
                                            </div>
                                            <h3 class="text-base font-semibold text-slate-900 mb-1">
                                                {if locale_ctx.is_arabic() { "لم يتم العثور على دراسات حالة" } else { "No case studies found" }}
                                            </h3>
                                            <p class="text-sm text-slate-500 mb-4">
                                                {if locale_ctx.is_arabic() { "لا توجد مشاريع تطابق هذا التصنيف حالياً." } else { "No projects currently match this industry filter." }}
                                            </p>
                                            <button
                                                type="button"
                                                on:click=move |_| filter.set("All")
                                                class="px-4 py-2 bg-brand-600 text-white text-sm font-medium rounded-lg hover:bg-brand-700 transition-colors"
                                            >
                                                {if locale_ctx.is_arabic() { "عرض كافة دراسات الحالة" } else { "Show All Case Studies" }}
                                            </button>
                                        </div>
                                    }.into_any()
                                } else {
                                    filtered.into_iter().map(|cs| {
                                        let cs_id = cs.id;
                                        view! {
                                            <div
                                                tabindex="0"
                                                role="button"
                                                aria-label=move || format!("View case study details: {}", if locale_ctx.is_arabic() { cs.title_ar } else { cs.title_en })
                                                on:click=move |_| selected_case_id.set(Some(cs_id))
                                                on:keydown=move |e: leptos::ev::KeyboardEvent| {
                                                    if e.key() == "Enter" || e.key() == " " {
                                                        e.prevent_default();
                                                        selected_case_id.set(Some(cs_id));
                                                    }
                                                }
                                                class="bg-white border border-slate-200 rounded-xl overflow-hidden hover:shadow-xl hover:border-brand-300 hover:-translate-y-0.5 transition-all duration-200 group cursor-pointer flex flex-col focus:outline-none focus:ring-2 focus:ring-brand-500 text-start"
                                            >
                                                <div class="relative h-48 bg-slate-100 overflow-hidden aspect-[16/9]">
                                                    <img
                                                        src=cs.image
                                                        alt=move || if locale_ctx.is_arabic() { cs.title_ar } else { cs.title_en }
                                                        loading="lazy"
                                                        class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                                                    />
                                                    <div class="absolute inset-0 bg-gradient-to-t from-slate-950/50 via-transparent to-transparent opacity-60 group-hover:opacity-40 transition-opacity"></div>
                                                    <div class="absolute top-3 start-3 flex gap-1.5 flex-wrap">
                                                        <Badge variant="navy".to_string()>{move || if locale_ctx.is_arabic() { cs.industry_ar } else { cs.industry_en }}</Badge>
                                                        <Badge variant="default".to_string()>{move || if locale_ctx.is_arabic() { cs.service_ar } else { cs.service_en }}</Badge>
                                                    </div>
                                                    <div class="absolute bottom-3 end-3">
                                                        <span class="inline-flex items-center gap-1 px-2 py-1 rounded-md bg-white/90 backdrop-blur-xs text-xs font-medium text-slate-800 shadow-xs">
                                                            <Icon name="clock".to_string() size=11 />
                                                            {move || if locale_ctx.is_arabic() { cs.duration_ar } else { cs.duration_en }}
                                                        </span>
                                                    </div>
                                                </div>

                                                <div class="p-6 flex-1 flex flex-col justify-between">
                                                    <div>
                                                        <p class="text-xs text-brand-600 font-semibold uppercase tracking-wider mb-1.5">
                                                            {move || if locale_ctx.is_arabic() { cs.client_ar } else { cs.client_en }}
                                                        </p>
                                                        <h2 class="text-base font-semibold text-slate-900 leading-snug mb-3 group-hover:text-brand-600 transition-colors">
                                                            {move || if locale_ctx.is_arabic() { cs.title_ar } else { cs.title_en }}
                                                        </h2>
                                                        <div class="flex items-start gap-2 p-3 bg-emerald-50/60 rounded-lg border border-emerald-100 mb-4">
                                                            <Icon name="trending".to_string() size=14 class="text-emerald-600 flex-shrink-0 mt-0.5".to_string() />
                                                            <p class="text-xs text-emerald-800 font-medium leading-relaxed">
                                                                {move || if locale_ctx.is_arabic() { cs.result_ar } else { cs.result_en }}
                                                            </p>
                                                        </div>
                                                    </div>

                                                    <div>
                                                        <div class="flex items-center justify-between pt-3 border-t border-slate-100">
                                                            <div class="flex flex-wrap gap-1">
                                                                {cs.tags.iter().map(|t| {
                                                                    view! {
                                                                        <Badge variant="neutral".to_string()>{*t}</Badge>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                            <span class="text-xs font-semibold text-brand-600 group-hover:translate-x-1 rtl:group-hover:-translate-x-1 transition-transform inline-flex items-center gap-1">
                                                                <span>{move || if locale_ctx.is_arabic() { "تفاصيل المشروع" } else { "Read Story" }}</span>
                                                                <Icon name="arrowRight".to_string() size=12 class="rtl:rotate-180".to_string() />
                                                            </span>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                    </div>
                </section>

                // Interactive Case Study Detail Modal
                {move || selected_case_id.get().and_then(|id| ALL_CASES.iter().find(|c| c.id == id)).map(|cs| {
                    view! {
                        <div
                            class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 bg-slate-950/70 backdrop-blur-xs animate-fade-in overflow-y-auto cursor-pointer"
                            role="dialog"
                            aria-modal="true"
                            aria-labelledby="case-modal-title"
                            on:click=move |_| selected_case_id.set(None)
                        >
                            <div
                                class="bg-white rounded-2xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto border border-slate-200 relative my-auto animate-scale-up cursor-default text-start"
                                on:click=move |e| e.stop_propagation()
                            >
                                // Close button
                                <button
                                    type="button"
                                    on:click=move |_| selected_case_id.set(None)
                                    class="absolute top-4 end-4 z-10 w-9 h-9 rounded-full bg-white/90 backdrop-blur-sm shadow-md flex items-center justify-center text-slate-500 hover:text-slate-900 hover:bg-white transition-colors cursor-pointer"
                                    aria-label="Close case study details"
                                >
                                    <Icon name="x".to_string() size=18 />
                                </button>

                                // Modal Header Media
                                <div class="relative h-56 sm:h-64 bg-slate-900 overflow-hidden rounded-t-2xl aspect-[16/9] sm:aspect-auto">
                                    <img
                                        src=cs.image
                                        alt=move || if locale_ctx.is_arabic() { cs.title_ar } else { cs.title_en }
                                        class="w-full h-full object-cover opacity-80"
                                    />
                                    <div class="absolute inset-0 bg-gradient-to-t from-slate-950/90 via-slate-950/40 to-transparent"></div>
                                    <div class="absolute bottom-5 start-5 end-5 text-white">
                                        <div class="flex gap-2 mb-2 flex-wrap">
                                            <Badge variant="navy".to_string()>{move || if locale_ctx.is_arabic() { cs.industry_ar } else { cs.industry_en }}</Badge>
                                            <Badge variant="default".to_string()>{move || if locale_ctx.is_arabic() { cs.service_ar } else { cs.service_en }}</Badge>
                                            <span class="text-xs bg-white/20 backdrop-blur-xs px-2 py-0.5 rounded text-white font-medium">
                                                {move || if locale_ctx.is_arabic() { format!("المدة: {}", cs.duration_ar) } else { format!("Duration: {}", cs.duration_en) }}
                                            </span>
                                        </div>
                                        <p class="text-xs text-brand-400 uppercase tracking-widest font-semibold">
                                            {move || if locale_ctx.is_arabic() { cs.client_ar } else { cs.client_en }}
                                        </p>
                                        <h2 id="case-modal-title" class="font-display text-xl sm:text-2xl font-bold text-white mt-1 leading-snug">
                                            {move || if locale_ctx.is_arabic() { cs.title_ar } else { cs.title_en }}
                                        </h2>
                                    </div>
                                </div>

                                // Modal Body
                                <div class="p-6 sm:p-8 space-y-6">
                                    // Primary Outcome Highlight
                                    <div class="p-4 rounded-xl bg-emerald-50 border border-emerald-200 flex items-start gap-3">
                                        <div class="w-8 h-8 rounded-lg bg-emerald-600 text-white flex items-center justify-center flex-shrink-0 mt-0.5">
                                            <Icon name="trending".to_string() size=18 />
                                        </div>
                                        <div>
                                            <p class="text-xs font-bold text-emerald-800 uppercase tracking-wider">
                                                {move || if locale_ctx.is_arabic() { "الإنجاز الأساسي" } else { "Primary Achievement" }}
                                            </p>
                                            <p class="text-sm font-semibold text-emerald-950 mt-0.5">
                                                {move || if locale_ctx.is_arabic() { cs.result_ar } else { cs.result_en }}
                                            </p>
                                        </div>
                                    </div>

                                    // Challenge & Solution Narrative
                                    <div class="grid grid-cols-1 gap-5">
                                        <div>
                                            <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-2 flex items-center gap-1.5">
                                                <Icon name="alertCircle".to_string() size=13 class="text-amber-500".to_string() />
                                                <span>{move || if locale_ctx.is_arabic() { "التحدي الهندسي" } else { "The Engineering Challenge" }}</span>
                                            </h3>
                                            <p class="text-sm text-slate-700 leading-relaxed bg-slate-50 p-4 rounded-xl border border-slate-200">
                                                {move || if locale_ctx.is_arabic() { cs.challenge_ar } else { cs.challenge_en }}
                                            </p>
                                        </div>

                                        <div>
                                            <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-2 flex items-center gap-1.5">
                                                <Icon name="checkCircle".to_string() size=13 class="text-brand-600".to_string() />
                                                <span>{move || if locale_ctx.is_arabic() { "الحل المعماري الهندسي" } else { "Our Architectural Solution" }}</span>
                                            </h3>
                                            <p class="text-sm text-slate-700 leading-relaxed bg-slate-50 p-4 rounded-xl border border-slate-200">
                                                {move || if locale_ctx.is_arabic() { cs.solution_ar } else { cs.solution_en }}
                                            </p>
                                        </div>
                                    </div>

                                    // Key Metrics List
                                    <div>
                                        <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-3">
                                            {move || if locale_ctx.is_arabic() { "النتائج ومؤشرات الأداء المحققة" } else { "Verified Business Outcomes" }}
                                        </h3>
                                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
                                            {move || {
                                                let metrics = if locale_ctx.is_arabic() { cs.key_metrics_ar } else { cs.key_metrics_en };
                                                metrics.iter().map(|metric| {
                                                    view! {
                                                        <div class="flex items-start gap-2 p-2.5 rounded-lg bg-slate-50 border border-slate-200 text-xs text-slate-800">
                                                            <Icon name="check".to_string() size=14 class="text-emerald-600 flex-shrink-0 mt-0.5".to_string() />
                                                            <span class="font-medium">{*metric}</span>
                                                        </div>
                                                    }
                                                }).collect_view()
                                            }}
                                        </div>
                                    </div>

                                    // Technologies
                                    <div>
                                        <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-2">
                                            {move || if locale_ctx.is_arabic() { "التقنيات المستخدمة" } else { "Technologies Utilized" }}
                                        </h3>
                                        <div class="flex flex-wrap gap-1.5">
                                            {cs.tags.iter().map(|t| {
                                                view! {
                                                    <span class="px-3 py-1 rounded-md bg-slate-100 text-slate-700 font-medium text-xs border border-slate-200">
                                                        {*t}
                                                    </span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>

                                    // Modal Footer Action
                                    <div class="pt-6 border-t border-slate-200 flex flex-col sm:flex-row items-center justify-between gap-3">
                                        <button
                                            type="button"
                                            on:click=move |_| selected_case_id.set(None)
                                            class="w-full sm:w-auto px-4 py-2.5 border border-slate-300 text-slate-700 text-sm font-medium rounded-lg hover:bg-slate-50 transition-colors"
                                        >
                                            {move || if locale_ctx.is_arabic() { "إغلاق" } else { "Close" }}
                                        </button>
                                        <a
                                            href="/quote"
                                            class="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-2.5 bg-brand-600 text-white text-sm font-semibold rounded-lg hover:bg-brand-700 transition-colors shadow-sm"
                                        >
                                            <span>{move || if locale_ctx.is_arabic() { "اطلب حلاً مماثلاً" } else { "Request Similar Solution" }}</span>
                                            <Icon name="arrowRight".to_string() size=14 class="rtl:rotate-180".to_string() />
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                })}

                // CTA
                <section class="bg-white border-t border-slate-200 py-16 text-center">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <h2 class="font-display text-3xl font-semibold text-slate-900 mb-3">
                            {move || if locale_ctx.is_arabic() { "مشروعك القادم يمكن أن يكون هنا" } else { "Your project could be next" }}
                        </h2>
                        <p class="text-slate-500 mb-8 max-w-md mx-auto leading-relaxed">
                            {move || if locale_ctx.is_arabic() {
                                "مستعد لبناء شيء استثنائي؟ شاركنا متطلباتك وسيقوم قادة الهندسة لدينا بإعداد مقترح شامل ومفصل."
                            } else {
                                "Ready to build something meaningful? Share your technical requirements and our engineering leads will prepare a comprehensive proposal."
                            }}
                        </p>
                        <div class="flex items-center justify-center gap-4 flex-wrap">
                            <a
                                href="/quote"
                                class="inline-flex items-center justify-center font-medium rounded-md text-base px-6 py-3 bg-brand-600 text-white hover:bg-brand-700 transition-colors gap-2 shadow-sm"
                            >
                                <span>{move || if locale_ctx.is_arabic() { "اطلب مقترح عمل" } else { "Get a Proposal" }}</span>
                                <Icon name="arrowRight".to_string() size=15 class="rtl:rotate-180".to_string() />
                            </a>
                            <a
                                href="/contact"
                                class="inline-flex items-center justify-center font-medium rounded-md text-base px-6 py-3 border border-slate-300 text-slate-700 hover:bg-slate-50 transition-colors"
                            >
                                {move || if locale_ctx.is_arabic() { "تحدث مع مهندس معمارية" } else { "Talk to an Architect" }}
                            </a>
                        </div>
                    </div>
                </section>
            </div>
        </PublicLayout>
    }
}
