use crate::forms::model::{Form, Question, QuestionKind, QuestionOption};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

#[component]
pub fn ReadOnlyFormDefinition(form: Form) -> Element {
    rsx! {
        div { class: "admin-readonly-form",
            header {
                h2 { "{form.title}" }
                if let Some(description) = form.description_markdown.as_ref() {
                    MarkdownDescription { markdown: description.clone() }
                }
            }
            for section in form.sections.iter() {
                section {
                    class: "admin-readonly-section",
                    key: "{section.section_id}",
                    h3 { "{section.number}. {section.title}" }
                    if let Some(description) = section.description_markdown.as_ref() {
                        MarkdownDescription { markdown: description.clone() }
                    }
                    for question in section.questions.iter() {
                        article {
                            class: "admin-readonly-question",
                            key: "{question.question_id}",
                            h4 { "{question.number}. {question.title}" }
                            if question.required {
                                span { class: "admin-status admin-status--active", "Required" }
                            }
                            QuestionKindSummary { question: question.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionKindSummary(question: Question) -> Element {
    match question.kind {
        QuestionKind::Validation {
            description_markdown,
            confirm_prompt,
            summary_item,
        } => rsx! {
            MarkdownDescription { markdown: description_markdown }
            p {
                strong { "Prompt: " }
                "{confirm_prompt}"
            }
            p {
                strong { "Summary: " }
                "{summary_item}"
            }
        },
        QuestionKind::Text {
            description_markdown,
            placeholder,
            multiline,
            max_length,
        } => {
            let field_label = if multiline {
                "Text field (multiline)"
            } else {
                "Text field"
            };
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{field_label}" }
                if let Some(placeholder) = placeholder {
                    p {
                        strong { "Placeholder: " }
                        "{placeholder}"
                    }
                }
                if let Some(max_length) = max_length {
                    p {
                        strong { "Max length: " }
                        "{max_length}"
                    }
                }
            }
        }
        QuestionKind::Choice {
            description_markdown,
            options,
            allow_comment,
        } => {
            let label = if allow_comment {
                "Single choice with comment"
            } else {
                "Single choice"
            };
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{label}" }
                OptionList { options }
            }
        }
        QuestionKind::MultiChoice {
            description_markdown,
            options,
            min_selected,
            max_selected,
            allow_comment,
        } => {
            let label = if allow_comment {
                "Multi choice with comment"
            } else {
                "Multi choice"
            };
            let min = min_selected
                .map(|v| v.to_string())
                .unwrap_or_else(|| "0".to_string());
            let max = max_selected
                .map(|v| v.to_string())
                .unwrap_or_else(|| "any".to_string());
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{label}" }
                if min_selected.is_some() || max_selected.is_some() {
                    p { "Selection range: {min} to {max}" }
                }
                OptionList { options }
            }
        }
        QuestionKind::Email {
            description_markdown,
            placeholder,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            p { "Email field" }
            if let Some(ph) = placeholder {
                p {
                    strong { "Placeholder: " }
                    "{ph}"
                }
            }
        },
        QuestionKind::Phone {
            description_markdown,
            placeholder,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            p { "Phone field" }
            if let Some(ph) = placeholder {
                p {
                    strong { "Placeholder: " }
                    "{ph}"
                }
            }
        },
        QuestionKind::Date {
            description_markdown,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            p { "Date field" }
        },
        QuestionKind::Number {
            description_markdown,
            placeholder,
            min,
            max,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            p { "Number field" }
            if let Some(ph) = placeholder {
                p {
                    strong { "Placeholder: " }
                    "{ph}"
                }
            }
            if min.is_some() || max.is_some() {
                p {
                    strong { "Range: " }
                    "{min.map(|v| v.to_string()).unwrap_or_else(|| \"any\".to_string())} to {max.map(|v| v.to_string()).unwrap_or_else(|| \"any\".to_string())}"
                }
            }
        },
        QuestionKind::Dropdown {
            description_markdown,
            options,
            allow_comment,
        } => {
            let label = if allow_comment {
                "Dropdown with comment"
            } else {
                "Dropdown"
            };
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{label}" }
                OptionList { options }
            }
        }
        QuestionKind::MultiDropdown {
            description_markdown,
            options,
            min_selected,
            max_selected,
            allow_comment,
        } => {
            let label = if allow_comment {
                "Multi dropdown with comment"
            } else {
                "Multi dropdown"
            };
            let min = min_selected
                .map(|v| v.to_string())
                .unwrap_or_else(|| "0".to_string());
            let max = max_selected
                .map(|v| v.to_string())
                .unwrap_or_else(|| "any".to_string());
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{label}" }
                if min_selected.is_some() || max_selected.is_some() {
                    p { "Selection range: {min} to {max}" }
                }
                OptionList { options }
            }
        }
        QuestionKind::RankedList {
            description_markdown,
            options,
            randomize_initial_order,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            if randomize_initial_order {
                p { "Ranked list. Initial order randomized." }
            } else {
                p { "Ranked list. Initial order uses item order." }
            }
            OptionList { options }
        },
        QuestionKind::ContentBlock { content_markdown } => rsx! {
            MarkdownDescription { markdown: content_markdown }
        },
    }
}

#[component]
fn OptionList(options: Vec<QuestionOption>) -> Element {
    rsx! {
        ul {
            for option in options.iter() {
                li { key: "{option.question_option_id}", "{option.label}" }
            }
        }
    }
}
