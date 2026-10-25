use crate::routes::AppRoutes;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};

pub use crate::server::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let initial_locale = crate::i18n::resolve_request_locale();
    #[cfg(not(feature = "ssr"))]
    let initial_locale = design_system::Locale::En;

    let _ = crate::i18n::provide_locale(initial_locale);
    let lang = initial_locale.as_str();
    let dir = initial_locale.dir_str();
    let is_ar = initial_locale.is_arabic();

    let meta_desc = if is_ar {
        "تيب تك — حلول هندسية وبرمجيات مؤسسية متقدمة، بنية تحتية سحابية، أمن سيبراني، وذكاء اصطناعي تواكب تطلعات نمو أعمالكم."
    } else {
        "TeebTech — Enterprise-grade software, cloud infrastructure, cybersecurity, and AI solutions that scale with your ambition."
    };

    let og_locale = if is_ar { "ar_SA" } else { "en_US" };
    let og_locale_alt = if is_ar { "en_US" } else { "ar_SA" };

    view! {
        <!DOCTYPE html>
        <html lang=lang dir=dir>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content=meta_desc/>

                // Multilingual SEO Alternates & Canonical
                <link rel="alternate" hreflang="en" href="/en"/>
                <link rel="alternate" hreflang="ar" href="/ar"/>
                <link rel="alternate" hreflang="x-default" href="/"/>

                // Open Graph & Social Sharing
                <meta name="og:type" content="website"/>
                <meta name="og:site_name" content="TeebTech"/>
                <meta name="og:locale" content=og_locale/>
                <meta name="og:locale:alternate" content=og_locale_alt/>

                <meta name="theme-color" content="#0b1220"/>
                <link rel="apple-touch-icon" href="/images/logo.png"/>

                // Fonts — Bricolage Grotesque & Cairo (Hero), Fraunces (display), Inter (body), JetBrains Mono
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
                <link href="https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:wght@400;500;700;800&family=Cairo:wght@400;600;700;800&family=Fraunces:ital,opsz,wght@0,9..144,300;0,9..144,400;0,9..144,500;0,9..144,600;0,9..144,700;1,9..144,300;1,9..144,400;1,9..144,600&family=Inter:wght@300;400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet"/>
                <link rel="icon" type="image/png" href="/images/logo.png"/>

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="min-h-screen flex flex-col bg-slate-50 text-slate-900 font-sans antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let locale_ctx = crate::i18n::use_locale();

    view! {
        <Stylesheet id="leptos" href="/pkg/home.css"/>
        <Title text=move || {
            if locale_ctx.is_arabic() {
                "تيب تك — حلول تقنية وبرمجيات مؤسسية متقدمة"
            } else {
                "TeebTech — Enterprise Technology Services"
            }
        }/>
        <AppRoutes/>
    }
}
