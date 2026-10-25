//! # Complete Translation Dictionaries
//!
//! Provides comprehensive, strongly-typed, natural Arabic and English messages
//! across all website routes, landing sections, navigation, forms, modals, and error states.

use design_system::Locale;

pub struct NavT {
    pub services: &'static str,
    pub work: &'static str,
    pub about: &'static str,
    pub careers: &'static str,
    pub contact: &'static str,
    pub sign_in: &'static str,
    pub get_quote: &'static str,
    pub skip_content: &'static str,
}

pub fn nav(locale: Locale) -> NavT {
    match locale {
        Locale::En => NavT {
            services: "Services",
            work: "Work",
            about: "About",
            careers: "Careers",
            contact: "Contact",
            sign_in: "Sign In",
            get_quote: "Get a Quote",
            skip_content: "Skip to main content",
        },
        Locale::Ar => NavT {
            services: "الخدمات",
            work: "الأعمال",
            about: "عن تيب تك",
            careers: "الوظائف",
            contact: "تواصل معنا",
            sign_in: "تسجيل الدخول",
            get_quote: "طلب عرض أسعار",
            skip_content: "انتقل إلى المحتوى الرئيسي",
        },
    }
}

pub struct FooterT {
    pub tagline: &'static str,
    pub services_title: &'static str,
    pub industries_title: &'static str,
    pub company_title: &'static str,
    pub legal_title: &'static str,
    pub operational: &'static str,
    pub copyright: &'static str,
}

pub fn footer(locale: Locale) -> FooterT {
    match locale {
        Locale::En => FooterT {
            tagline: "Engineering enterprise-grade technology solutions that scale with your ambition. Trusted by 200+ organizations worldwide.",
            services_title: "Services",
            industries_title: "Industries",
            company_title: "Company",
            legal_title: "Legal & Compliance",
            operational: "All systems operational • 99.99% uptime",
            copyright: "© 2026 TeebTech Ltd. All rights reserved.",
        },
        Locale::Ar => FooterT {
            tagline: "هندسة حلول تقنية مؤسسية قابلة للتوسع مع طموحات أعمالكم. موثوقون من قبل أكثر من 200 مؤسسة حول العالم.",
            services_title: "الخدمات",
            industries_title: "القطاعات",
            company_title: "الشركة",
            legal_title: "الشؤون القانونية والامتثال",
            operational: "جميع الأنظمة تعمل بكفاءة • جاهزية 99.99%",
            copyright: "© 2026 شركة تيب تك المحدودة. جميع الحقوق محفوظة.",
        },
    }
}

pub struct StatsT {
    pub clients_val: &'static str,
    pub clients_lbl: &'static str,
    pub projects_val: &'static str,
    pub projects_lbl: &'static str,
    pub retention_val: &'static str,
    pub retention_lbl: &'static str,
    pub industries_val: &'static str,
    pub industries_lbl: &'static str,
}

pub fn stats(locale: Locale) -> StatsT {
    match locale {
        Locale::En => StatsT {
            clients_val: "200+",
            clients_lbl: "Clients worldwide",
            projects_val: "500+",
            projects_lbl: "Projects delivered",
            retention_val: "98%",
            retention_lbl: "Client retention rate",
            industries_val: "12+",
            industries_lbl: "Industries served",
        },
        Locale::Ar => StatsT {
            clients_val: "+200",
            clients_lbl: "عميل حول العالم",
            projects_val: "+500",
            projects_lbl: "مشروع منجز بنجاح",
            retention_val: "98%",
            retention_lbl: "معدل استبقاء العملاء",
            industries_val: "+12",
            industries_lbl: "قطاعاً نخدمه",
        },
    }
}

pub struct ProcessT {
    pub tag: &'static str,
    pub heading: &'static str,
    pub subtitle: &'static str,
    pub steps: [(&'static str, &'static str, &'static str); 5],
}

pub fn process(locale: Locale) -> ProcessT {
    match locale {
        Locale::En => ProcessT {
            tag: "Our Process",
            heading: "How We Deliver",
            subtitle: "A transparent, repeatable delivery framework that's served over 500 successful projects.",
            steps: [
                ("01", "Discover", "Deep-dive stakeholder workshops, technical audits, and competitive analysis to define the real problem."),
                ("02", "Plan", "Detailed scoping, architecture blueprints, resource allocation, and a transparent project roadmap."),
                ("03", "Build", "Agile sprints with weekly demos, continuous integration, and rigorous code review."),
                ("04", "Deploy", "Blue-green deployments, performance benchmarking, security hardening, and user acceptance testing."),
                ("05", "Support", "SLA-backed monitoring, proactive maintenance, training, and continuous improvement."),
            ],
        },
        Locale::Ar => ProcessT {
            tag: "منهجية العمل",
            heading: "كيف ننجز مشاريعنا الهندسية",
            subtitle: "إطار عمل شفاف وموثوق أثبت نجاحه في أكثر من 500 مشروع تقني عالي الأهمية.",
            steps: [
                ("01", "الاستكشاف والتحليل", "ورش عمل معمقة مع أصحاب المصلحة، وتدقيق للبنية الحالية لتحديد المتطلبات بدقة متناهية."),
                ("02", "التخطيط والهندسة", "تحديد نطاق العمل المفصل، ومخططات البنية التحتية، وخارطة طريق زمنية واضحة وشفافة."),
                ("03", "التطوير السريع", "دورات تطوير رشيقة (Agile) مع استعراض أسبوعي للإنجاز، ومراجعة صارمة للشيفرة البرمجية."),
                ("04", "النشر والتدشين", "إطلاق آمن بنظام النشر المزدوج، واختبارات ضغط وأداء، وتحصين أمني شامل قبل التدشين."),
                ("05", "الدعم والتحسين", "مراقبة مستمرة مدعومة باتفاقيات SLA، وصيانة استباقية وتطوير مستمر للأداء."),
            ],
        },
    }
}

pub struct CtaT {
    pub tag: &'static str,
    pub heading_prefix: &'static str,
    pub heading_highlight: &'static str,
    pub subtitle: &'static str,
    pub primary_btn: &'static str,
    pub secondary_btn: &'static str,
    pub guarantee: &'static str,
}

pub fn cta(locale: Locale) -> CtaT {
    match locale {
        Locale::En => CtaT {
            tag: "Ready to start?",
            heading_prefix: "Transform Your Technology. ",
            heading_highlight: "Today.",
            subtitle: "Get a free consultation with a senior engineer and receive a detailed proposal within 48 hours.",
            primary_btn: "Request a Proposal",
            secondary_btn: "Contact Sales",
            guarantee: "No commitment required. Response within 24 hours.",
        },
        Locale::Ar => CtaT {
            tag: "هل أنت مستعد للانطلاق؟",
            heading_prefix: "طوّر منظومتك التقنية ",
            heading_highlight: "اليوم.",
            subtitle: "احصل على استشارة هندسية متخصصة واستلم خطة مشروع تفصيلية وعرض أسعار خلال 48 ساعة.",
            primary_btn: "اطلب عرض أسعار مخصص",
            secondary_btn: "تحدث مع المبيعات",
            guarantee: "بدون أي التزام مسبق. رد موثوق خلال 24 ساعة.",
        },
    }
}

pub struct NotFoundT {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub description: &'static str,
    pub back_home: &'static str,
    pub browse_services: &'static str,
    pub contact_support: &'static str,
}

pub fn not_found(locale: Locale) -> NotFoundT {
    match locale {
        Locale::En => NotFoundT {
            title: "Error 404",
            subtitle: "Page not found",
            description: "The page you are looking for might have been moved, renamed, or is temporarily unavailable. Let us help you get back on track.",
            back_home: "Return to Home",
            browse_services: "Browse Services",
            contact_support: "Contact Support",
        },
        Locale::Ar => NotFoundT {
            title: "خطأ 404",
            subtitle: "الصفحة غير موجودة",
            description: "الصفحة التي تبحث عنها ربما تم نقلها أو تغيير اسمها أو أنها غير متاحة حالياً. يسعدنا مساعدتك للوصول إلى وجهتك.",
            back_home: "العودة للرئيسية",
            browse_services: "استعراض الخدمات",
            contact_support: "التواصل مع الدعم",
        },
    }
}
