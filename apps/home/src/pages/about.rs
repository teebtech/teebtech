//! # About Page
//!
//! Matches UI reference About.tsx with full Arabic and English support:
//! Hero, company story with photo, team & values cards, leadership section, and CTA.

use crate::i18n::use_locale;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use ui::components::Icon;
use ui::layout::PublicLayout;

struct TeamMember {
    name: &'static str,
    role_en: &'static str,
    role_ar: &'static str,
    bio_en: &'static str,
    bio_ar: &'static str,
    image: &'static str,
}

const TEAM: &[TeamMember] = &[
    TeamMember {
        name: "Emeka Obi",
        role_en: "CEO & Co-founder",
        role_ar: "الرئيس التنفيذي والمؤسس المشارك",
        bio_en: "Former Principal Engineer at Microsoft Azure. 18 years building enterprise software.",
        bio_ar: "كبير مهندسين سابق في Microsoft Azure. خبرة تمتد لـ 18 عاماً في هندسة البرمجيات المؤسسية.",
        image: "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=200&h=200&fit=crop&auto=format",
    },
    TeamMember {
        name: "Dr. Amara Diallo",
        role_en: "CTO",
        role_ar: "كبير مسؤولي التقنية",
        bio_en: "PhD Computer Science, Stanford. Led cloud architecture at Stripe and Cloudflare.",
        bio_ar: "دكتوراه في علوم الحاسوب من ستانفورد. قادت المعمارية السحابية في Stripe و Cloudflare.",
        image: "https://images.unsplash.com/photo-1531123897727-8f129e1688ce?w=200&h=200&fit=crop&auto=format",
    },
    TeamMember {
        name: "Lars Eriksson",
        role_en: "VP Engineering",
        role_ar: "نائب رئيس الهندسة",
        bio_en: "Previously Head of Platform at Klarna. Expert in distributed systems and reliability.",
        bio_ar: "رئيس منصات سابق في Klarna. خبير متخصص في الأنظمة الموزعة والموثوقية العالية.",
        image: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=200&h=200&fit=crop&auto=format",
    },
    TeamMember {
        name: "Priya Venkataraman",
        role_en: "Head of Cybersecurity",
        role_ar: "رئيسة قسم الأمن السيبراني",
        bio_en: "Former CISO at Standard Chartered Bank. CISSP, CISM certified.",
        bio_ar: "مديرة أمن معلومات سابقة في Standard Chartered Bank. حاصلة على شهادات CISSP و CISM.",
        image: "https://images.unsplash.com/photo-1573497019940-1c28c88b4f3e?w=200&h=200&fit=crop&auto=format",
    },
    TeamMember {
        name: "Marcus O'Brien",
        role_en: "Head of AI & Data",
        role_ar: "رئيس قسم الذكاء الاصطناعي والبيانات",
        bio_en: "ML research background from DeepMind. Specializes in production ML systems.",
        bio_ar: "خلفية بحثية في تعلم الآلة من DeepMind. متخصص في نشر أنظمة الذكاء الاصطناعي الإنتاجية.",
        image: "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=200&h=200&fit=crop&auto=format",
    },
    TeamMember {
        name: "Sophia Mensah",
        role_en: "Head of Design",
        role_ar: "رئيسة قسم التصميم",
        bio_en: "Previously Design Lead at Intercom. Pioneer in design systems for enterprise SaaS.",
        bio_ar: "قائدة تصميم سابقة في Intercom. رائدة في بناء أنظمة التصميم لبرمجيات SaaS المؤسسية.",
        image: "https://images.unsplash.com/photo-1580489944761-15a19d654956?w=200&h=200&fit=crop&auto=format",
    },
];

struct ValueItem {
    icon: &'static str,
    title_en: &'static str,
    title_ar: &'static str,
    desc_en: &'static str,
    desc_ar: &'static str,
}

const VALUES: &[ValueItem] = &[
    ValueItem {
        icon: "shield",
        title_en: "Engineering Integrity",
        title_ar: "النزاهة الهندسية",
        desc_en: "We say what we build and build what we say. No hidden technical debt, no inflated estimates.",
        desc_ar: "نلتزم بما نبنيه ونبني ما نعد به بدقة. لا ديون تقنية خفية ولا تقديرات مبالغ فيها.",
    },
    ValueItem {
        icon: "users",
        title_en: "Client Partnership",
        title_ar: "الشراكة الحقيقية",
        desc_en: "Our success is measured by your outcomes, not by hours billed. We're invested in your results.",
        desc_ar: "نجاحنا يقاس بمخرجاتك وأثرها في السوق، وليس بمجرد ساعات العمل المسجلة.",
    },
    ValueItem {
        icon: "trending",
        title_en: "Relentless Improvement",
        title_ar: "التطور المستمر",
        desc_en: "Every project generates lessons that make the next one better. We operate a continuous improvement culture.",
        desc_ar: "كل مشروع يولد خبرات نطور بها مشاريعنا القادمة وفق ثقافة التحسين المستمر.",
    },
    ValueItem {
        icon: "globe",
        title_en: "Diverse Perspectives",
        title_ar: "تنوع الرؤى والخبرات",
        desc_en: "Our team spans 22 nationalities. This diversity is not a policy — it is our competitive advantage.",
        desc_ar: "يضم فريقنا خبرات من 22 جنسية مختلفة، وهذا التنوع هو ميزتنا التنافسية الكبرى.",
    },
];

#[component]
pub fn AboutPage() -> impl IntoView {
    let locale_ctx = use_locale();

    view! {
        <Title text=move || if locale_ctx.is_arabic() {
            "عن تيب تك والقيادة الهندسية — تيب تك"
        } else {
            "About Us & Engineering Leadership — TeebTech"
        } />
        <Meta name="description" content=move || if locale_ctx.is_arabic() {
            "تعرف على تيب تك: تأسست عام 2011، شركة استشارات وتطوير تقنية تضم أكثر من 180 مهندساً ومعمارياً وخبيراً أمنياً عبر أربع قارات."
        } else {
            "Meet TeebTech: Founded in 2011, we are an engineering-first technology consulting firm of 180+ architects, developers, and security specialists across four continents."
        } />

        <PublicLayout current_page="about".to_string()>
            <div>
                // Hero
                <section class="hero-grid py-16 lg:py-24 text-white">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <div class="max-w-2xl text-start">
                            <p class="text-xs font-semibold tracking-widest text-brand-400 uppercase mb-3">
                                {move || if locale_ctx.is_arabic() { "عن تيب تك" } else { "About TeebTech" }}
                            </p>
                            <h1 class="font-display text-4xl lg:text-5xl font-semibold text-white leading-tight mb-5">
                                {move || if locale_ctx.is_arabic() {
                                    view! {
                                        <span>"نحن مهندسون شغوفون "</span><br />
                                        <span class="italic text-brand-400">"نبني أعمالاً تصنع الفارق."</span>
                                    }.into_any()
                                } else {
                                    view! {
                                        <span>"We are engineers who"</span><br />
                                        <span class="italic text-brand-400">"happen to do business."</span>
                                    }.into_any()
                                }}
                            </h1>
                            <p class="text-slate-400 leading-relaxed text-base lg:text-lg">
                                {move || if locale_ctx.is_arabic() {
                                    "تأسست تيب تك في عام 2011 كشركة متخصصة في الاستشارات البرمجية، ونمت لتصبح شريكاً تقنياً عالمياً بمكاتب في أربع قارات تخدم المؤسسات والشركات الطموحة حول العالم."
                                } else {
                                    "Founded in 2011, TeebTech started as a boutique software consultancy in Lagos and has grown into a global technology services firm with offices on four continents, serving clients from Fortune 500s to ambitious scale-ups."
                                }}
                            </p>
                        </div>
                    </div>
                </section>

                // Story
                <section class="bg-white py-16 lg:py-20">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center">
                            <div class="text-start">
                                <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                                    {move || if locale_ctx.is_arabic() { "قصتنا" } else { "Our Story" }}
                                </p>
                                <h2 class="font-display text-3xl lg:text-4xl font-semibold text-slate-900 leading-snug mb-5">
                                    {move || if locale_ctx.is_arabic() {
                                        "بنيت بأيدي مهندسين، من أجل المهندسين والشركات التي تعتمد على التقنية."
                                    } else {
                                        "Built by engineers, for engineers — and the businesses who depend on them."
                                    }}
                                </h2>
                                <div class="space-y-4 text-slate-600 leading-relaxed text-sm sm:text-base">
                                    <p>
                                        {move || if locale_ctx.is_arabic() {
                                            "عندما تأسست تيب تك، كانت الفجوة بين استشارات التكنولوجيا النظرية والتنفيذ الهندسي الفعلي هائلة. تأسست تيب تك خصيصاً لسد تلك الفجوة وتوفير هندسة حقيقية يمكن الاعتماد عليها."
                                        } else {
                                            "When TeebTech was founded, the gap between technology consulting and technology execution was enormous. Traditional consultants produced slide decks that engineering teams could not realistically build. TeebTech was founded specifically to close that gap."
                                        }}
                                    </p>
                                    <p>
                                        {move || if locale_ctx.is_arabic() {
                                            "مبدأنا الأساسي بسيط: كل مشروع يقوده ممارسون ومهندسون متمرسون ذوو خبرة إنتاجية عميقة. استشاراتنا تنبع دائماً من واقع ما يعمل بنجاح وأمان على نطاق واسع."
                                        } else {
                                            "Our founding principle is simple: every engagement is spearheaded by senior practitioners with deep production experience. Our advisory work is grounded in hands-on knowledge of what actually works reliably at scale."
                                        }}
                                    </p>
                                    <p>
                                        {move || if locale_ctx.is_arabic() {
                                            "على مدار 14 عاماً، أثمر هذا النهج عن إنجاز أكثر من 500 مشروع بنجاح، ومعدل استبقاء عملاء استثنائي بلغ 98%، ومكانة دولية راسخة كشريك تقني أول للمؤسسات الرائدة."
                                        } else {
                                            "Over 14 years, that disciplined approach has produced 500+ delivered projects, an industry-leading 98% client retention rate, and an international reputation as the technology partner of choice for organizations that take engineering seriously."
                                        }}
                                    </p>
                                </div>
                                <div class="mt-8 flex items-center gap-4 flex-wrap">
                                    <a
                                        href="/case-studies"
                                        class="inline-flex items-center justify-center font-medium rounded-lg text-sm px-5 py-2.5 bg-brand-600 text-white hover:bg-brand-700 transition-colors shadow-sm"
                                    >
                                        {move || if locale_ctx.is_arabic() { "استعرض أعمالنا" } else { "View Our Work" }}
                                    </a>
                                    <a
                                        href="/careers"
                                        class="inline-flex items-center justify-center font-medium rounded-lg text-sm px-5 py-2.5 border border-slate-300 text-slate-700 bg-white hover:bg-slate-50 transition-colors"
                                    >
                                        {move || if locale_ctx.is_arabic() { "انضم لفريقنا" } else { "Join the Team" }}
                                    </a>
                                </div>
                            </div>
                            <div class="relative">
                                <img
                                    src="https://images.unsplash.com/photo-1522071820081-009f0129c71c?w=720&h=540&fit=crop&auto=format"
                                    alt="TeebTech engineering team collaborating in an open workspace"
                                    loading="lazy"
                                    decoding="async"
                                    class="w-full rounded-2xl shadow-xl object-cover"
                                />
                                <div class="absolute bottom-4 start-4 sm:bottom-6 sm:start-6 bg-white/95 backdrop-blur-md rounded-xl px-5 py-3.5 border border-slate-200 shadow-md">
                                    <div class="flex items-center gap-3">
                                        <div class="flex -space-x-2 rtl:space-x-reverse">
                                            {["AO", "KO", "PE", "TN"].iter().map(|i| {
                                                view! {
                                                    <div class="w-8 h-8 rounded-full bg-navy-900 border-2 border-white flex items-center justify-center text-white text-xs font-semibold shadow-xs">
                                                        {i.chars().next().unwrap_or('U').to_string()}
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <div class="text-start">
                                            <p class="text-xs font-bold text-slate-900">
                                                {move || if locale_ctx.is_arabic() { "180+ كبير مهندسين" } else { "180+ Senior Engineers" }}
                                            </p>
                                            <p class="text-2xs text-slate-500">
                                                {move || if locale_ctx.is_arabic() { "عبر 4 مراكز عالمية" } else { "Across 4 Global Hubs" }}
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Values
                <section class="bg-slate-50 py-16 lg:py-20 border-y border-slate-200">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8 text-start">
                        <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                            {move || if locale_ctx.is_arabic() { "قيمنا" } else { "Our Values" }}
                        </p>
                        <h2 class="font-display text-3xl font-semibold text-slate-900 mb-10">
                            {move || if locale_ctx.is_arabic() { "ما يحرك شغفنا والتزامنا" } else { "What drives us" }}
                        </h2>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                            {VALUES.iter().map(|v| {
                                view! {
                                    <div class="bg-white border border-slate-200 rounded-xl p-6 shadow-xs hover:shadow-md transition-shadow text-start">
                                        <div class="w-10 h-10 bg-brand-50 rounded-lg flex items-center justify-center text-brand-600 mb-4">
                                            <Icon name=v.icon.to_string() size=18 />
                                        </div>
                                        <h3 class="text-base font-semibold text-slate-900 mb-2">
                                            {move || if locale_ctx.is_arabic() { v.title_ar } else { v.title_en }}
                                        </h3>
                                        <p class="text-xs text-slate-500 leading-relaxed">
                                            {move || if locale_ctx.is_arabic() { v.desc_ar } else { v.desc_en }}
                                        </p>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </section>

                // Leadership
                <section class="bg-white py-16 lg:py-20">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8 text-start">
                        <p class="text-xs font-semibold tracking-widest text-brand-600 uppercase mb-2">
                            {move || if locale_ctx.is_arabic() { "القيادة" } else { "Leadership" }}
                        </p>
                        <h2 class="font-display text-3xl font-semibold text-slate-900 mb-10">
                            {move || if locale_ctx.is_arabic() { "المهندسون وراء نجاح تيب تك" } else { "The engineers behind the firm" }}
                        </h2>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                            {TEAM.iter().map(|member| {
                                view! {
                                    <div class="flex items-start gap-4 p-5 border border-slate-200 rounded-xl hover:shadow-md transition-all bg-white hover:border-slate-300 text-start">
                                        <img
                                            src=member.image
                                            alt=format!("Headshot of {}", member.name)
                                            loading="lazy"
                                            decoding="async"
                                            class="w-14 h-14 rounded-full object-cover flex-shrink-0 border border-slate-100 shadow-xs"
                                        />
                                        <div>
                                            <h3 class="text-sm font-semibold text-slate-900">{member.name}</h3>
                                            <p class="text-xs text-brand-600 font-medium mb-1.5">
                                                {move || if locale_ctx.is_arabic() { member.role_ar } else { member.role_en }}
                                            </p>
                                            <p class="text-xs text-slate-500 leading-relaxed">
                                                {move || if locale_ctx.is_arabic() { member.bio_ar } else { member.bio_en }}
                                            </p>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </section>

                // CTA
                <section class="bg-navy-900 py-16 text-white text-center">
                    <div class="max-w-7xl mx-auto px-4 lg:px-8">
                        <h2 class="font-display text-3xl lg:text-4xl font-semibold text-white mb-3">
                            {move || if locale_ctx.is_arabic() { "اعمل معنا اليوم" } else { "Work with us" }}
                        </h2>
                        <p class="text-slate-400 max-w-md mx-auto mb-8 leading-relaxed">
                            {move || if locale_ctx.is_arabic() {
                                "سواء كنت تبحث عن شريك هندسي موثوق لتسريع أعمالك أو مستعداً للانضمام إلى فريقنا العالمي المتميز."
                            } else {
                                "Whether you are looking for a reliable engineering partner or ready to join our world-class distributed team."
                            }}
                        </p>
                        <div class="flex items-center justify-center gap-4 flex-wrap">
                            <a
                                href="/quote"
                                class="inline-flex items-center justify-center font-medium rounded-lg text-sm px-6 py-3 bg-brand-600 text-white hover:bg-brand-700 transition-colors shadow-sm"
                            >
                                {move || if locale_ctx.is_arabic() { "ابدأ مشروعك" } else { "Start a Project" }}
                            </a>
                            <a
                                href="/careers"
                                class="inline-flex items-center justify-center font-medium rounded-lg text-sm px-6 py-3 border border-slate-600 text-slate-300 hover:bg-white/10 hover:text-white transition-colors"
                            >
                                {move || if locale_ctx.is_arabic() { "انضم لفريقنا" } else { "Join Our Team" }}
                            </a>
                        </div>
                    </div>
                </section>
            </div>
        </PublicLayout>
    }
}
