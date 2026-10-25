//! # Internationalization (i18n) Foundation
//!
//! Provides core locale and text direction types for TeebTech applications.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Supported locales across TeebTech web platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Locale {
    #[default]
    En,
    Ar,
}

/// Text and layout reading direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Locale {
    /// Two-letter BCP-47 language tag.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ar => "ar",
        }
    }

    /// Primary text reading direction for this locale.
    pub fn direction(&self) -> Direction {
        match self {
            Self::En => Direction::Ltr,
            Self::Ar => Direction::Rtl,
        }
    }

    /// HTML `dir` attribute value ("ltr" or "rtl").
    pub fn dir_str(&self) -> &'static str {
        match self.direction() {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
        }
    }

    /// Native localized name of this locale.
    pub fn label(&self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ar => "العربية",
        }
    }

    /// Label shown on the language switcher to switch TO the alternative language.
    pub fn switcher_target_label(&self) -> &'static str {
        match self {
            Self::En => "العربية",
            Self::Ar => "English",
        }
    }

    /// Toggle between supported languages.
    pub fn toggle(&self) -> Self {
        match self {
            Self::En => Self::Ar,
            Self::Ar => Self::En,
        }
    }

    /// Parse locale from string (e.g. from cookie, URL, or accept-language).
    pub fn from_str_loose(s: &str) -> Self {
        let trimmed = s.trim().to_ascii_lowercase();
        if trimmed.starts_with("ar") {
            Self::Ar
        } else {
            Self::En
        }
    }

    pub fn is_arabic(&self) -> bool {
        matches!(self, Self::Ar)
    }

    pub fn is_english(&self) -> bool {
        matches!(self, Self::En)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_defaults_and_attributes() {
        let en = Locale::En;
        assert_eq!(en.as_str(), "en");
        assert_eq!(en.direction(), Direction::Ltr);
        assert_eq!(en.dir_str(), "ltr");
        assert_eq!(en.label(), "English");
        assert_eq!(en.switcher_target_label(), "العربية");
        assert!(en.is_english());
        assert!(!en.is_arabic());

        let ar = Locale::Ar;
        assert_eq!(ar.as_str(), "ar");
        assert_eq!(ar.direction(), Direction::Rtl);
        assert_eq!(ar.dir_str(), "rtl");
        assert_eq!(ar.label(), "العربية");
        assert_eq!(ar.switcher_target_label(), "English");
        assert!(ar.is_arabic());
        assert!(!ar.is_english());
    }

    #[test]
    fn test_locale_toggle() {
        assert_eq!(Locale::En.toggle(), Locale::Ar);
        assert_eq!(Locale::Ar.toggle(), Locale::En);
    }

    #[test]
    fn test_from_str_loose() {
        assert_eq!(Locale::from_str_loose("ar"), Locale::Ar);
        assert_eq!(Locale::from_str_loose("ar-SA"), Locale::Ar);
        assert_eq!(Locale::from_str_loose("AR_AE"), Locale::Ar);
        assert_eq!(Locale::from_str_loose("arabic"), Locale::Ar);

        assert_eq!(Locale::from_str_loose("en"), Locale::En);
        assert_eq!(Locale::from_str_loose("en-US"), Locale::En);
        assert_eq!(Locale::from_str_loose("fr"), Locale::En);
        assert_eq!(Locale::from_str_loose(""), Locale::En);
        assert_eq!(Locale::from_str_loose("unknown"), Locale::En);
    }
}

