//! # Locale Context & Server Resolution
//!
//! Provides request-scoped and client-hydrated reactive LocaleContext for Leptos,
//! re-exporting the core `LocaleContext` from `ui::i18n` and providing SSR resolution.

pub use design_system::i18n::{Direction, Locale};
pub use ui::i18n::*;


/// Resolves locale from individual request inputs with strict priority:
/// 1. Path prefix (/ar or /ar/...)
/// 2. Query param (lang=ar or locale=ar)
/// 3. Cookie header (locale=ar or locale=en)
/// 4. Accept-Language header
/// 5. Default English
pub fn resolve_locale_from_inputs(
    path: &str,
    query: Option<&str>,
    cookie: Option<&str>,
    accept_language: Option<&str>,
) -> Locale {
    // 1. Priority 1: Explicit URL path (/ar or /ar/...)
    if path == "/ar" || path.starts_with("/ar/") {
        return Locale::Ar;
    }
    if path == "/en" || path.starts_with("/en/") {
        return Locale::En;
    }

    // Priority 1b: Explicit query param (?lang=ar or ?lang=en)
    if let Some(q) = query {
        for param in q.split('&') {
            let mut kv = param.split('=');
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                if k == "lang" || k == "locale" {
                    return Locale::from_str_loose(v);
                }
            }
        }
    }

    // 2. Priority 2: Cookie header (locale=ar or locale=en)
    if let Some(cookie_val) = cookie {
        for item in cookie_val.split(';') {
            let mut kv = item.trim().split('=');
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                if k == "locale" {
                    return Locale::from_str_loose(v);
                }
            }
        }
    }

    // 3. Priority 3: Accept-Language header
    if let Some(accept) = accept_language {
        let accept_lower = accept.to_ascii_lowercase();
        let ar_pos = accept_lower.find("ar");
        let en_pos = accept_lower.find("en");

        match (ar_pos, en_pos) {
            (Some(a), Some(e)) => {
                if a < e {
                    return Locale::Ar;
                } else {
                    return Locale::En;
                }
            }
            (Some(_), None) => return Locale::Ar,
            (None, Some(_)) => return Locale::En,
            (None, None) => {}
        }
    }

    // 4. Default: English
    Locale::En
}

/// Server-side resolution of locale from request parts.
#[cfg(feature = "ssr")]
pub fn resolve_request_locale() -> Locale {
    use axum::http::header::{ACCEPT_LANGUAGE, COOKIE};
    use axum::http::request::Parts;
    use leptos::prelude::use_context;

    if let Some(parts) = use_context::<Parts>() {
        let path = parts.uri.path();
        let query = parts.uri.query();
        let cookie = parts.headers.get(COOKIE).and_then(|v| v.to_str().ok());
        let accept = parts.headers.get(ACCEPT_LANGUAGE).and_then(|v| v.to_str().ok());

        resolve_locale_from_inputs(path, query, cookie, accept)
    } else {
        Locale::En
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_priority() {
        // Path /ar overrides English cookies and accept-language
        assert_eq!(
            resolve_locale_from_inputs("/ar/services", None, Some("locale=en"), Some("en-US")),
            Locale::Ar
        );
        // Path /en overrides Arabic cookies and accept-language
        assert_eq!(
            resolve_locale_from_inputs("/en/about", None, Some("locale=ar"), Some("ar-SA")),
            Locale::En
        );
    }

    #[test]
    fn test_query_priority() {
        // Query param ?lang=ar overrides English cookie
        assert_eq!(
            resolve_locale_from_inputs("/", Some("lang=ar"), Some("locale=en"), Some("en-US")),
            Locale::Ar
        );
        assert_eq!(
            resolve_locale_from_inputs("/", Some("locale=en"), Some("locale=ar"), Some("ar-SA")),
            Locale::En
        );
    }

    #[test]
    fn test_cookie_priority() {
        // Cookie overrides Accept-Language
        assert_eq!(
            resolve_locale_from_inputs("/", None, Some("locale=ar; session=123"), Some("en-US")),
            Locale::Ar
        );
        assert_eq!(
            resolve_locale_from_inputs("/", None, Some("locale=en"), Some("ar-SA,ar;q=0.9")),
            Locale::En
        );
    }

    #[test]
    fn test_accept_language_resolution() {
        // When no cookie or path is set, Accept-Language is used
        assert_eq!(
            resolve_locale_from_inputs("/", None, None, Some("ar-SA,ar;q=0.9,en;q=0.8")),
            Locale::Ar
        );
        assert_eq!(
            resolve_locale_from_inputs("/", None, None, Some("en-US,en;q=0.9,ar;q=0.8")),
            Locale::En
        );
        assert_eq!(
            resolve_locale_from_inputs("/", None, None, Some("fr-FR,fr;q=0.9")),
            Locale::En
        );
    }

    #[test]
    fn test_default_fallback() {
        assert_eq!(resolve_locale_from_inputs("/", None, None, None), Locale::En);
    }
}

