use crate::forms::model::{Question, QuestionKind};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

#[component]
pub(super) fn PreviewQuestion(question: Question) -> Element {
    rsx! {
        div { class: "form-question",
            div { class: "form-question__heading",
                h3 {
                    "{question.number}. "
                    if question.title.is_empty() {
                        em { style: "opacity:0.4", "Untitled question" }
                    } else {
                        "{question.title}"
                    }
                }
                if question.required {
                    span { class: "form-question__required", "Required" }
                }
            }
            PreviewQuestionBody { kind: question.kind.clone() }
        }
    }
}

#[component]
fn PreviewQuestionBody(kind: QuestionKind) -> Element {
    rsx! {
        match kind {
            QuestionKind::Text { description_markdown, placeholder, multiline, .. } => {
                let ph = placeholder.unwrap_or_default();
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    if multiline {
                        textarea { disabled: true, placeholder: "{ph}" }
                    } else {
                        input { r#type: "text", disabled: true, placeholder: "{ph}" }
                    }
                }
            }
            QuestionKind::Choice { description_markdown, options, .. } => rsx! {
                if let Some(desc) = description_markdown {
                    MarkdownDescription { markdown: desc }
                }
                div { class: "choice-list",
                    for option in options.iter() {
                        label { key: "{option.question_option_id}",
                            input { r#type: "radio", disabled: true }
                            "{option.label}"
                        }
                    }
                }
            },
            QuestionKind::MultiChoice { description_markdown, options, .. } => {
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    div { class: "choice-list",
                        for option in options.iter() {
                            label { key: "{option.question_option_id}",
                                input { r#type: "checkbox", disabled: true }
                                "{option.label}"
                            }
                        }
                    }
                }
            }
            QuestionKind::Validation { description_markdown, confirm_prompt, .. } => {
                rsx! {
                    MarkdownDescription { markdown: description_markdown }
                    div { class: "form-question__prompt",
                        strong { "Please confirm: " }
                        "{confirm_prompt}"
                    }
                    div { class: "radio-row",
                        label {
                            input { r#type: "radio", disabled: true }
                            "Confirmed"
                        }
                        label {
                            input { r#type: "radio", disabled: true }
                            "Not correct"
                        }
                    }
                }
            }
            QuestionKind::Email { description_markdown, placeholder } => {
                let ph = placeholder.unwrap_or_else(|| "email@example.com".to_string());
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    input { r#type: "text", disabled: true, placeholder: "{ph}" }
                }
            }
            QuestionKind::Phone { description_markdown, placeholder } => {
                let ph = placeholder.unwrap_or_else(|| "+1 (555) 000-0000".to_string());
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    input { r#type: "text", disabled: true, placeholder: "{ph}" }
                }
            }
            QuestionKind::Date { description_markdown } => rsx! {
                if let Some(desc) = description_markdown {
                    MarkdownDescription { markdown: desc }
                }
                input { r#type: "date", disabled: true }
            },
            QuestionKind::Number { description_markdown, placeholder, .. } => {
                let ph = placeholder.unwrap_or_default();
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    input { r#type: "text", disabled: true, placeholder: "{ph}" }
                }
            }
            QuestionKind::Dropdown { description_markdown, options, .. } => rsx! {
                if let Some(desc) = description_markdown {
                    MarkdownDescription { markdown: desc }
                }
                select { disabled: true,
                    option { value: "", "— Select —" }
                    for option in options.iter() {
                        option {
                            key: "{option.question_option_id}",
                            value: "{option.question_option_id}",
                            "{option.label}"
                        }
                    }
                }
            },
            QuestionKind::MultiDropdown { description_markdown, options, .. } => {
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    select { disabled: true, multiple: true,
                        for option in options.iter() {
                            option {
                                key: "{option.question_option_id}",
                                value: "{option.question_option_id}",
                                "{option.label}"
                            }
                        }
                    }
                }
            }
            QuestionKind::RankedList {
                description_markdown,
                options,
                randomize_initial_order,
            } => rsx! {
                if let Some(desc) = description_markdown {
                    MarkdownDescription { markdown: desc }
                }
                if randomize_initial_order {
                    p { class: "form-question__prompt", "Initial order randomized for respondents." }
                }
                div { class: "ranked-list-preview",
                    for (i, option) in options.iter().enumerate() {
                        div {
                            key: "{option.question_option_id}",
                            class: "ranked-list-preview__item",
                            span { class: "ranked-list-preview__rank", "{i + 1}" }
                            span { "{option.label}" }
                        }
                    }
                }
            },
            QuestionKind::ContentBlock { content_markdown } => rsx! {
                MarkdownDescription { markdown: content_markdown }
            },
        }
    }
}
