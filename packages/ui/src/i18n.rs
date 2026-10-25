//! # UI i18n Foundation & Language Switcher
//!
//! Provides the reactive `LocaleContext`, `use_locale` hook, and accessible `LanguageSwitcher` component.

use design_system::{Direction, Locale};
use leptos::prelude::*;

/// Reactive Locale Context provided at the root of the application or layout.
#[derive(Debug, Clone, Copy)]
pub struct LocaleContext {
    pub locale: RwSignal<Locale>,
}

impl LocaleContext {
    pub fn new(initial: Locale) -> Self {
        Self {
            locale: RwSignal::new(initial),
        }
    }

    pub fn get(&self) -> Locale {
        self.locale.get()
    }

    pub fn is_arabic(&self) -> bool {
        self.locale.get().is_arabic()
    }

    pub fn is_english(&self) -> bool {
        self.locale.get().is_english()
    }

    pub fn direction(&self) -> Direction {
        self.locale.get().direction()
    }

    pub fn dir_str(&self) -> &'static str {
        self.locale.get().dir_str()
    }

    pub fn lang_str(&self) -> &'static str {
        self.locale.get().as_str()
    }

    /// Toggle language between English and Arabic.
    pub fn toggle(&self) {
        let next = self.locale.get().toggle();
        self.set(next);
    }

    /// Set an explicit locale, persisting it to cookie and updating document attributes.
    pub fn set(&self, new_locale: Locale) {
        self.locale.set(new_locale);

        #[cfg(feature = "hydrate")]
        {
            use web_sys::wasm_bindgen::JsCast;

            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(root) = doc.document_element() {
                        let _ = root.set_attribute("lang", new_locale.as_str());
                        let _ = root.set_attribute("dir", new_locale.dir_str());
                    }
                    // Persist to cookie (1 year expiry, SameSite=Lax) via HtmlDocument
                    if let Ok(html_doc) = doc.dyn_into::<web_sys::HtmlDocument>() {
                        let cookie_str = format!(
                            "locale={}; path=/; max-age=31536000; SameSite=Lax",
                            new_locale.as_str()
                        );
                        let _ = html_doc.set_cookie(&cookie_str);
                    }
                }
            }
        }
    }
}

/// Retrieve the current `LocaleContext` from Leptos context, or fallback to client-detected default.
pub fn use_locale() -> LocaleContext {
    use_context::<LocaleContext>().unwrap_or_else(|| {
        let initial = detect_initial_client_locale();
        let ctx = LocaleContext::new(initial);
        provide_context(ctx);
        ctx
    })
}

/// Provide the `LocaleContext` at root level.
pub fn provide_locale(initial: Locale) -> LocaleContext {
    let ctx = LocaleContext::new(initial);
    provide_context(ctx);
    ctx
}

/// Detect initial locale on client from documentElement lang attribute or cookies.
pub fn detect_initial_client_locale() -> Locale {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            // 1. Check documentElement attribute rendered by SSR
            if let Some(doc) = window.document() {
                if let Some(root) = doc.document_element() {
                    if let Some(lang) = root.get_attribute("lang") {
                        if lang.starts_with("ar") {
                            return Locale::Ar;
                        } else if lang.starts_with("en") {
                            return Locale::En;
                        }
                    }
                }

                // 2. Check document cookies
                if let Ok(html_doc) = doc.dyn_into::<web_sys::HtmlDocument>() {
                    if let Ok(cookie_str) = html_doc.cookie() {
                        for cookie in cookie_str.split(';') {
                            let pair: Vec<&str> = cookie.trim().split('=').collect();
                            if pair.len() == 2 && pair[0] == "locale" {
                                return Locale::from_str_loose(pair[1]);
                            }
                        }
                    }
                }
            }

            // 3. Check window.location.pathname
            if let Ok(path) = window.location().pathname() {
                if path.starts_with("/ar") {
                    return Locale::Ar;
                } else if path.starts_with("/en") {
                    return Locale::En;
                }
            }
        }
    }
    Locale::En
}

/// Header & navigation language toggle button.
///
/// Clean, accessible, and responsive toggle supporting both desktop and mobile bars.
#[component]
pub fn LanguageSwitcher(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let locale_ctx = use_locale();
    let extra_class = class.unwrap_or_default();

    let on_click = move |_| {
        locale_ctx.toggle();
    };

    view! {
        <button
            type="button"
            on:click=on_click
            class=format!(
                "inline-flex items-center justify-center gap-1.5 px-2.5 py-1.5 text-xs font-semibold rounded-lg border border-slate-200/90 bg-white/95 text-slate-700 hover:text-brand-600 hover:bg-slate-50 hover:border-brand-200 transition-all duration-150 shadow-2xs focus:outline-none focus:ring-2 focus:ring-brand-500 focus:ring-offset-1 cursor-pointer select-none group {}",
                extra_class
            )
            aria-label=move || if locale_ctx.is_arabic() {
                "التبديل إلى اللغة الإنجليزية (Switch to English)"
            } else {
                "Switch to Arabic (التبديل إلى العربية)"
            }
            title=move || if locale_ctx.is_arabic() {
                "Switch to English"
            } else {
                "التبديل إلى العربية"
            }
        >
            // Globe icon
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.8"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="text-slate-400 group-hover:text-brand-600 transition-colors flex-shrink-0"
                aria-hidden="true"
            >
                <circle cx="12" cy="12" r="10" />
                <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
                <path d="M2 12h20" />
            </svg>

            // Label shows the target language to switch to
            <span class="font-medium tracking-tight">
                {move || locale_ctx.get().switcher_target_label()}
            </span>
        </button>
    }
}
