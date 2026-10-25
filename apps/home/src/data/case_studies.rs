use design_system::Locale;

#[derive(Clone, Copy)]
pub struct CaseStudy {
    pub industry: &'static str,
    pub industry_ar: &'static str,
    pub title: &'static str,
    pub title_ar: &'static str,
    pub result: &'static str,
    pub result_ar: &'static str,
    pub image: &'static str,
    pub tags: &'static [&'static str],
    pub tags_ar: &'static [&'static str],
}

impl CaseStudy {
    pub fn localized_industry(&self, locale: Locale) -> &'static str {
        if locale.is_arabic() { self.industry_ar } else { self.industry }
    }

    pub fn localized_title(&self, locale: Locale) -> &'static str {
        if locale.is_arabic() { self.title_ar } else { self.title }
    }

    pub fn localized_result(&self, locale: Locale) -> &'static str {
        if locale.is_arabic() { self.result_ar } else { self.result }
    }

    pub fn localized_tags(&self, locale: Locale) -> &'static [&'static str] {
        if locale.is_arabic() { self.tags_ar } else { self.tags }
    }
}

pub fn get_case_studies() -> Vec<CaseStudy> {
    vec![
        CaseStudy {
            industry: "Finance",
            industry_ar: "المالية والمصارف",
            title: "Core Banking Modernization for FirstNile Bank",
            title_ar: "تحديث المنظومة المصرفية الأساسية لبنك النيل الأول",
            result: "99.97% uptime, 60% faster processing",
            result_ar: "جاهزية 99.97% ومعالجة أسرع للعمليات بنسبة 60%",
            image: "https://images.unsplash.com/photo-1560472355-536de3962603?w=600&h=360&fit=crop&auto=format",
            tags: &["Cloud Migration", "DevOps", "Security"],
            tags_ar: &["الترحيل السحابي", "هندسة العمليات", "الأمن السيبراني"],
        },
        CaseStudy {
            industry: "Healthcare",
            industry_ar: "الرعاية الصحية",
            title: "AI-Powered Diagnostic Platform for MedCore Group",
            title_ar: "منصة تشخيص مدعومة بالذكاء الاصطناعي لمجموعة ميدكور",
            result: "40% reduction in diagnostic turnaround time",
            result_ar: "تقليص زمن التشخيص الطبي بنسبة 40%",
            image: "https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?w=600&h=360&fit=crop&auto=format",
            tags: &["AI/ML", "HIPAA Compliance", "Integration"],
            tags_ar: &["ذكاء اصطناعي", "امتثال HIPAA", "تكامل الأنظمة"],
        },
        CaseStudy {
            industry: "Retail",
            industry_ar: "التجارة الإلكترونية",
            title: "Omnichannel Commerce Suite for PanAfrica Retail",
            title_ar: "منصة التجارة الموحدة متعددة القنوات لشركة بان أفريكا للتجزئة",
            result: "3.2x revenue growth in 18 months",
            result_ar: "نمو الإيرادات بمعدل 3.2 أضعاف خلال 18 شهراً",
            image: "https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=600&h=360&fit=crop&auto=format",
            tags: &["E-commerce", "Mobile", "ERP"],
            tags_ar: &["تجارة إلكترونية", "تطبيق هاتف", "نظام ERP"],
        },
    ]
}
