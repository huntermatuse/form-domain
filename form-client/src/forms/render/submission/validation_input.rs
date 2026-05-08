use super::response_helpers::{
    current_validation_comment, current_validation_status, empty_string_as_none, response_for,
    set_validation_response,
};
use super::state::SubmissionDraft;
use crate::forms::model::{Response, ValidationStatus};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

#[component]
pub(super) fn ValidationQuestionInput(
    question_id: String,
    description_markdown: String,
    confirm_prompt: String,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let current = response_for(&draft.read().responses, &question_id).cloned();
    let status = match current.as_ref().map(|response| &response.response) {
        Some(Response::Validation { status, .. }) => Some(status.clone()),
        _ => None,
    };
    let comment = match current.as_ref().map(|response| &response.response) {
        Some(Response::Validation { comment, .. }) => comment.clone().unwrap_or_default(),
        _ => String::new(),
    };

    rsx! {
        MarkdownDescription { markdown: description_markdown }

        div { class: "form-question__prompt",
            strong { "Please confirm:" }
            " {confirm_prompt}"
        }

        div { class: "radio-row",
            label {
                input {
                    r#type: "radio",
                    name: "{question_id}",
                    checked: status == Some(ValidationStatus::Confirmed),
                    onchange: {
                        let question_id = question_id.clone();
                        move |_| {
                            let comment =
                                current_validation_comment(&draft.read().responses, &question_id);
                            set_validation_response(
                                draft,
                                question_id.clone(),
                                ValidationStatus::Confirmed,
                                comment,
                            );
                        }
                    },
                }

                "Confirmed"
            }

            label {
                input {
                    r#type: "radio",
                    name: "{question_id}",
                    checked: status == Some(ValidationStatus::NotCorrect),
                    onchange: {
                        let question_id = question_id.clone();
                        move |_| {
                            let comment =
                                current_validation_comment(&draft.read().responses, &question_id);
                            set_validation_response(
                                draft,
                                question_id.clone(),
                                ValidationStatus::NotCorrect,
                                comment,
                            );
                        }
                    },
                }

                "Not correct"
            }
        }

        textarea {
            placeholder: "Comments or corrections",
            value: "{comment}",
            oninput: {
                let question_id = question_id.clone();
                move |event| {
                    let status = current_validation_status(
                        &draft.read().responses,
                        &question_id,
                    );
                    if let Some(status) = status {
                        set_validation_response(
                            draft,
                            question_id.clone(),
                            status,
                            empty_string_as_none(event.value()),
                        );
                    }
                }
            },
        }
    }
}
