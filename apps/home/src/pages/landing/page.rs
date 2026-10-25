//! # Homepage
//!
//! Matches UI reference Home.tsx and PublicLayout.tsx section order exactly:
//! 1. Header (Navbar with Services dropdown, CTAs, mobile menu)
//! 2. Hero (Navy dot-grid, badge, Fraunces h1, CTAs, trust signals, tech grid)
//! 3. Stats Bar (White, 4-column metrics with icons)
//! 4. Services (bg-slate-50, 12 cards in 4-column grid, All Services CTA)
//! 5. Process (How We Deliver, 5-step delivery framework with connector line)
//! 6. Case Studies (bg-slate-50, 3-column cards with images, badges, results, tags)
//! 7. Industries (12 domain expertise pill tags)
//! 8. Testimonials (bg-slate-50, 3 quote cards with star ratings and client avatars)
//! 9. CTA (hero-grid background, proposal & sales CTAs)
//! 10. Footer (bg-navy-900, brand info, 4 link columns, system status)

use crate::components::{Footer, Navbar};
use crate::pages::landing::sections::{
    CaseStudiesSection, CtaSection, Hero, IndustriesSection, ProcessSection, ServicesSection,
    TestimonialsSection,
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-slate-50 text-slate-900">
            // Navigation (Fixed Header)
            <Navbar />

            // Main content
            <main id="main-content" class="flex-1 pt-14 focus:outline-none" tabindex="-1">
                // 1. Hero
                <Hero />

                // 2. Services (What We Do)
                <ServicesSection />

                // 4. How We Work (Our Process)
                <ProcessSection />

                // 5. Case Studies
                <CaseStudiesSection />

                // 6. Industries
                <IndustriesSection />

                // 7. Testimonials (Client Feedback)
                <TestimonialsSection />

                // 8. CTA (Ready to start?)
                <CtaSection />
            </main>

            // Footer
            <Footer />
        </div>
    }
}
