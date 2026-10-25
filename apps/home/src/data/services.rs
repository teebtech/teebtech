use design_system::Locale;

#[derive(Clone, Copy)]
pub struct ServiceItem {
    pub icon: &'static str,
    pub title: &'static str,
    pub title_ar: &'static str,
    pub desc: &'static str,
    pub desc_ar: &'static str,
}

impl ServiceItem {
    pub fn localized_title(&self, locale: Locale) -> &'static str {
        if locale.is_arabic() {
            self.title_ar
        } else {
            self.title
        }
    }

    pub fn localized_desc(&self, locale: Locale) -> &'static str {
        if locale.is_arabic() {
            self.desc_ar
        } else {
            self.desc
        }
    }
}

pub fn get_services() -> Vec<ServiceItem> {
    vec![
        ServiceItem {
            icon: "code",
            title: "Software Development",
            title_ar: "تطوير البرمجيات",
            desc: "Web, mobile, and enterprise applications built with modern stacks and best-in-class architecture.",
            desc_ar: "تطبيقات الويب والموبايل والأنظمة المؤسسية بأحدث المعماريات الهندسية المتقدمة.",
        },
        ServiceItem {
            icon: "server",
            title: "Cloud & DevOps",
            title_ar: "السحابة وهندسة العمليات",
            desc: "AWS, Azure, GCP architecture, containerization, CI/CD pipelines, and infrastructure as code.",
            desc_ar: "معماريات AWS و Azure و GCP والحاويات وخطوط CI/CD وإدارة البنية التحتية كشيفرة برمجية.",
        },
        ServiceItem {
            icon: "shield",
            title: "Cybersecurity",
            title_ar: "الأمن السيبراني",
            desc: "Penetration testing, compliance frameworks, SOC monitoring, and zero-trust implementation.",
            desc_ar: "اختبارات الاختراق وأطر الامتثال ومراقبة مراكز العمليات SOC وبنية الثقة الصفرية.",
        },
        ServiceItem {
            icon: "cpu",
            title: "AI & Data Engineering",
            title_ar: "الذكاء الاصطناعي والبيانات",
            desc: "Machine learning models, data pipelines, business intelligence, and intelligent automation.",
            desc_ar: "نماذج تعلم الآلة وخطوط معالجة البيانات الضخمة وذكاء الأعمال والأتمتة الذكية.",
        },
        ServiceItem {
            icon: "briefcase",
            title: "IT Consulting",
            title_ar: "الاستشارات التقنية",
            desc: "Strategic technology advisory, digital transformation roadmaps, and vendor selection.",
            desc_ar: "استشارات استراتيجية للتكنولوجيا وخطط التحول الرقمي الشامل واختيار الموردين الأنسب.",
        },
        ServiceItem {
            icon: "settings",
            title: "Managed IT Services",
            title_ar: "خدمات الدعم المدارة",
            desc: "24/7 infrastructure monitoring, helpdesk support, patch management, and SLA guarantees.",
            desc_ar: "مراقبة مستمرة للبنية التحتية على مدار الساعة ودعم فني متخصص وضمانات SLA موثوقة.",
        },
        ServiceItem {
            icon: "database",
            title: "ERP & CRM",
            title_ar: "أنظمة ERP و CRM",
            desc: "SAP, Salesforce, Odoo, and Dynamics implementations, migrations, and custom modules.",
            desc_ar: "تطبيق وتكامل حلول SAP و Salesforce و Odoo و Dynamics مع ترحيل البيانات وتطوير الوحدات.",
        },
        ServiceItem {
            icon: "layers",
            title: "UI/UX Design",
            title_ar: "تصميم واجهات المستخدم",
            desc: "User research, design systems, prototyping, and accessibility-first interface design.",
            desc_ar: "أبحاث المستخدمين وبناء أنظمة التصميم الشاملة والنماذج التفاعلية الموجهة لسهولة الوصول.",
        },
        ServiceItem {
            icon: "trending",
            title: "Digital Marketing & SEO",
            title_ar: "التسويق الرقمي و SEO",
            desc: "Technical SEO, PPC, content strategy, and analytics to maximize your digital presence.",
            desc_ar: "تحسين محركات البحث SEO التقني والحملات الممولة PPC واستراتيجيات المحتوى الموجه للنمو.",
        },
        ServiceItem {
            icon: "activity",
            title: "QA & Testing",
            title_ar: "ضبط واختبار الجودة",
            desc: "Automated test suites, manual testing, performance audits, and quality engineering.",
            desc_ar: "حزم الاختبار الآلي والاختبار اليدوي الشامل وتدقيق الأداء وضمان الجودة الهندسية.",
        },
        ServiceItem {
            icon: "globe",
            title: "BPO Services",
            title_ar: "خدمات التعهيد والإسناد",
            desc: "Business process outsourcing for data entry, customer support, and back-office operations.",
            desc_ar: "تعهيد العمليات التجارية وإدخال البيانات ودعم العملاء وإدارة العمليات الخلفية بكفاءة.",
        },
        ServiceItem {
            icon: "star",
            title: "Emerging Technology",
            title_ar: "التقنيات الناشئة",
            desc: "Blockchain, IoT, AR/VR, and Web3 solutions for forward-thinking organizations.",
            desc_ar: "حلول البلوكشين وإنترنت الأشياء والواقع المعزز/الافتراضي للمؤسسات الرائدة.",
        },
    ]
}
