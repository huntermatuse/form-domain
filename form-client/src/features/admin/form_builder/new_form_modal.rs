use super::helpers::default_section;
use super::state::BuilderDraft;
use crate::api;
use dioxus::prelude::*;

#[component]
pub(super) fn NewFormModal(
    on_confirm: EventHandler<BuilderDraft>,
    on_cancel: EventHandler<MouseEvent>,
) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut created_by = use_signal(String::new);
    let mut title_error = use_signal(|| None::<String>);
    let mut checking = use_signal(|| false);
    let mut title_available = use_signal(|| true);

    rsx! {
        div { class: "modal-backdrop",
            div { class: "modal",
                h2 { class: "modal-title", "New form" }

                label { class: "modal-label",
                    "Form name"
                    div { class: "modal-field-row",
                        input {
                            class: "modal-input",
                            r#type: "text",
                            placeholder: "e.g. Vendor Onboarding 2025",
                            value: "{title.read()}",
                            oninput: move |e| {
                                let v = e.value();
                                title.set(v.clone());
                                title_error.set(None);
                                title_available.set(true);
                                if !v.trim().is_empty() {
                                    checking.set(true);
                                    spawn(async move {
                                        match api::admin::check_form_title_available(v.trim()).await {
                                            Ok(available) => {
                                                title_available.set(available);
                                                if !available {
                                                    title_error
                                                        .set(Some("That name is already taken.".to_string()));
                                                }
                                            }
                                            Err(_) => {}
                                        }
                                        checking.set(false);
                                    });
                                }
                            },
                        }
                        if *checking.read() {
                            span { class: "modal-field-hint", "Checking..." }
                        } else if *title_available.read() && !title.read().trim().is_empty() {
                            span { class: "modal-field-hint modal-field-hint--ok", "Available" }
                        }
                    }
                    if let Some(err) = title_error.read().as_ref() {
                        span { class: "modal-field-error", "{err}" }
                    }
                }

                label { class: "modal-label",
                    "Version"
                    input {
                        class: "modal-input modal-input--readonly",
                        r#type: "text",
                        value: "1",
                        readonly: true,
                    }
                }

                label { class: "modal-label",
                    "Description (optional)"
                    textarea {
                        class: "modal-textarea",
                        placeholder: "Short description shown at the top of the form",
                        value: "{description.read()}",
                        oninput: move |e| description.set(e.value()),
                    }
                }

                label { class: "modal-label",
                    "Prepared by"
                    input {
                        class: "modal-input",
                        r#type: "text",
                        placeholder: "Team or person creating this form",
                        value: "{created_by.read()}",
                        oninput: move |e| created_by.set(e.value()),
                    }
                }

                div { class: "modal-actions",
                    button {
                        class: "admin-secondary-button",
                        r#type: "button",
                        onclick: move |e| on_cancel.call(e),
                        "Cancel"
                    }
                    button {
                        class: "admin-primary-button",
                        r#type: "button",
                        disabled: !*title_available.read() || title.read().trim().is_empty()
                            || created_by.read().trim().is_empty(),
                        onclick: move |_| {
                            let t = title.read().trim().to_string();
                            let cb = created_by.read().trim().to_string();
                            if t.is_empty() {
                                title_error.set(Some("Name is required.".to_string()));
                                return;
                            }
                            if cb.is_empty() {
                                return;
                            }
                            on_confirm
                                .call(BuilderDraft {
                                    title: t,
                                    description_markdown: description.read().clone(),
                                    created_by: cb,
                                    sections: vec![default_section(0)],
                                });
                        },
                        "Create form"
                    }
                }
            }
        }
    }
}
