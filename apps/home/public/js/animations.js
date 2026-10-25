/**
 * TeebTech Homepage — GSAP Animation Bridge
 *
 * This module provides isolated GSAP + ScrollTrigger animation primitives
 * called from Rust/WASM via window-level functions.
 *
 * Dependencies loaded from CDN:
 * - gsap@3.12
 * - ScrollTrigger plugin
 */

(function () {
  "use strict";

  function waitForGsap(callback, attempts) {
    attempts = attempts || 0;
    if (window.gsap && window.ScrollTrigger) {
      callback();
    } else if (attempts < 50) {
      setTimeout(function () {
        waitForGsap(callback, attempts + 1);
      }, 50);
    }
  }

  waitForGsap(function () {
    try {
      gsap.registerPlugin(ScrollTrigger);
    } catch (e) {
      console.warn("GSAP registerPlugin warning:", e);
    }

    // ─── Hero Timeline ──────────────────────────────────────
    window.initHeroTimeline = function (containerSelector) {
      var container = document.querySelector(containerSelector);
      if (!container) return null;

      try {
        var words = container.querySelectorAll(".hero-heading .word");
        var badge = container.querySelector(".hero-badge");
        var subtitle = container.querySelector(".hero-subtitle");
        var cta = container.querySelector(".hero-cta");

        var tl = gsap.timeline({ delay: 0.2 });

        if (words && words.length > 0) {
          tl.fromTo(
            words,
            { y: 40, opacity: 0 },
            {
              y: 0,
              opacity: 1,
              duration: 0.8,
              stagger: 0.08,
              ease: "power3.out",
            }
          );
        }

        if (badge) {
          tl.fromTo(
            badge,
            { opacity: 0, y: 12 },
            { opacity: 1, y: 0, duration: 0.5, ease: "power3.out" },
            "-=0.4"
          );
        }

        if (subtitle) {
          tl.fromTo(
            subtitle,
            { opacity: 0, y: 16 },
            { opacity: 1, y: 0, duration: 0.6, ease: "power3.out" },
            "-=0.3"
          );
        }

        if (cta) {
          tl.fromTo(
            cta,
            { opacity: 0, y: 12 },
            { opacity: 1, y: 0, duration: 0.5, ease: "power3.out" },
            "-=0.2"
          );
        }

        // Hero scroll transformation
        gsap.to(containerSelector + " .hero-heading", {
          scale: 0.9,
          y: -40,
          opacity: 0.4,
          ease: "none",
          scrollTrigger: {
            trigger: containerSelector,
            start: "top top",
            end: "bottom top",
            scrub: 1.2,
          },
        });
      } catch (err) {
        console.warn("Hero timeline init error:", err);
      }

      return "hero-timeline";
    };

    // ─── Horizontal Scroll Section ──────────────────────────
    window.initHorizontalScroll = function (sectionSelector, trackSelector) {
      var section = document.querySelector(sectionSelector);
      var track = document.querySelector(trackSelector);
      if (!section || !track) return null;

      // Only on desktop viewports
      if (window.innerWidth < 768) return null;

      try {
        var trackWidth = track.scrollWidth;
        var viewportWidth = window.innerWidth;
        var scrollDistance = trackWidth - viewportWidth + 120;

        if (scrollDistance > 0) {
          gsap.to(track, {
            x: -scrollDistance,
            ease: "none",
            scrollTrigger: {
              trigger: sectionSelector,
              start: "top top",
              end: "+=" + scrollDistance,
              pin: true,
              scrub: 1,
              invalidateOnRefresh: true,
            },
          });
        }
      } catch (err) {
        console.warn("Horizontal scroll init error:", err);
      }

      return "horizontal-scroll";
    };

    // ─── Reveal Animations ──────────────────────────────────
    window.initRevealAnimations = function (selector) {
      var elements = document.querySelectorAll(selector);
      if (!elements.length) return;

      elements.forEach(function (el) {
        try {
          ScrollTrigger.create({
            trigger: el,
            start: "top 88%",
            onEnter: function () {
              el.classList.add("revealed");
            },
          });
        } catch (e) {}
      });
    };

    // ─── Manifesto / About word reveal ──────────────────────
    window.initManifestoReveal = function (containerSelector) {
      var container = document.querySelector(containerSelector);
      if (!container) return null;

      try {
        var words = container.querySelectorAll(".manifesto-word");
        words.forEach(function (word, i) {
          gsap.fromTo(
            word,
            { opacity: 0.15 },
            {
              opacity: 1,
              duration: 0.4,
              ease: "power2.out",
              scrollTrigger: {
                trigger: containerSelector,
                start: "top+=" + (i * 18) + " 65%",
                toggleActions: "play none none reverse",
              },
            }
          );
        });
      } catch (err) {
        console.warn("Manifesto reveal error:", err);
      }

      return "manifesto-reveal";
    };

    // ─── CTA Section reveal ─────────────────────────────────
    window.initCtaReveal = function (containerSelector) {
      var container = document.querySelector(containerSelector);
      if (!container) return null;

      try {
        var lines = container.querySelectorAll(".cta-line");
        lines.forEach(function (line, i) {
          gsap.fromTo(
            line,
            { opacity: 0, y: 40 },
            {
              opacity: 1,
              y: 0,
              duration: 0.8,
              ease: "power3.out",
              scrollTrigger: {
                trigger: containerSelector,
                start: "top 75%",
                toggleActions: "play none none none",
              },
              delay: i * 0.12,
            }
          );
        });
      } catch (err) {
        console.warn("CTA reveal error:", err);
      }

      return "cta-reveal";
    };

    // ─── Refresh ScrollTrigger ──────────────────────────────
    window.refreshScrollTrigger = function () {
      if (window.ScrollTrigger) {
        ScrollTrigger.refresh();
      }
    };

    // ─── Cleanup all ────────────────────────────────────────
    window.cleanupAllAnimations = function () {
      if (window.ScrollTrigger) {
        ScrollTrigger.getAll().forEach(function (st) {
          st.kill();
        });
      }
      if (window.gsap) {
        gsap.killTweensOf("*");
      }
    };
  });
})();
