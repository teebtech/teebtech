//! # Design System Tokens
//!
//! Color palette constants, typography specifications, radius, and layout tokens
//! aligned with the TeebTech reference design language.

/// Core brand, surface, and semantic color definitions.
pub struct ColorTokens;

impl ColorTokens {
    // Navy Backgrounds & Dark Surfaces
    pub const NAVY_950: &'static str = "#060c18";
    pub const NAVY_900: &'static str = "#0b1220";
    pub const NAVY_800: &'static str = "#0f1a2e";
    pub const NAVY_700: &'static str = "#152238";
    pub const NAVY_600: &'static str = "#1c2e4a";
    pub const NAVY_500: &'static str = "#24395e";

    // Brand Blues
    pub const BRAND_50: &'static str = "#eff6ff";
    pub const BRAND_100: &'static str = "#dbeafe";
    pub const BRAND_200: &'static str = "#bfdbfe";
    pub const BRAND_500: &'static str = "#3b82f6";
    pub const BRAND_600: &'static str = "#2563eb";
    pub const BRAND_700: &'static str = "#1d4ed8";
    pub const BRAND_800: &'static str = "#1e40af";
    pub const BRAND_900: &'static str = "#1e3a8a";

    // Neutral Slates
    pub const SLATE_50: &'static str = "#f8fafc";
    pub const SLATE_100: &'static str = "#f1f5f9";
    pub const SLATE_200: &'static str = "#e2e8f0";
    pub const SLATE_300: &'static str = "#cbd5e1";
    pub const SLATE_400: &'static str = "#94a3b8";
    pub const SLATE_500: &'static str = "#64748b";
    pub const SLATE_600: &'static str = "#475569";
    pub const SLATE_700: &'static str = "#334155";
    pub const SLATE_800: &'static str = "#1e293b";
    pub const SLATE_900: &'static str = "#0f172a";

    // Semantic Accents
    pub const EMERALD_500: &'static str = "#10b981";
    pub const EMERALD_600: &'static str = "#059669";
    pub const AMBER_500: &'static str = "#f59e0b";
    pub const RED_600: &'static str = "#dc2626";
    pub const CYAN_600: &'static str = "#0891b2";
}

/// Spacing and typography tokens.
pub struct LayoutTokens;

impl LayoutTokens {
    pub const CONTAINER_MAX_WIDTH: &'static str = "1280px";
    pub const FONT_DISPLAY: &'static str = "'Fraunces', Georgia, serif";
    pub const FONT_SANS: &'static str = "'Inter', system-ui, -apple-system, sans-serif";
    pub const FONT_MONO: &'static str = "'JetBrains Mono', 'Courier New', monospace";
}
