use crate::forms::model::{
    CompletedForm, FormSubmission, Question, QuestionKind, QuestionResponse, Response,
};
use crate::forms::render::display::{
    choice_label, display_or_empty, response_for, validation_status_label,
};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;
use web_sys::window;

const PRINT_CSS: &str = r#"
@media screen {
  .completed-form-print-mount {
    display: none !important;
  }
}

@page {
  size: letter;
  margin: 0.55in;
}

@media print {
  body {
    margin: 0 !important;
    background: #fff !important;
    color: #111827 !important;
  }

  .public-viewer-screen,
  .completed-form-viewer,
  .admin-nav,
  .admin-page-title,
  .admin-page-actions,
  .admin-action-items {
    display: none !important;
  }

  .admin-app,
  .admin-main,
  .public-app,
  .wrap {
    display: block !important;
    margin: 0 !important;
    padding: 0 !important;
    max-width: none !important;
    min-height: 0 !important;
    background: #fff !important;
    color: #111827 !important;
  }

  .completed-form-print-mount {
    display: block !important;
  }
}

.completed-form-print-document {
  box-sizing: border-box;
  width: 100%;
  color: #111827;
  background: #fff;
  font-family: Arial, Helvetica, sans-serif;
  font-size: 10.5pt;
  line-height: 1.38;
}

.completed-form-print-document * {
  box-sizing: border-box;
}

.completed-form-print-header {
  border-bottom: 1px solid #9ca3af;
  margin-bottom: 16px;
  padding-bottom: 12px;
}

.completed-form-print-header h1 {
  margin: 0 0 6px;
  color: #111827;
  font-size: 18pt;
  line-height: 1.15;
}

.completed-form-print-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 6px 16px;
  margin: 0;
  color: #4b5563;
  font-size: 8.5pt;
}

.completed-form-print-summary {
  break-inside: avoid;
  page-break-inside: avoid;
  margin: 0 0 16px;
  padding: 10px 0;
  border-bottom: 1px solid #d1d5db;
}

.completed-form-print-summary h2,
.completed-form-print-section > h2 {
  margin: 0 0 8px;
  color: #111827;
  font-size: 12pt;
  line-height: 1.2;
}

.completed-form-print-summary-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 7px 20px;
  margin: 0;
}

.completed-form-print-detail {
  min-width: 0;
}

.completed-form-print-detail dt {
  margin: 0 0 2px;
  color: #6b7280;
  font-size: 7.5pt;
  font-weight: 700;
  text-transform: uppercase;
}

.completed-form-print-detail dd {
  margin: 0;
  overflow-wrap: anywhere;
  color: #111827;
  font-size: 10pt;
}

.completed-form-print-description {
  margin: 0 0 16px;
}

.completed-form-print-section {
  break-before: auto;
  margin: 0 0 16px;
}

.completed-form-print-section > h2 {
  break-after: avoid;
  page-break-after: avoid;
  padding-bottom: 5px;
  border-bottom: 1px solid #d1d5db;
}

.completed-form-print-section-intro {
  break-after: avoid;
  break-inside: avoid;
  break-inside: avoid-page;
  page-break-after: avoid;
  page-break-inside: avoid;
}

.completed-form-print-question {
  break-inside: avoid;
  break-inside: avoid-page;
  page-break-inside: avoid;
  margin: 0;
  padding: 10px 0;
  border-bottom: 1px solid #e5e7eb;
}

.completed-form-print-question--content {
  break-inside: avoid;
  break-inside: avoid-page;
  page-break-inside: avoid;
}

.completed-form-print-question-heading {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 6px;
  break-after: avoid;
  page-break-after: avoid;
  margin-bottom: 5px;
}

.completed-form-print-question-heading h3 {
  margin: 0;
  color: #111827;
  font-size: 10.5pt;
  line-height: 1.25;
}

.completed-form-print-required {
  color: #374151;
  font-size: 7.5pt;
  font-weight: 700;
  text-transform: uppercase;
}

.completed-form-print-prompt {
  margin: 4px 0 0;
  color: #374151;
  font-size: 9pt;
}

.completed-form-print-answer {
  break-inside: auto;
  page-break-inside: auto;
  margin-top: 7px;
  padding: 8px 10px;
  border: 1px solid #d1d5db;
  background: #f9fafb;
}

.completed-form-print-answer h4 {
  margin: 0 0 4px;
  color: #6b7280;
  font-size: 7.5pt;
  text-transform: uppercase;
}

.completed-form-print-answer p,
.completed-form-print-answer li {
  margin: 0;
  overflow-wrap: anywhere;
}

.completed-form-print-answer ul,
.completed-form-print-answer ol {
  margin: 0;
  padding-left: 18px;
}

.completed-form-print-answer li + li {
  margin-top: 2px;
}

.completed-form-print-status {
  font-weight: 700;
}

.completed-form-print-missing,
.completed-form-print-answer-meta {
  color: #6b7280;
  font-style: italic;
}

.completed-form-print-answer-meta {
  margin-top: 6px !important;
  font-size: 8pt;
}

.completed-form-print-comment {
  margin-top: 7px;
  padding-left: 8px;
  border-left: 2px solid #9ca3af;
}

.completed-form-print-comment strong {
  display: block;
  margin-bottom: 2px;
  color: #4b5563;
  font-size: 7.5pt;
  text-transform: uppercase;
}

.completed-form-print-comment p {
  margin: 0;
  overflow-wrap: anywhere;
}

.completed-form-print-document .markdown {
  margin: 0;
}

.completed-form-print-document .markdown p,
.completed-form-print-document .markdown li {
  margin: 0 0 5px;
  orphans: 3;
  widows: 3;
}

.completed-form-print-document .markdown ul {
  margin: 0 0 6px;
  padding-left: 18px;
}

.completed-form-print-document .markdown h2,
.completed-form-print-document .markdown h3 {
  break-after: avoid;
  page-break-after: avoid;
  margin: 8px 0 5px;
  color: #111827;
  font-size: 10.5pt;
}

.completed-form-print-document .markdown blockquote {
  margin: 7px 0;
  padding: 6px 8px;
  border-left: 2px solid #9ca3af;
  background: #f9fafb;
}

.completed-form-print-document .markdown blockquote p {
  margin: 0;
}
"#;

#[component]
pub fn PrintCompletedFormMount(completed_form: CompletedForm) -> Element {
    rsx! {
        document::Style { {PRINT_CSS} }
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
                            response: response_for(
                                &completed_form.responses,
                                &question.question_id,
                            ).cloned(),
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
                            PrintResponseValue {
                                question: question.clone(),
                                response: response.clone(),
                            }
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
