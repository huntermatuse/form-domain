use super::response_helpers::{response_for, upsert_response};
use super::state::SubmissionDraft;
use crate::forms::model::Response;
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

#[component]
pub(super) fn TextQuestionInput(
    question_id: String,
    description_markdown: Option<String>,
    placeholder: Option<String>,
    multiline: bool,
    max_length: Option<usize>,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let value = match response_for(&draft.read().responses, &question_id)
        .map(|response| &response.response)
    {
        Some(Response::Text { value }) => value.clone(),
        _ => String::new(),
    };
    let placeholder = placeholder.unwrap_or_default();

    rsx! {
        if let Some(description_markdown) = description_markdown {
            MarkdownDescription { markdown: description_markdown }
        }

        if multiline {
            textarea {
                placeholder: "{placeholder}",
                value: "{value}",
                maxlength: max_length.map(|length| length.to_string()),
                oninput: move |event| {
                    upsert_response(
                        draft,
                        question_id.clone(),
                        Response::Text {
                            value: event.value(),
                        },
                    );
                },
            }
        } else {
            input {
                r#type: "text",
                placeholder: "{placeholder}",
                value: "{value}",
                maxlength: max_length.map(|length| length.to_string()),
                oninput: move |event| {
                    upsert_response(
                        draft,
                        question_id.clone(),
                        Response::Text {
                            value: event.value(),
                        },
                    );
                },
            }
        }
    }
}

#[component]
pub(super) fn SimpleTextInput(
    question_id: String,
    description_markdown: Option<String>,
    placeholder: Option<String>,
    input_type: &'static str,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let value = match response_for(&draft.read().responses, &question_id)
        .map(|response| &response.response)
    {
        Some(Response::Text { value }) => value.clone(),
        _ => String::new(),
    };
    let placeholder = placeholder.unwrap_or_default();

    rsx! {
        if let Some(md) = description_markdown {
            MarkdownDescription { markdown: md }
        }
        input {
            r#type: "{input_type}",
            placeholder: "{placeholder}",
            value: "{value}",
            oninput: move |event| {
                upsert_response(
                    draft,
                    question_id.clone(),
                    Response::Text {
                        value: event.value(),
                    },
                );
            },
        }
    }
}

#[component]
pub(super) fn NumberQuestionInput(
    question_id: String,
    description_markdown: Option<String>,
    placeholder: Option<String>,
    min: Option<f64>,
    max: Option<f64>,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let value = match response_for(&draft.read().responses, &question_id)
        .map(|response| &response.response)
    {
        Some(Response::Text { value }) => value.clone(),
        _ => String::new(),
    };
    let placeholder = placeholder.unwrap_or_default();

    rsx! {
        if let Some(md) = description_markdown {
            MarkdownDescription { markdown: md }
        }
        input {
            r#type: "number",
            placeholder: "{placeholder}",
            value: "{value}",
            min: min.map(|v| v.to_string()),
            max: max.map(|v| v.to_string()),
            oninput: move |event| {
                upsert_response(
                    draft,
                    question_id.clone(),
                    Response::Text {
                        value: event.value(),
                    },
                );
            },
        }
    }
}
