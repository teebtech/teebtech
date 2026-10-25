use crate::pages::{
    AboutPage, CareersPage, CaseStudiesPage, ContactPage, HomePage, LoginPage, QuotePage,
    ServicesPage,
};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};
use ui::components::Icon;
use ui::layout::PublicLayout;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    let locale_ctx = crate::i18n::use_locale();

    view! {
        <Title text=move || if locale_ctx.is_arabic() { "الصفحة غير موجودة (404) — تيب تك" } else { "Page Not Found (404) — TeebTech" } />
        <Meta name="description" content=move || if locale_ctx.is_arabic() {
            "الصفحة المطلوبة غير موجودة على منصة تيب تك المؤسسية. يرجى العودة للرئيسية أو استعراض خدماتنا."
        } else {
            "The requested page could not be found on the TeebTech corporate portal. Return home or browse our enterprise engineering services."
        } />
        <PublicLayout current_page="404".to_string()>
            <div class="min-h-[60vh] py-16 lg:py-24 px-4 flex flex-col items-center justify-center text-center max-w-2xl mx-auto">
                <div class="w-16 h-16 rounded-2xl bg-brand-50 text-brand-600 flex items-center justify-center mb-6 shadow-xs">
                    <Icon name="alertCircle".to_string() size=32 />
                </div>
                <span class="text-xs font-bold text-brand-600 uppercase tracking-widest mb-2">
                    {move || if locale_ctx.is_arabic() { "خطأ 404" } else { "Error 404" }}
                </span>
                <h1 class="text-4xl sm:text-5xl font-bold font-display text-slate-900 mb-4">
                    {move || if locale_ctx.is_arabic() { "الصفحة غير موجودة" } else { "Page not found" }}
                </h1>
                <p class="text-slate-600 text-base leading-relaxed mb-8 max-w-md">
                    {move || if locale_ctx.is_arabic() {
                        "الصفحة التي تبحث عنها ربما تم نقلها أو تغيير اسمها أو أنها غير متاحة حالياً. يسعدنا مساعدتك للعودة إلى المسار الصحيح."
                    } else {
                        "The page you are looking for might have been moved, renamed, or is temporarily unavailable. Let us help you get back on track."
                    }}
                </p>
                <div class="flex items-center justify-center gap-3 flex-wrap mb-10">
                    <a
                        href="/"
                        class="px-5 py-2.5 bg-brand-600 text-white rounded-lg text-sm font-semibold hover:bg-brand-700 transition-colors shadow-sm inline-flex items-center gap-2"
                    >
                        <Icon name="arrowLeft".to_string() size=14 class="rtl:rotate-180".to_string() />
                        <span>{move || if locale_ctx.is_arabic() { "العودة للرئيسية" } else { "Return to Home" }}</span>
                    </a>
                    <a
                        href="/services"
                        class="px-5 py-2.5 border border-slate-300 text-slate-700 bg-white rounded-lg text-sm font-medium hover:bg-slate-50 transition-colors"
                    >
                        {move || if locale_ctx.is_arabic() { "استعراض الخدمات" } else { "Browse Services" }}
                    </a>
                    <a
                        href="/contact"
                        class="px-5 py-2.5 border border-slate-300 text-slate-700 bg-white rounded-lg text-sm font-medium hover:bg-slate-50 transition-colors"
                    >
                        {move || if locale_ctx.is_arabic() { "تواصل مع الدعم" } else { "Contact Support" }}
                    </a>
                </div>
            </div>
        </PublicLayout>
    }
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFoundPage>
                // Root public routes
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("services") view=ServicesPage/>
                <Route path=StaticSegment("about") view=AboutPage/>
                <Route path=StaticSegment("case-studies") view=CaseStudiesPage/>
                <Route path=StaticSegment("work") view=CaseStudiesPage/>
                <Route path=StaticSegment("contact") view=ContactPage/>
                <Route path=StaticSegment("quote") view=QuotePage/>
                <Route path=StaticSegment("login") view=LoginPage/>
                <Route path=StaticSegment("careers") view=CareersPage/>

                // Arabic-prefixed routes (/ar/...)
                <ParentRoute path=StaticSegment("ar") view=|| view! { <leptos_router::components::Outlet/> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("services") view=ServicesPage/>
                    <Route path=StaticSegment("about") view=AboutPage/>
                    <Route path=StaticSegment("case-studies") view=CaseStudiesPage/>
                    <Route path=StaticSegment("work") view=CaseStudiesPage/>
                    <Route path=StaticSegment("contact") view=ContactPage/>
                    <Route path=StaticSegment("quote") view=QuotePage/>
                    <Route path=StaticSegment("login") view=LoginPage/>
                    <Route path=StaticSegment("careers") view=CareersPage/>
                </ParentRoute>

                // English-prefixed routes (/en/...)
                <ParentRoute path=StaticSegment("en") view=|| view! { <leptos_router::components::Outlet/> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("services") view=ServicesPage/>
                    <Route path=StaticSegment("about") view=AboutPage/>
                    <Route path=StaticSegment("case-studies") view=CaseStudiesPage/>
                    <Route path=StaticSegment("work") view=CaseStudiesPage/>
                    <Route path=StaticSegment("contact") view=ContactPage/>
                    <Route path=StaticSegment("quote") view=QuotePage/>
                    <Route path=StaticSegment("login") view=LoginPage/>
                    <Route path=StaticSegment("careers") view=CareersPage/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
