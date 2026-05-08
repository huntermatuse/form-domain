use crate::api;
use crate::api::http::{clear_auth_token, ApiError};
use crate::Route;
use dioxus::document::eval;
use dioxus::prelude::*;
use manganis::asset;

#[component]
pub fn AdminFrame(title: String, children: Element) -> Element {
    let navigator = use_navigator();
    let home_nav = navigator.clone();
    let forms_nav = navigator.clone();
    let new_nav = navigator.clone();
    let logout_nav = navigator.clone();

    use_effect(move || {
        let _ = eval(
            r#"
            (() => {
                const el = document.getElementById('admin-nav-clock');
                if (!el) return;
                const tick = () => {
                    const now = new Date();
                    el.textContent = now.toLocaleTimeString([], {
                        hour: '2-digit', minute: '2-digit', hour12: false
                    });
                };
                tick();
                if (window.__adminNavClock) clearInterval(window.__adminNavClock);
                window.__adminNavClock = setInterval(tick, 30000);
            })();
        "#,
        );
    });

    rsx! {
        div { class: "admin-app",
            nav { class: "admin-nav",
                button {
                    class: "admin-nav-brand",
                    r#type: "button",
                    onclick: move |_| {
                        home_nav.push(Route::AdminLandingPage {});
                    },
                    img {
                        class: "admin-nav-brand-mark",
                        src: asset!("/assets/icons/form-icons/web-app-manifest-512x512.png"),
                        alt: "",
                        aria_hidden: "true",
                    }
                    "Elev8 Forms"
                }
                span { class: "admin-nav-divider", aria_hidden: "true" }
                div { class: "admin-nav-links",
                    button {
                        class: "admin-nav-link",
                        r#type: "button",
                        onclick: move |_| {
                            forms_nav.push(Route::AdminFormListPage {});
                        },
                        "Forms"
                    }
                    button {
                        class: "admin-nav-link",
                        r#type: "button",
                        onclick: move |_| {
                            new_nav.push(Route::AdminFormBuilderPage {});
                        },
                        "New form"
                    }
                }
                button {
                    class: "admin-nav-signout",
                    r#type: "button",
                    onclick: move |_| {
                        let nav = logout_nav.clone();
                        spawn(async move {
                            let _ = api::admin::logout().await;
                            clear_auth_token();
                            nav.replace(Route::AdminLoginPage {});
                        });
                    },
                    "Sign out"
                }
                span {
                    id: "admin-nav-clock",
                    class: "admin-nav-clock",
                    aria_label: "Current time",
                }
            }
            main { class: "admin-main",
                h1 { class: "admin-page-title", "{title}" }
                {children}
            }
        }
    }
}

#[component]
pub fn AdminError(err: ApiError) -> Element {
    rsx! {
        p { class: "admin-error", "{err}" }
    }
}

#[component]
pub fn StatusPill(active: bool) -> Element {
    let class = if active {
        "admin-status admin-status--active"
    } else {
        "admin-status"
    };
    rsx! {
        span { class,
            if active {
                "Active"
            } else {
                "Inactive"
            }
        }
    }
}

pub fn increment_signal(mut signal: Signal<u32>) {
    let next = {
        let current = signal.read();
        *current + 1
    };
    signal.set(next);
}

pub fn optional_string(value: String) -> Option<String> {
    let value = value.trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

pub fn optional_datetime(value: String) -> Option<String> {
    optional_string(value).map(|value| format!("{value}:00Z"))
}
