use crate::api::http::has_auth_token;
use crate::features::admin::shared::AdminFrame;
use crate::Route;
use dioxus::prelude::*;

const LANDING_CSS: &str = r#"
.forms-home {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: calc(100dvh - 120px);
    padding: 40px 24px;
    gap: 48px;
}

.forms-home__heading {
    text-align: center;
}

.forms-kicker {
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: #3cc4dc;
    margin-bottom: 10px;
}

.forms-home__heading h1 {
    margin: 0;
    font-size: 2rem;
    font-weight: 700;
    color: #f7f8fb;
}

.forms-actions {
    display: flex;
    gap: 20px;
    flex-wrap: wrap;
    justify-content: center;
    width: 100%;
    max-width: 900px;
}

.forms-action {
    display: flex;
    flex-direction: column;
    gap: 16px;
    flex: 1;
    min-width: 220px;
    max-width: 280px;
    padding: 28px 24px 20px;
    background: #151922;
    border: 1px solid #2d3340;
    border-radius: 12px;
    color: #f7f8fb;
    text-decoration: none;
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s, transform 0.12s;
}

.forms-action:hover {
    border-color: #3cc4dc;
    background: #1a2030;
    transform: translateY(-2px);
}

.forms-action__icon {
    width: 36px;
    height: 36px;
    color: #3cc4dc;
    flex-shrink: 0;
}

.forms-action__label {
    font-size: 1.05rem;
    font-weight: 700;
    color: #f7f8fb;
    margin-bottom: 2px;
}

.forms-action__sub {
    font-size: 0.85rem;
    color: #8a95a8;
}

.forms-action__cta {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.82rem;
    font-weight: 700;
    color: #3cc4dc;
    margin-top: auto;
    padding-top: 12px;
    border-top: 1px solid #252d3d;
}

.arrow-icon {
    flex-shrink: 0;
}
"#;

#[component]
pub fn AdminLandingPage() -> Element {
    let navigator = use_navigator();

    if !has_auth_token() {
        navigator.replace(Route::AdminLoginPage {});
    }

    rsx! {
        document::Style { {LANDING_CSS} }
        AdminFrame { title: "Forms".to_string(),
            div { class: "forms-home",
                div { class: "forms-home__heading",
                    div { class: "forms-kicker", "Forms" }
                    h1 { "What would you like to do?" }
                }
                div { class: "forms-actions",
                    // New form
                    Link {
                        class: "forms-action",
                        to: Route::AdminFormBuilderPage {},
                        svg {
                            class: "forms-action__icon",
                            view_box: "0 0 28 28",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.4",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "4",
                                y: "4",
                                width: "20",
                                height: "20",
                                rx: "2",
                            }
                            line {
                                x1: "9",
                                y1: "10",
                                x2: "19",
                                y2: "10",
                            }
                            line {
                                x1: "9",
                                y1: "14",
                                x2: "15",
                                y2: "14",
                            }
                            line {
                                x1: "17",
                                y1: "17",
                                x2: "23",
                                y2: "17",
                            }
                            line {
                                x1: "20",
                                y1: "14",
                                x2: "20",
                                y2: "20",
                            }
                        }
                        div {
                            div { class: "forms-action__label", "Builder" }
                            div { class: "forms-action__sub", "Create a new form" }
                        }
                        span { class: "forms-action__cta",
                            "Get started"
                            ArrowIcon {}
                        }
                    }
                    // Manage forms
                    Link {
                        class: "forms-action",
                        to: Route::AdminFormListPage {},
                        svg {
                            class: "forms-action__icon",
                            view_box: "0 0 28 28",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.4",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "4",
                                y: "4",
                                width: "20",
                                height: "20",
                                rx: "2",
                            }
                            line {
                                x1: "9",
                                y1: "10",
                                x2: "19",
                                y2: "10",
                            }
                            line {
                                x1: "9",
                                y1: "14",
                                x2: "16",
                                y2: "14",
                            }
                            line {
                                x1: "9",
                                y1: "18",
                                x2: "13",
                                y2: "18",
                            }
                        }
                        div {
                            div { class: "forms-action__label", "Existing Forms" }
                            div { class: "forms-action__sub", "Browse and manage forms" }
                        }
                        span { class: "forms-action__cta",
                            "Browse"
                            ArrowIcon {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArrowIcon() -> Element {
    rsx! {
        svg {
            class: "arrow-icon",
            width: "14",
            height: "14",
            view_box: "0 0 14 14",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            stroke_linecap: "round",
            line {
                x1: "3",
                y1: "7",
                x2: "11",
                y2: "7",
            }
            polyline { points: "7,3 11,7 7,11" }
        }
    }
}
