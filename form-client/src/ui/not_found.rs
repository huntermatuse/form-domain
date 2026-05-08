use dioxus::prelude::*;

const SHELL_STYLE: &str = "position: relative; z-index: 1; width: 100%; min-height: 100dvh; height: 100%; overflow: hidden; background: var(--bg); color: var(--text); display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 0; font-family: 'Sora', sans-serif;";
const DIGITS_STYLE: &str = "font-family: 'DM Mono', monospace; font-weight: 300; font-size: clamp(120px, 18vw, 240px); letter-spacing: -0.06em; line-height: 1; color: var(--text); user-select: none;";
const TOKEN_STYLE: &str = "font-family: 'DM Mono', monospace; font-weight: 300; font-size: clamp(52px, 8vw, 104px); letter-spacing: 0; line-height: 1; color: var(--text); user-select: none;";
const DIGIT_STYLE: &str = "display: inline-block;";
const ACCENT_DIGIT_STYLE: &str = "display: inline-block; color: var(--accent-a);";
const DIVIDER_STYLE: &str =
    "width: 1px; height: 40px; background: var(--border); margin: 8px 0 16px;";
const MESSAGE_STYLE: &str = "font-size: clamp(11px, 1.2vw, 14px); font-weight: 300; letter-spacing: 0.18em; text-transform: uppercase; color: var(--muted); margin-bottom: 36px;";

pub fn simple_error_page(message: &'static str) -> Element {
    rsx! {
        div { aria_label: message, style: SHELL_STYLE,
            div { style: DIGITS_STYLE,
                span { style: DIGIT_STYLE, "4" }
                span { style: ACCENT_DIGIT_STYLE, "0" }
                span { style: DIGIT_STYLE, "4" }
            }
            div { style: DIVIDER_STYLE, aria_hidden: "true" }
            h1 { style: MESSAGE_STYLE, "{message}" }
        }
    }
}

pub fn token_unavailable_page() -> Element {
    rsx! {
        div { aria_label: "Token not found or Expired", style: SHELL_STYLE,
            div { style: TOKEN_STYLE,
                span { style: DIGIT_STYLE, "TOKEN ERROR" }
            }
            div { style: DIVIDER_STYLE, aria_hidden: "true" }
            h1 { style: MESSAGE_STYLE, "Token not found or Expired" }
        }
    }
}

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let _ = route;

    simple_error_page("Page not found")
}
