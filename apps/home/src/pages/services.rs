//! # Services Page
//!
//! Matches UI reference Services.tsx with full Arabic and English support:
//! Hero heading, interactive category navigation (horizontal swipe on mobile, sticky sidebar on desktop),
//! URL hash detection, capability checklists, tech badges, CTAs, and discovery call booking.

use crate::i18n::use_locale;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use ui::components::Icon;
use ui::layout::PublicLayout;

struct ServiceCategory {
    id: &'static str,
    icon: &'static str,
    title_en: &'static str,
    title_ar: &'static str,
    tagline_en: &'static str,
    tagline_ar: &'static str,
    desc_en: &'static str,
    desc_ar: &'static str,
    capabilities_en: &'static [&'static str],
    capabilities_ar: &'static [&'static str],
    tech: &'static [&'static str],
}

const SERVICE_CATEGORIES: &[ServiceCategory] = &[
    ServiceCategory {
        id: "software",
        icon: "code",
        title_en: "Software Development",
        title_ar: "تطوير البرمجيات",
        tagline_en: "Web, mobile, and enterprise applications built to scale.",
        tagline_ar: "تطبيقات الويب والهواتف والأنظمة المؤسسية المصممة للنمو والتوسع.",
        desc_en: "We design and engineer software that solves real business problems — from greenfield platforms to complex legacy modernizations. Our full-stack teams work in React, Next.js, Rust, Go, Python, and more.",
        desc_ar: "نصمم ونطور برمجيات متقدمة تحل تحديات الأعمال الفعلية — من المنصات الجديدة كلياً إلى تحديث الأنظمة القديمة المعقدة. فرقنا المتكاملة تتقن أحدث اللغات وأطر العمل.",
        capabilities_en: &[
            "Web Applications & SaaS Platforms",
            "Mobile Applications (iOS & Android)",
            "API Design & Enterprise Integration",
            "Microservices & Event-Driven Systems",
            "Legacy Monolith Modernization",
            "Real-time Collaboration & Streaming",
        ],
        capabilities_ar: &[
            "تطبيقات الويب ومنصات SaaS السحابية",
            "تطبيقات الهواتف الذكية (iOS و Android)",
            "تصميم واجهات برمجة التطبيقات والتكامل المؤسسي",
            "بنية الخدمات المصغرة والأنظمة الموجهة بالأحداث",
            "تحديث وتفكيك الأنظمة القديمة الضخمة",
            "أنظمة التعاون اللحظي والبث المباشر للبيانات",
        ],
        tech: &["React", "Next.js", "Node.js", "Rust", "Go", "Python", "PostgreSQL", "Redis"],
    },
    ServiceCategory {
        id: "cloud",
        icon: "server",
        title_en: "Cloud & DevOps",
        title_ar: "السحابة وهندسة العمليات",
        tagline_en: "Resilient, cost-optimized infrastructure at any scale.",
        tagline_ar: "بنية تحتية سحابية فائقة المرونة ومحسّنة التكاليف لأي حجم عمل.",
        desc_en: "We design multi-cloud architectures, automate deployments, and implement platform engineering practices that let your engineering teams ship faster and more reliably.",
        desc_ar: "نصمم معماريات سحابية متعددة ونؤتمت عمليات النشر والتدشين، ونطبق أفضل ممارسات هندسة المنصات لتسريع إطلاق الميزات بأعلى درجات الموثوقية.",
        capabilities_en: &[
            "AWS / Azure / GCP Multi-Cloud Architecture",
            "Kubernetes & Container Orchestration",
            "Automated CI/CD Pipeline Engineering",
            "Infrastructure as Code (Terraform, Pulumi)",
            "FinOps & Cloud Cost Optimization",
            "Disaster Recovery & Multi-Region Failover",
        ],
        capabilities_ar: &[
            "معماريات السحابة المتعددة AWS / Azure / GCP",
            "إدارة وتنظيم الحاويات عبر Kubernetes",
            "هندسة مسارات CI/CD المؤتمتة بالكامل",
            "إدارة البنية التحتية كشيفرة برمجية (IaC)",
            "إدارة التكاليف السحابية وتحسين الكفاءة FinOps",
            "التعافي من الكوارث والتحويل التلقائي بين المناطق",
        ],
        tech: &["AWS", "Azure", "GCP", "Kubernetes", "Terraform", "Helm", "ArgoCD", "Datadog"],
    },
    ServiceCategory {
        id: "security",
        icon: "shield",
        title_en: "Cybersecurity",
        title_ar: "الأمن السيبراني",
        tagline_en: "Proactive security across every layer of your stack.",
        tagline_ar: "حماية استباقية شاملة عبر كل طبقة من طبقات منظومتك التقنية.",
        desc_en: "From penetration testing to 24/7 SIEM implementation, we help enterprises construct proactive security programs that protect valuable assets, satisfy regulators, and foster customer trust.",
        desc_ar: "من اختبارات الاختراق إلى مراكز العمليات الأمنية المتكاملة، نساعد المؤسسات على بناء برامج أمنية متينة تحمي الأصول الحيوية وتلبي متطلبات الامتثال الصارمة.",
        capabilities_en: &[
            "Penetration Testing & Red Teaming",
            "SOC Design & SIEM Implementation",
            "Zero Trust Architecture Implementation",
            "ISO 27001, SOC 2, & HIPAA Compliance",
            "Continuous Vulnerability Management",
            "Incident Response & Forensics Retainers",
        ],
        capabilities_ar: &[
            "اختبارات الاختراق ومحاكاة الهجمات (Red Teaming)",
            "تصميم مراكز العمليات الأمنية وبناء أنظمة SIEM",
            "تطبيق معمارية الثقة الصفرية Zero-Trust",
            "الامتثال لمعايير ISO 27001 و SOC 2 و HIPAA",
            "الإدارة المستمرة للثغرات وتقييم المخاطر",
            "الاستجابة للحوادث الأمنية والتحقيق الجنائي الرقمي",
        ],
        tech: &["Splunk", "CrowdStrike", "Tenable", "Burp Suite", "OpenVAS", "AWS Security Hub"],
    },
    ServiceCategory {
        id: "ai",
        icon: "cpu",
        title_en: "AI & Data Engineering",
        title_ar: "الذكاء الاصطناعي وهندسة البيانات",
        tagline_en: "From raw data to intelligent, actionable systems.",
        tagline_ar: "تحويل البيانات الخام إلى أنظمة ذكية تدعم اتخاذ القرارات.",
        desc_en: "We build modern data foundations and production ML platforms that turn organizational data into tangible competitive advantage — without the hype, with measurable outcomes.",
        desc_ar: "نبني قواعد بيانات حديثة ومنصات تعلم آلة إنتاجية تحول بيانات المؤسسة إلى ميزة تنافسية ملموسة ونتائج قابلة للقياس الدقيق.",
        capabilities_en: &[
            "Data Warehouse & Lakehouse Design",
            "Predictive ML Model Development",
            "LLM Integration & Domain Fine-tuning",
            "Executive Business Intelligence & Dashboards",
            "Real-time Streaming Analytics",
            "MLOps & Model Governance Lifecycles",
        ],
        capabilities_ar: &[
            "تصميم مستودعات وبحيرات البيانات الضخمة",
            "تطوير نماذج التنبؤ وتطبيقات تعلم الآلة",
            "دمج وتخصيص نماذج الذكاء الاصطناعي التوليدي LLMs",
            "لوحات ذكاء الأعمال والتحليلات التنفيذية",
            "تحليلات البث اللحظي للبيانات الضخمة",
            "عمليات MLOps وإدارة دورة حياة النماذج الذكية",
        ],
        tech: &["Python", "Spark", "dbt", "Airflow", "TensorFlow", "PyTorch", "Snowflake", "Databricks"],
    },
    ServiceCategory {
        id: "consulting",
        icon: "briefcase",
        title_en: "IT Consulting",
        title_ar: "الاستشارات التقنية",
        tagline_en: "Strategic technology advisory grounded in execution reality.",
        tagline_ar: "استشارات استراتيجية للتكنولوجيا مبنية على واقع وقابلية التنفيذ.",
        desc_en: "Our consulting engagements go beyond abstract slide decks. Every advisory recommendation is backed by engineering feasibility analysis, clear roadmaps, and a realistic path to execution.",
        desc_ar: "تتجاوز استشاراتنا مجرد العروض النظرية؛ فكل توصية ندعمها بدراسة جدوى هندسية دقيقة وخارطة طريق تنفيذية واضحة ومسار محدد للتطبيق.",
        capabilities_en: &[
            "Digital Transformation Roadmaps",
            "Architecture Review & Technical Audits",
            "Enterprise Vendor Evaluation & Selection",
            "Technology Due Diligence for M&A",
            "IT Governance & Risk Frameworks",
            "Staff Augmentation & Agile Coaching",
        ],
        capabilities_ar: &[
            "خطط وخارطة طريق التحول الرقمي الشامل",
            "مراجعة المعمارية والتدقيق الهندسي التقني",
            "تقييم واختيار مزودي الحلول والبرمجيات",
            "الفحص التقني النافي للجهالة لصفقات الاستحواذ",
            "أطر الحوكمة وإدارة المخاطر التقنية",
            "تعزيز الفرق الهندسية والتدريب على منهجيات Agile",
        ],
        tech: &["TOGAF", "ITIL", "COBIT", "Agile", "SAFe", "Gartner Frameworks"],
    },
    ServiceCategory {
        id: "managed",
        icon: "settings",
        title_en: "Managed IT Services",
        title_ar: "خدمات الدعم المدارة",
        tagline_en: "24/7 proactive monitoring and support with guaranteed SLAs.",
        tagline_ar: "مراقبة استباقية ودعم فني على مدار الساعة مع ضمانات SLA موثوقة.",
        desc_en: "Let us monitor, maintain, and optimize your technology infrastructure around the clock while your internal teams focus exclusively on core revenue-generating business initiatives.",
        desc_ar: "نتولى مراقبة وصيانة وتحسين بنيتك التقنية على مدار الساعة بينما تتفرغ فرقك الداخلية للتركيز على المبادرات الأساسية وتنمية الأعمال.",
        capabilities_en: &[
            "24/7/365 NOC & SOC Proactive Monitoring",
            "Tier 1-3 Engineering Helpdesk Support",
            "Automated Patch & Firmware Management",
            "Continuous Backup Verification & DR Testing",
            "Microsoft 365 & Google Workspace Admin",
            "Corporate Network & SD-WAN Management",
        ],
        capabilities_ar: &[
            "مراقبة مستمرة 24/7/365 عبر مراكز NOC و SOC",
            "دعم فني هندسي متخصص من المستوى 1 إلى 3",
            "الإدارة المؤتمتة للتحديثات والترقيعات الأمنية",
            "التحقق الدوري من النسخ الاحتياطية واختبار التعافي",
            "إدارة سحابية لـ Microsoft 365 و Google Workspace",
            "إدارة شبكات الشركات وتقنيات SD-WAN المتقدمة",
        ],
        tech: &["Zabbix", "PagerDuty", "ConnectWise", "ManageEngine", "Veeam", "Cisco"],
    },
    ServiceCategory {
        id: "erp",
        icon: "database",
        title_en: "ERP & CRM Solutions",
        title_ar: "أنظمة ERP و CRM",
        tagline_en: "Enterprise systems that unify operations, finance, and customer workflows.",
        tagline_ar: "أنظمة مؤسسية توحد مسارات العمل والمالية والعمليات وإدارة العملاء.",
        desc_en: "We architect, integrate, and customize world-class ERP and CRM ecosystems — including SAP S/4HANA, Salesforce, Microsoft Dynamics 365, and Odoo — reducing manual handoffs and unlocking end-to-end operational visibility.",
        desc_ar: "نصمم وندمج ونخصص أحدث أنظمة ERP و CRM العالمية — بما فيها SAP و Salesforce و Dynamics 365 و Odoo — للقضاء على العمل اليدوي وتوفير رؤية تشغيلية شاملة.",
        capabilities_en: &[
            "SAP S/4HANA Implementation & Migration",
            "Salesforce Customization & Flow Automation",
            "Microsoft Dynamics 365 Enterprise Rollouts",
            "Odoo Open-Source Business Apps Deployment",
            "Legacy ERP Data Migration & Cleanse",
            "Custom Middleware & Real-time Integration",
        ],
        capabilities_ar: &[
            "تطبيق وترحيل البيانات إلى SAP S/4HANA",
            "تخصيص Salesforce وأتمتة مسارات العمل",
            "إطلاق ونشر Microsoft Dynamics 365 للشركات",
            "تطبيق وتطوير تطبيقات Odoo مفتوحة المصدر",
            "ترحيل وتنقية بيانات أنظمة ERP القديمة",
            "بناء البرمجيات الوسيطة والتكامل اللحظي بين الأنظمة",
        ],
        tech: &["SAP S/4HANA", "Salesforce", "Dynamics 365", "Odoo", "MuleSoft", "PostgreSQL", "Apache Kafka"],
    },
    ServiceCategory {
        id: "design",
        icon: "layers",
        title_en: "UI/UX Design Systems",
        title_ar: "تصميم واجهات المستخدم",
        tagline_en: "Research-driven, accessibility-first interfaces that delight users.",
        tagline_ar: "واجهات سهلة الاستخدام مبنية على أبحاث دقيقة وموجهة لرضا المستخدم.",
        desc_en: "Our design team transforms complex enterprise software workflows into intuitive, elegant digital products. We build scalable design systems in Figma that maintain brand consistency across web, mobile, and desktop touchpoints.",
        desc_ar: "يحول فريق التصميم لدينا مسارات العمل البرمجية المعقدة إلى منتجات رقمية أنيقة وبديهية، مع بناء أنظمة تصميم قابلة للتوسع تحافظ على هوية علامتك التجارية.",
        capabilities_en: &[
            "User Research & Journey Mapping",
            "Multi-Platform Enterprise Design Systems",
            "High-Fidelity Prototyping & Usability Testing",
            "WCAG 2.2 AA Accessibility Auditing",
            "Product Redesign & Information Architecture",
            "Design-to-Code Engineering Hand-off",
        ],
        capabilities_ar: &[
            "أبحاث تجربة المستخدم ورسم رحلة العميل",
            "بناء أنظمة التصميم الشاملة للمنصات المتعددة",
            "نماذج تفاعلية عالية الدقة واختبارات قابلية الاستخدام",
            "تدقيق معايير سهولة الوصول WCAG 2.2 AA",
            "إعادة تصميم المنتجات وهيكلة هندسة المعلومات",
            "تسليم التصاميم البرمجية بدقة عالية للفرق الهندسية",
        ],
        tech: &["Figma", "Storybook", "Tailwind CSS", "Tokens Studio", "Zeroheight", "Lottie"],
    },
    ServiceCategory {
        id: "marketing",
        icon: "trending",
        title_en: "Digital Marketing & SEO",
        title_ar: "التسويق الرقمي و SEO",
        tagline_en: "Data-backed technical growth and conversion rate optimization.",
        tagline_ar: "نمو تقني قائم على البيانات وتحسين مستمر لمعدلات التحويل.",
        desc_en: "We combine deep engineering rigor with growth marketing — delivering technical SEO architecture, server-side tracking, performance marketing funnels, and marketing automation systems that drive sustainable pipeline.",
        desc_ar: "ندمج الدقة الهندسية مع استراتيجيات النمو التسويقي — لنقدم معمارية SEO فنية، وتتبعاً دقيقاً من جانب الخادم، وحملات تحويل تدعم استدامة المبيعات.",
        capabilities_en: &[
            "Technical SEO Auditing & Core Web Vitals",
            "Server-Side Google Analytics 4 & Tag Management",
            "High-Intent Performance PPC & Retargeting",
            "HubSpot & Marketo Marketing Automation",
            "Conversion Rate Optimization (CRO) & A/B Testing",
            "B2B Content Strategy & Thought Leadership",
        ],
        capabilities_ar: &[
            "تدقيق SEO التقني وتحسين مؤشرات Core Web Vitals",
            "إعداد التتبع السحابي في GA4 وإدارة الوسوم",
            "حملات الإعلانات الممولة PPC وإعادة الاستهداف",
            "أتمتة التسويق عبر HubSpot و Marketo",
            "تحسين معدلات التحويل (CRO) واختبارات A/B",
            "استراتيجيات المحتوى التخصصي لقطاع الأعمال B2B",
        ],
        tech: &["Google Analytics 4", "Search Console", "HubSpot", "Semrush", "Ahrefs", "Google Tag Manager"],
    },
    ServiceCategory {
        id: "qa",
        icon: "activity",
        title_en: "QA & Testing Automation",
        title_ar: "ضبط الجودة وأتمتة الاختبارات",
        tagline_en: "Zero-defect confidence from automated CI/CD pipeline tests.",
        tagline_ar: "ثقة مطلقة في خلو البرمجيات من العيوب عبر الاختبارات المؤتمتة.",
        desc_en: "We implement continuous testing frameworks, end-to-end automation, and rigorous performance testing pipelines that catch bugs early, accelerate release velocity, and protect production stability.",
        desc_ar: "نطبق أطر اختبار مستمرة وأتمتة شاملة لكافة المسارات واختبارات أداء صارمة ترصد الأخطاء مبكراً وتسرع وتيرة الإطلاق وتحمي استقرار البيئة الإنتاجية.",
        capabilities_en: &[
            "End-to-End Automated Test Suite Engineering",
            "API Contract & Integration Testing",
            "Load, Stress & Chaos Engineering Testing",
            "Cross-Browser & Mobile Device Cloud Testing",
            "Security Regression & Static Analysis (SAST)",
            "Continuous QA Integration in CI/CD Pipelines",
        ],
        capabilities_ar: &[
            "هندسة حزم الاختبار المؤتمتة من البداية إلى النهاية E2E",
            "اختبار عقود واجهات برمجة التطبيقات والتكامل",
            "اختبارات التحمل والضغط وهندسة الفوضى Chaos Testing",
            "اختبار التوافق عبر المتصفحات والأجهزة السحابية",
            "الفحص الأمني للشيفرة البرمجية والتحليل الساكن SAST",
            "الدمج المستمر لضبط الجودة ضمن مسارات CI/CD",
        ],
        tech: &["Playwright", "Cypress", "Selenium", "k6", "Postman", "JUnit", "SonarQube"],
    },
];

#[component]
pub fn ServicesPage() -> impl IntoView {
    let locale_ctx = use_locale();
    let active_id = RwSignal::new("software");

    // Check hash on hydration and hash change events
    #[cfg(feature = "hydrate")]
    {
        let check_hash = move || {
            if let Some(window) = web_sys::window() {
                if let Ok(hash) = window.location().hash() {
                    let trimmed_hash = hash.trim_start_matches('#');
                    if let Some(cat) = SERVICE_CATEGORIES.iter().find(|s| s.id == trimmed_hash) {
                        active_id.set(cat.id);
                    }
                }
            }
        };

        check_hash();

        let _ = window_event_listener(leptos::ev::hashchange, move |_| {
            check_hash();
        });
    }

    view! {
        <Title text=move || if locale_ctx.is_arabic() {
            "الخدمات التقنية والهندسية المؤسسية — تيب تك"
        } else {
            "Enterprise Technology Services — TeebTech"
        } />
        <Meta name="description" content=move || if locale_ctx.is_arabic() {
            "خدمات تقنية متكاملة: تطوير البرمجيات، السحابة والعمليات، الأمن السيبراني، الذكاء الاصطناعي، الاستشارات، الخدمات المدارة، ERP، وتصميم الواجهات."
        } else {
            "End-to-end technology services: Custom software development, cloud DevOps, cybersecurity, AI data engineering, IT consulting, managed IT, ERP, UI/UX design, digital marketing, and QA automation."
        } />

        <PublicLayout current_page="services".to_string()>
            <div>
                // Hero
                <section class="bg-white border-b border-slate-200 py-14 lg:py-18">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8 text-start">
                        <div class="max-w-2xl">
                            <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-3">
                                {move || if locale_ctx.is_arabic() { "قدراتنا وإمكاناتنا" } else { "Our Capabilities" }}
                            </p>
                            <h1 class="font-display text-4xl lg:text-5xl font-semibold text-slate-900 leading-tight mb-4">
                                {move || if locale_ctx.is_arabic() {
                                    "كل ما تحتاجه منظومتك التقنية، تحت سقف واحد."
                                } else {
                                    "Everything your technology needs, under one roof."
                                }}
                            </h1>
                            <p class="text-slate-500 text-base lg:text-lg leading-relaxed">
                                {move || if locale_ctx.is_arabic() {
                                    "من المعمارية الهندسية وتطوير البرمجيات إلى التشغيل المستمر على مدار الساعة — تقدم تيب تك خدمات شاملة تحقق نتائج أعمال ملموسة."
                                } else {
                                    "From system architecture and full-stack engineering to ongoing 24/7 operations — TeebTech provides end-to-end technology services that deliver measurable business outcomes."
                                }}
                            </p>
                        </div>
                    </div>
                </section>

                // Services interactive section
                <section class="bg-slate-50 py-12 lg:py-16">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        // Responsive Layout: horizontal scrolling chip tabs on mobile, sticky sidebar on desktop
                        <div class="flex flex-col lg:flex-row gap-8 items-start">
                            // Navigation Tabs / Sidebar
                            <div class="w-full lg:w-72 flex-shrink-0 text-start">
                                <div class="lg:sticky lg:top-24">
                                    <h2 class="text-xs font-bold text-slate-400 uppercase tracking-wider mb-3 px-1">
                                        {move || if locale_ctx.is_arabic() { "مجالات الخدمات" } else { "Service Areas" }}
                                    </h2>
                                    
                                    <nav
                                        class="flex flex-row overflow-x-auto pb-2 lg:pb-0 lg:flex-col gap-1.5 no-scrollbar"
                                        role="tablist"
                                        aria-label="Service categories"
                                    >
                                        {SERVICE_CATEGORIES.iter().map(|s| {
                                            let s_id = s.id;
                                            let is_active = move || active_id.get() == s_id;
                                            view! {
                                                <button
                                                    type="button"
                                                    role="tab"
                                                    aria-selected=move || if is_active() { "true" } else { "false" }
                                                    aria-controls=format!("service-panel-{}", s.id)
                                                    on:click=move |_| active_id.set(s_id)
                                                    class=move || format!(
                                                        "flex-shrink-0 flex items-center gap-2.5 px-3.5 py-2.5 rounded-xl text-sm font-medium transition-all text-start cursor-pointer focus:outline-none focus:ring-2 focus:ring-brand-500 {}",
                                                        if is_active() {
                                                            "bg-brand-600 text-white shadow-sm font-semibold"
                                                        } else {
                                                            "bg-white border border-slate-200 text-slate-700 hover:bg-slate-100 hover:text-slate-900 lg:border-transparent lg:bg-transparent"
                                                        }
                                                    )
                                                >
                                                    <Icon name=s.icon.to_string() size=16 class="flex-shrink-0".to_string() />
                                                    <span class="whitespace-nowrap">
                                                        {move || if locale_ctx.is_arabic() { s.title_ar } else { s.title_en }}
                                                    </span>
                                                </button>
                                            }
                                        }).collect_view()}
                                    </nav>
                                </div>
                            </div>

                            // Detail Card
                            <div class="flex-1 w-full text-start">
                                {move || {
                                    let current_id = active_id.get();
                                    let svc = SERVICE_CATEGORIES.iter().find(|s| s.id == current_id).unwrap_or(&SERVICE_CATEGORIES[0]);
                                    view! {
                                        <div
                                            id=format!("service-panel-{}", svc.id)
                                            role="tabpanel"
                                            class="bg-white border border-slate-200 rounded-2xl p-6 sm:p-8 lg:p-10 shadow-sm animate-fade-in"
                                        >
                                            <div class="flex flex-col sm:flex-row sm:items-center gap-4 mb-6 pb-6 border-b border-slate-100">
                                                <div class="w-14 h-14 bg-brand-50 rounded-2xl flex items-center justify-center text-brand-600 flex-shrink-0 shadow-xs">
                                                    <Icon name=svc.icon.to_string() size=26 />
                                                </div>
                                                <div>
                                                    <h3 class="font-display text-2xl sm:text-3xl font-bold text-slate-900 leading-tight">
                                                        {move || if locale_ctx.is_arabic() { svc.title_ar } else { svc.title_en }}
                                                    </h3>
                                                    <p class="text-sm sm:text-base text-brand-600 font-medium mt-1">
                                                        {move || if locale_ctx.is_arabic() { svc.tagline_ar } else { svc.tagline_en }}
                                                    </p>
                                                </div>
                                            </div>

                                            <p class="text-slate-600 text-base leading-relaxed mb-8">
                                                {move || if locale_ctx.is_arabic() { svc.desc_ar } else { svc.desc_en }}
                                            </p>

                                            <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-8">
                                                <div class="bg-slate-50 p-5 rounded-xl border border-slate-200">
                                                    <h4 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-4 flex items-center gap-2">
                                                        <Icon name="checkCircle".to_string() size=14 class="text-brand-600".to_string() />
                                                        <span>{move || if locale_ctx.is_arabic() { "القدرات الأساسية" } else { "Core Capabilities" }}</span>
                                                    </h4>
                                                    <ul class="space-y-3">
                                                        {move || {
                                                            let caps = if locale_ctx.is_arabic() { svc.capabilities_ar } else { svc.capabilities_en };
                                                            caps.iter().map(|cap| {
                                                                view! {
                                                                    <li class="flex items-start gap-2.5 text-sm text-slate-700">
                                                                        <Icon name="check".to_string() size=14 class="text-emerald-500 flex-shrink-0 mt-0.5".to_string() />
                                                                        <span>{*cap}</span>
                                                                    </li>
                                                                }
                                                            }).collect_view()
                                                        }}
                                                    </ul>
                                                </div>

                                                <div class="bg-slate-50 p-5 rounded-xl border border-slate-200">
                                                    <h4 class="text-xs font-bold text-slate-500 uppercase tracking-wider mb-4 flex items-center gap-2">
                                                        <Icon name="code".to_string() size=14 class="text-brand-600".to_string() />
                                                        <span>{move || if locale_ctx.is_arabic() { "التقنيات المستخدمة" } else { "Featured Tech Stack" }}</span>
                                                    </h4>
                                                    <div class="flex flex-wrap gap-2">
                                                        {svc.tech.iter().map(|t| {
                                                            view! {
                                                                <span class="px-3 py-1.5 bg-white rounded-lg text-xs font-medium text-slate-700 border border-slate-200 shadow-2xs">
                                                                    {*t}
                                                                </span>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="pt-6 border-t border-slate-100 flex flex-col sm:flex-row items-stretch sm:items-center gap-3">
                                                <a
                                                    href="/quote"
                                                    class="inline-flex items-center justify-center font-medium rounded-lg text-sm px-5 py-3 bg-brand-600 text-white hover:bg-brand-700 transition-colors gap-2 shadow-sm"
                                                >
                                                    <span>{move || if locale_ctx.is_arabic() { "اطلب عرض أسعار مخصص" } else { "Request a Proposal" }}</span>
                                                    <Icon name="arrowRight".to_string() size=14 class="rtl:rotate-180".to_string() />
                                                </a>
                                                <a
                                                    href="/case-studies"
                                                    class="inline-flex items-center justify-center font-medium rounded-lg text-sm px-5 py-3 border border-slate-300 text-slate-700 bg-white hover:bg-slate-50 transition-colors"
                                                >
                                                    {move || if locale_ctx.is_arabic() { "استكشف دراسات الحالة ذات الصلة" } else { "Explore Related Case Studies" }}
                                                </a>
                                            </div>
                                        </div>
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </section>

                // Bottom Discovery CTA
                <section class="bg-white py-16 border-t border-slate-200 text-center">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <p class="text-xs font-semibold text-brand-600 uppercase tracking-widest mb-3">
                            {move || if locale_ctx.is_arabic() { "هل أنت مستعد للاستكشاف؟" } else { "Ready to explore?" }}
                        </p>
                        <h2 class="font-display text-3xl font-semibold text-slate-900 mb-3">
                            {move || if locale_ctx.is_arabic() { "لست متأكداً من أين تبدأ؟" } else { "Not sure where to begin?" }}
                        </h2>
                        <p class="text-slate-500 max-w-lg mx-auto mb-8 leading-relaxed">
                            {move || if locale_ctx.is_arabic() {
                                "احجز جلسة استكشافية مجانية لمدة 45 دقيقة مع كبير مهندسي تيب تك — مناقشة تقنية عملية لا تخضع لأي ضغوط بيعية لمراجعة متطلباتك."
                            } else {
                                "Book a free 45-minute discovery call with a senior TeebTech engineering lead — no sales pressure, just a practical technical discussion about your requirements."
                            }}
                        </p>
                        <div class="flex items-center justify-center gap-4 flex-wrap">
                            <a
                                href="/contact"
                                class="inline-flex items-center justify-center font-medium rounded-md text-base px-6 py-3 bg-brand-600 text-white hover:bg-brand-700 transition-colors gap-2 shadow-sm"
                            >
                                <span>{move || if locale_ctx.is_arabic() { "احجز جلسة استكشافية" } else { "Book Discovery Call" }}</span>
                                <Icon name="arrowRight".to_string() size=16 class="rtl:rotate-180".to_string() />
                            </a>
                            <a
                                href="/about"
                                class="inline-flex items-center justify-center font-medium rounded-md text-base px-6 py-3 border border-slate-300 text-slate-700 hover:bg-slate-50 transition-colors"
                            >
                                {move || if locale_ctx.is_arabic() { "تعرف على فريقنا" } else { "Learn About Our Team" }}
                            </a>
                        </div>
                    </div>
                </section>
            </div>
        </PublicLayout>
    }
}
