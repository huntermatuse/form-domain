use crate::features::admin::shared::AdminFrame;
use crate::Route;
use dioxus::prelude::*;

const LANDING_CSS: Asset = asset!(
    "/assets/css/pages/landing.css",
    AssetOptions::css().with_static_head(true)
);

#[component]
pub fn AdminLandingPage() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LANDING_CSS }
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
