//! # Footer Component
//!
//! Matches UI reference PublicLayout.tsx Footer with full Arabic/English bilingual support:
//! bg-navy-900, brand column with logo + bio + 3 social icons,
//! 4 link columns (Services, Industries, Company, Legal),
//! bottom bar with copyright and green operational dot,
//! and interactive bilingual legal modal.

use crate::i18n::use_locale;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let locale_ctx = use_locale();
    let legal_modal_tab = RwSignal::new(Option::<&'static str>::None);

    let link_columns = vec![
        (
            "Services",
            "الخدمات",
            vec![
                ("Software Dev", "تطوير البرمجيات", "/services#software", false, None),
                ("Cloud & DevOps", "السحابة وهندسة العمليات", "/services#cloud", false, None),
                ("Cybersecurity", "الأمن السيبراني", "/services#security", false, None),
                ("AI & Data", "الذكاء الاصطناعي والبيانات", "/services#ai", false, None),
                ("IT Consulting", "الاستشارات التقنية", "/services#consulting", false, None),
                ("Managed IT", "الخدمات المدارة", "/services#managed", false, None),
            ],
        ),
        (
            "Industries",
            "القطاعات",
            vec![
                ("Finance & Banking", "المالية والمصارف", "/case-studies", false, None),
                ("Healthcare", "الرعاية الصحية", "/case-studies", false, None),
                ("Retail & E-commerce", "التجزئة والتجارة الإلكترونية", "/case-studies", false, None),
                ("Manufacturing", "الصناعة والإنتاج", "/case-studies", false, None),
                ("Government", "القطاع الحكومي", "/case-studies", false, None),
                ("Education", "التعليم والتدريب", "/case-studies", false, None),
            ],
        ),
        (
            "Company",
            "الشركة",
            vec![
                ("About Us", "من نحن", "/about", false, None),
                ("Careers", "الوظائف الشاغرة", "/careers", false, None),
                ("Case Studies", "دراسات الحالة", "/case-studies", false, None),
                ("Contact Us", "تواصل معنا", "/contact", false, None),
                ("Request Quote", "طلب عرض أسعار", "/quote", false, None),
                ("Client Portal", "بوابة العملاء", "/login", false, None),
            ],
        ),
        (
            "Legal & Compliance",
            "الشؤون القانونية والامتثال",
            vec![
                ("Privacy Policy", "سياسة الخصوصية", "#privacy", true, Some("privacy")),
                ("Terms of Service", "شروط الخدمة", "#terms", true, Some("terms")),
                ("Cookie Policy", "سياسة ملفات تعريف الارتباط", "#cookies", true, Some("cookies")),
                ("SLA Commitments", "اتفاقية مستوى الخدمة SLA", "#sla", true, Some("sla")),
                ("GDPR Compliance", "الامتثال للائحة GDPR", "#gdpr", true, Some("gdpr")),
                ("Security & ISO 27001", "الأمن ومعيار ISO 27001", "#security", true, Some("security")),
            ],
        ),
    ];

    view! {
        <footer class="bg-navy-900 text-white">
            <div class="max-w-7xl mx-auto px-4 lg:px-8 py-14">
                <div class="grid grid-cols-2 lg:grid-cols-6 gap-8">
                    // Brand column
                    <div class="col-span-2">
                        <div class="flex items-center gap-2.5 mb-4">
                            <img
                                src="/images/logo.png"
                                alt="TeebTech Logo"
                                width="32"
                                height="32"
                                class="w-8 h-8 object-contain flex-shrink-0 drop-shadow-sm"
                            />
                            <span class="font-bold font-sans text-xl tracking-tight leading-none">
                                <span class="text-white">"Teeb"</span>
                                <span class="bg-gradient-to-r from-sky-400 via-cyan-400 to-blue-500 bg-clip-text text-transparent ms-0.5">"Tech"</span>
                            </span>
                        </div>
                        <p class="text-sm text-slate-400 leading-relaxed max-w-xs">
                            {move || if locale_ctx.is_arabic() {
                                "هندسة حلول تقنية وبرمجيات مؤسسية تتوسع مع طموحك. موثوقون من قِبل أكثر من 200 مؤسسة وشركة عالمياً."
                            } else {
                                "Engineering enterprise-grade technology solutions that scale with your ambition. Trusted by 200+ organizations worldwide."
                            }}
                        </p>
                        <div class="flex items-center gap-3 mt-5">
                            // Globe icon
                            <a href="/" class="w-8 h-8 rounded-md bg-navy-800 flex items-center justify-center text-slate-400 hover:text-white hover:bg-navy-700 transition-colors" aria-label="TeebTech Home">
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <line x1="2" y1="12" x2="22" y2="12"></line>
                                    <path d="M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10 15.3 15.3 0 01-4 10 15.3 15.3 0 014-10z"></path>
                                </svg>
                            </a>
                            // Mail icon
                            <a href="mailto:hello@teebtech.com" class="w-8 h-8 rounded-md bg-navy-800 flex items-center justify-center text-slate-400 hover:text-white hover:bg-navy-700 transition-colors" aria-label="Email Us">
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                                </svg>
                            </a>
                            // Chat icon
                            <a href="/contact" class="w-8 h-8 rounded-md bg-navy-800 flex items-center justify-center text-slate-400 hover:text-white hover:bg-navy-700 transition-colors" aria-label="Contact">
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                                </svg>
                            </a>
                        </div>
                    </div>

                    // Link columns
                    {link_columns.into_iter().map(|(heading_en, heading_ar, links)| {
                        view! {
                            <div>
                                <p class="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">
                                    {move || if locale_ctx.is_arabic() { heading_ar } else { heading_en }}
                                </p>
                                <ul class="space-y-2">
                                    {links.into_iter().map(|(label_en, label_ar, href, is_modal, modal_key)| {
                                        view! {
                                            <li>
                                                {if is_modal {
                                                    view! {
                                                        <button
                                                            type="button"
                                                            on:click=move |_| legal_modal_tab.set(modal_key)
                                                            class="text-sm text-slate-400 hover:text-white transition-colors text-start cursor-pointer"
                                                        >
                                                            {move || if locale_ctx.is_arabic() { label_ar } else { label_en }}
                                                        </button>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <a href=href class="text-sm text-slate-400 hover:text-white transition-colors">
                                                            {move || if locale_ctx.is_arabic() { label_ar } else { label_en }}
                                                        </a>
                                                    }.into_any()
                                                }}
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }
                    }).collect_view()}
                </div>

                // Bottom bar
                <div class="mt-10 pt-6 border-t border-navy-800 flex flex-col sm:flex-row items-center justify-between gap-3">
                    <p class="text-xs text-slate-400">
                        {move || if locale_ctx.is_arabic() { "© 2026 شركة تيب تك المحدودة. جميع الحقوق محفوظة." } else { "© 2026 TeebTech Ltd. All rights reserved." }}
                    </p>
                    <div class="flex items-center gap-2">
                        <div class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></div>
                        <span class="text-xs text-slate-400">
                            {move || if locale_ctx.is_arabic() { "جميع الأنظمة تعمل بكفاءة • جاهزية 99.99%" } else { "All systems operational • 99.99% uptime" }}
                        </span>
                    </div>
                </div>
            </div>

            // Interactive Legal Policy Modal
            {move || if let Some(tab) = legal_modal_tab.get() {
                let is_ar = locale_ctx.is_arabic();
                view! {
                    <div
                        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-navy-950/70 backdrop-blur-sm cursor-pointer"
                        role="dialog"
                        aria-modal="true"
                        aria-labelledby="footer-legal-modal-title"
                        on:click=move |_| legal_modal_tab.set(None)
                    >
                        <div
                            class="bg-white rounded-xl shadow-2xl max-w-2xl w-full max-h-[85vh] flex flex-col overflow-hidden border border-slate-200 cursor-default text-start"
                            on:click=move |e| e.stop_propagation()
                        >
                            <div class="flex items-center justify-between px-6 py-4 border-b border-slate-200 bg-slate-50 text-slate-900">
                                <h3 id="footer-legal-modal-title" class="font-display font-semibold text-lg text-slate-900">
                                    {match (tab, is_ar) {
                                        ("privacy", true) => "سياسة الخصوصية العالمية لشركة تيب تك",
                                        ("privacy", false) => "TeebTech Global Privacy Policy",
                                        ("terms", true) => "شروط تقديم الخدمات والتعاقد",
                                        ("terms", false) => "Terms of Service & Engagement",
                                        ("cookies", true) => "سياسة ملفات تعريف الارتباط والجلسات",
                                        ("cookies", false) => "Cookie & Tracking Policy",
                                        ("sla", true) => "اتفاقية مستوى الخدمة والجاهزية (SLA)",
                                        ("sla", false) => "Service Level Agreement (SLA)",
                                        ("gdpr", true) => "الامتثال للائحة حماية البيانات العامة (GDPR)",
                                        ("gdpr", false) => "GDPR & International Data Protection",
                                        (_, true) => "معايير الأمن المؤسسي وشهادة ISO 27001",
                                        (_, false) => "Enterprise Security & ISO 27001 Posture",
                                    }}
                                </h3>
                                <button
                                    on:click=move |_| legal_modal_tab.set(None)
                                    class="p-1 rounded-md text-slate-400 hover:text-slate-600 hover:bg-slate-200 transition-colors"
                                    aria-label="Close modal"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </div>
                            <div class="p-6 overflow-y-auto space-y-4 text-sm text-slate-600 leading-relaxed">
                                {match (tab, is_ar) {
                                    ("privacy", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"تاريخ السريان: 1 يناير 2026"</p>
                                            <p>"تلتزم شركة تيب تك المحدودة بحماية بيانات مؤسستك وبياناتك الشخصية بأعلى المعايير الأمنية. تسري هذه السياسة عبر كافة منصاتنا وتطبيقاتنا وخدماتنا الاستشارية والسحابية."</p>
                                            <p>"نحن لا نقوم مطلقاً ببيع أو تداول بيانات العملاء مع أي أطراف تجارية. وتخضع كافة قواعد البيانات والاتصالات لتشفير صارم بمعايير TLS 1.3 و AES-256."</p>
                                        </div>
                                    }.into_any(),
                                    ("privacy", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Effective Date: January 1, 2026"</p>
                                            <p>"TeebTech Ltd. ('TeebTech', 'we', 'our') is committed to protecting your organizational and personal data. This privacy policy applies across all web platforms, client portals, APIs, and cloud services."</p>
                                            <p>"We never broker, monetize, or sell client information. All databases and network transmissions adhere to TLS 1.3 and AES-256 encryption standards."</p>
                                        </div>
                                    }.into_any(),
                                    ("terms", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"اتفاقية الخدمات الهندسية والتقنية الرئيسية"</p>
                                            <p>"تعتبر كافة البرمجيات والأنظمة والمعماريات المصممة خصيصاً للعميل ملكية فكرية حصرية له فور سداد الالتزامات المالية المتفق عليها."</p>
                                            <p>"يلتزم الطرفان باتفاقيات سرية متبادلة لحماية الشيفرة المصدرية والبيانات الحساسة."</p>
                                        </div>
                                    }.into_any(),
                                    ("terms", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Master Technology Services Agreement"</p>
                                            <p>"All bespoke software engineering, system architecture, and cloud infrastructure delivered by TeebTech become the exclusive intellectual property of the client upon invoice settlement."</p>
                                        </div>
                                    }.into_any(),
                                    ("cookies", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"معايير ملفات تعريف الارتباط الضرورية"</p>
                                            <p>"نستخدم فقط ملفات تعريف الارتباط الآمنة والمشفرة والضرورية لإدارة الجلسات وحماية المستخدمين من هجمات CSRF. لا نستخدم أدوات تعقب إعلانية خارجية."</p>
                                        </div>
                                    }.into_any(),
                                    ("cookies", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Strictly Necessary Session Storage"</p>
                                            <p>"We only utilize secure, HttpOnly authentication cookies necessary to protect tenant sessions against CSRF and session hijacking. No third-party ad trackers are loaded."</p>
                                        </div>
                                    }.into_any(),
                                    ("sla", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"ضمانات اتفاقية مستوى الخدمة (SLA)"</p>
                                            <p>"نضمن جاهزية بنسبة 99.99% عبر مراكز بيانات متعددة المناطق، مع استجابة فورية للحوادث الحرجة خلال أقل من 15 دقيقة على مدار الساعة."</p>
                                        </div>
                                    }.into_any(),
                                    ("sla", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Service Level Agreement (SLA)"</p>
                                            <p>"Our SLA guarantees 99.99% multi-region uptime, sub-15-minute response times for critical incidents, and automated disaster recovery failover."</p>
                                        </div>
                                    }.into_any(),
                                    ("gdpr", true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"الامتثال للأنظمة والمعايير الدولية لحماية البيانات"</p>
                                            <p>"نلتزم ببنود معالجة البيانات القياسية لحماية خصوصية المستخدمين وحق الوصول والحذف والتعديل عبر التواصل مع: data-privacy@teebtech.com."</p>
                                        </div>
                                    }.into_any(),
                                    ("gdpr", false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"GDPR & Cross-Border Compliance"</p>
                                            <p>"We provide standard contractual clauses (SCCs) and robust Data Processing Addendums. Data subject requests can be submitted to data-privacy@teebtech.com."</p>
                                        </div>
                                    }.into_any(),
                                    (_, true) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"ضوابط الأمن السيبراني ومعايير ISO 27001 / SOC 2"</p>
                                            <p>"عمليات معتمدة بشهادات ISO/IEC 27001:2022 و SOC 2 Type II، وتدقيق دوري مستمر للثغرات الأمنية مع تطبيق صارم لمبدأ الثقة الصفرية Zero-Trust."</p>
                                        </div>
                                    }.into_any(),
                                    (_, false) => view! {
                                        <div class="space-y-3">
                                            <p class="font-semibold text-slate-800">"Enterprise Security & ISO 27001 Controls"</p>
                                            <p>"Certified ISO/IEC 27001:2022 and SOC 2 Type II operations, continuous SAST/DAST vulnerability scanning, and strict Zero-Trust role-based access control."</p>
                                        </div>
                                    }.into_any(),
                                }}
                            </div>
                            <div class="px-6 py-3 border-t border-slate-200 bg-slate-50 flex justify-end">
                                <button
                                    on:click=move |_| legal_modal_tab.set(None)
                                    class="px-4 py-2 text-xs font-semibold rounded-md bg-brand-600 text-white hover:bg-brand-700 transition-colors"
                                >
                                    {move || if locale_ctx.is_arabic() { "فهمت وموافق" } else { "I Understand" }}
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div class="hidden"></div> }.into_any()
            }}
        </footer>
    }
}
