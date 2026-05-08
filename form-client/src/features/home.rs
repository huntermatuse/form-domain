use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum TokenMode {
    Submission,
    Viewer,
}

impl TokenMode {
    fn label(self) -> &'static str {
        match self {
            TokenMode::Submission => "submission",
            TokenMode::Viewer => "viewer",
        }
    }
}

#[component]
pub fn HomePage() -> Element {
    let navigator = use_navigator();
    let mut mode = use_signal(|| None::<TokenMode>);
    let mut token = use_signal(String::new);

    let selected_mode = *mode.read();
    let token_value = token.read().trim().to_string();
    let can_open = !token_value.is_empty();

    rsx! {
        main { class: "home-shell",
            section { class: "home-brand-panel", aria_label: "Forms home" }

            section { class: "home-action-panel", aria_label: "Open a form link",
                match selected_mode {
                    None => rsx! {
                        nav { class: "home-mode-nav", aria_label: "Choose destination",
                            button {
                                class: "home-action-button",
                                r#type: "button",
                                onclick: move |_| {
                                    navigator.push(Route::AdminLoginPage {});
                                },
                                "admin"
                            }
                            button {
                                class: "home-action-button",
                                r#type: "button",
                                onclick: move |_| mode.set(Some(TokenMode::Submission)),
                                "submission"
                            }
                            button {
                                class: "home-action-button",
                                r#type: "button",
                                onclick: move |_| mode.set(Some(TokenMode::Viewer)),
                                "viewer"
                            }
                        }
                    },
                    Some(selected_mode) => rsx! {
                        form {
                            class: "home-token-form",
                            onsubmit: move |event| {
                                event.prevent_default();
                                let token = token.read().trim().to_string();
                                if token.is_empty() {
                                    return;
                                }

                                match *mode.read() {
                                    Some(TokenMode::Submission) => {
                                        navigator.push(Route::PlasmaRfiPage { token });
                                    }
                                    Some(TokenMode::Viewer) => {
                                        navigator
                                            .push(Route::PublicCompletedFormViewerPage {
                                                token,
                                            });
                                    }
                                    None => {}
                                }
                            },
                            label { class: "home-token-label", r#for: "home-token", "{selected_mode.label()} token" }
                            input {
                                id: "home-token",
                                class: "home-token-input",
                                r#type: "text",
                                autocomplete: "off",
                                spellcheck: "false",
                                placeholder: "paste token here",
                                value: "{token}",
                                oninput: move |event| token.set(event.value()),
                            }
                            div { class: "home-token-actions",
                                button {
                                    class: "home-action-button home-back-button",
                                    r#type: "button",
                                    onclick: move |_| {
                                        token.set(String::new());
                                        mode.set(None);
                                    },
                                    "back"
                                }
                                button {
                                    class: "home-action-button home-open-button",
                                    r#type: "submit",
                                    disabled: !can_open,
                                    "open {selected_mode.label()}"
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
