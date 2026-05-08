use super::state::SubmissionDraft;
use dioxus::prelude::*;

#[component]
pub(super) fn SubmissionFields(draft: Signal<SubmissionDraft>) -> Element {
    let current = draft.read().clone();

    rsx! {
        section { class: "form-submission-details",
            h2 { "Submission" }

            div { class: "form-submission-details__grid",
                TextInput {
                    id: "company_name",
                    label: "Company",
                    input_type: "text",
                    value: current.company_name,
                    oninput: move |value| {
                        draft.write().company_name = value;
                    },
                }

                TextInput {
                    id: "signer_name",
                    label: "Signer name",
                    input_type: "text",
                    value: current.signer_name,
                    oninput: move |value| {
                        draft.write().signer_name = value;
                    },
                }

                TextInput {
                    id: "signer_title",
                    label: "Signer title",
                    input_type: "text",
                    value: current.signer_title,
                    oninput: move |value| {
                        draft.write().signer_title = value;
                    },
                }

                div { class: "field",
                    label { r#for: "submitted_at", "Submission date" }
                    input {
                        id: "submitted_at",
                        r#type: "date",
                        value: "{current.submitted_at}",
                        readonly: true,
                        class: "field__readonly",
                    }
                }
            }
        }
    }
}

#[component]
fn TextInput(
    id: &'static str,
    label: &'static str,
    input_type: &'static str,
    value: String,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "field",
            label { r#for: "{id}", "{label}" }

            input {
                id: "{id}",
                r#type: "{input_type}",
                value: "{value}",
                oninput: move |event| {
                    oninput.call(event.value());
                },
            }
        }
    }
}
