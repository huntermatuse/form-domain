use crate::forms::model::{
    CompletedForm, FormSubmission, Question, QuestionKind, QuestionResponse, Response,
};
use crate::forms::render::display::{
    choice_label, display_or_empty, response_for, validation_status_label,
};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;
use web_sys::window;

const PRINT_CSS: Asset = asset!("/assets/css/pages/print.css", AssetOptions::css());

#[component]
pub fn PrintCompletedFormMount(completed_form: CompletedForm) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PRINT_CSS }
        div { class: "completed-form-print-mount", aria_hidden: "true",
            CompletedFormPrintDocument { completed_form }
        }
    }
}

#[component]
pub fn CompletedFormPrintDocument(completed_form: CompletedForm) -> Element {
    let form = &completed_form.form;

    rsx! {
        article { class: "completed-form-print-document",
            header { class: "completed-form-print-header",
                h1 { "{form.title}" }
                div { class: "completed-form-print-meta",
                    span { "Form: {form.form_id}" }
                    span { "Version: {form.version}" }
                    span { "Completed form: {completed_form.completed_form_id}" }
                }
            }

            SubmissionSummary { submission: completed_form.submission.clone() }

            if let Some(description) = &form.description_markdown {
                div { class: "completed-form-print-description",
                    MarkdownDescription { markdown: description.clone() }
                }
            }

            for section in form.sections.iter() {
                section {
                    key: "{section.section_id}",
                    class: "completed-form-print-section",

                    div { class: "completed-form-print-section-intro",
                        h2 { "{section.number}. {section.title}" }

                        if let Some(description) = &section.description_markdown {
                            MarkdownDescription { markdown: description.clone() }
                        }
                    }

                    for question in section.questions.iter() {
                        PrintQuestion {
                            key: "{question.question_id}",
                            question: question.clone(),
                            response: response_for(&completed_form.responses, &question.question_id).cloned(),
                        }
                    }
                }
            }
        }
    }
}

pub fn print_current_document() {
    if let Some(window) = window() {
        let _ = window.print();
    }
}

#[component]
fn SubmissionSummary(submission: FormSubmission) -> Element {
    rsx! {
        section { class: "completed-form-print-summary",
            h2 { "Submission" }

            dl { class: "completed-form-print-summary-grid",
                DetailItem { label: "Company", value: submission.company_name }
                DetailItem { label: "Signer", value: submission.signer_name }
                DetailItem { label: "Title", value: submission.signer_title }
                DetailItem { label: "Submitted", value: submission.submitted_at }
            }
        }
    }
}

#[component]
fn DetailItem(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "completed-form-print-detail",
            dt { "{label}" }
            dd { "{display_or_empty(&value)}" }
        }
    }
}

#[component]
fn PrintQuestion(question: Question, response: Option<QuestionResponse>) -> Element {
    let is_content_block = matches!(&question.kind, QuestionKind::ContentBlock { .. });
    let class = if is_content_block {
        "completed-form-print-question completed-form-print-question--content"
    } else {
        "completed-form-print-question"
    };

    rsx! {
        article { class, id: "print-question-{question.question_id}",
            div { class: "completed-form-print-question-heading",
                h3 { "{question.number}. {question.title}" }

                if question.required {
                    span { class: "completed-form-print-required", "Required" }
                }
            }

            QuestionDescription { question: question.clone() }

            if !is_content_block {
                div { class: "completed-form-print-answer",
                    h4 { "Response" }

                    match response.as_ref().map(|response| &response.response) {
                        Some(response) => rsx! {
                            PrintResponseValue { question: question.clone(), response: response.clone() }
                        },
                        None => rsx! {
                            p { class: "completed-form-print-missing", "No response provided." }
                        },
                    }

                    if let Some(answered_at) = response
                        .as_ref()
                        .and_then(|response| response.answered_at.as_ref())
                    {
                        p { class: "completed-form-print-answer-meta", "Answered: {answered_at}" }
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionDescription(question: Question) -> Element {
    rsx! {
        match &question.kind {
            QuestionKind::Validation { description_markdown, confirm_prompt, .. } => {
                rsx! {
                    MarkdownDescription { markdown: description_markdown.clone() }

                    p { class: "completed-form-print-prompt",
                        strong { "Confirmation prompt: " }
                        "{confirm_prompt}"
                    }
                }
            }
            QuestionKind::ContentBlock { content_markdown } => rsx! {
                MarkdownDescription { markdown: content_markdown.clone() }
            },
            QuestionKind::Text { description_markdown, .. }
            | QuestionKind::Choice { description_markdown, .. }
            | QuestionKind::MultiChoice { description_markdown, .. }
            | QuestionKind::Email { description_markdown, .. }
            | QuestionKind::Phone { description_markdown, .. }
            | QuestionKind::Date { description_markdown }
            | QuestionKind::Number { description_markdown, .. }
            | QuestionKind::Dropdown { description_markdown, .. }
            | QuestionKind::MultiDropdown { description_markdown, .. }
            | QuestionKind::RankedList { description_markdown, .. } => rsx! {
                if let Some(description_markdown) = description_markdown {
                    MarkdownDescription { markdown: description_markdown.clone() }
                }
            },
        }
    }
}

#[component]
fn PrintResponseValue(question: Question, response: Response) -> Element {
    rsx! {
        match response {
            Response::Validation { status, comment } => rsx! {
                p {
                    span { class: "completed-form-print-status", "{validation_status_label(&status)}" }
                }

                if let Some(comment) = comment {
                    PrintComment { comment }
                }
            },
            Response::Text { value } => rsx! {
                p { "{display_or_empty(&value)}" }
            },
            Response::Choice { selected_option_id, comment } => rsx! {
                p { "{choice_label(&question, &selected_option_id)}" }

                if let Some(comment) = comment {
                    PrintComment { comment }
                }
            },
            Response::MultiChoice { selected_option_ids, comment } => rsx! {
                if selected_option_ids.is_empty() {
                    p { class: "completed-form-print-missing", "No options selected." }
                } else {
                    ul {
                        for option_id in selected_option_ids.iter() {
                            li { key: "{option_id}", "{choice_label(&question, option_id)}" }
                        }
                    }
                }

                if let Some(comment) = comment {
                    PrintComment { comment }
                }
            },
            Response::RankedList { ranked_option_ids } => rsx! {
                if ranked_option_ids.is_empty() {
                    p { class: "completed-form-print-missing", "Not ranked." }
                } else {
                    ol {
                        for option_id in ranked_option_ids.iter() {
                            li { key: "{option_id}", "{choice_label(&question, option_id)}" }
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn PrintComment(comment: String) -> Element {
    rsx! {
        div { class: "completed-form-print-comment",
            strong { "Comment" }
            p { "{comment}" }
        }
    }
}
