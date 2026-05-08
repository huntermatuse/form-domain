use crate::forms::model::{
    CompletedForm, FormSubmission, Question, QuestionKind, QuestionResponse, Response,
};
use crate::forms::render::display::{
    choice_label, display_or_empty, response_for, validation_status_label,
};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

const VIEWER_CSS: &str = r#"
.completed-form-viewer {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.completed-form-viewer__header {
    background: #151922;
    border: 1px solid #2d3340;
    border-radius: 10px;
    padding: 20px 24px;
}

.completed-form-viewer__header h1 {
    margin: 0 0 6px;
    font-size: 1.5rem;
    font-weight: 700;
    color: #f7f8fb;
}

.completed-form-submission {
    background: #151922;
    border: 1px solid #2d3340;
    border-radius: 10px;
    padding: 20px 24px;
}

.completed-form-submission h2 {
    margin: 0 0 16px;
    font-size: 0.8rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: #8a95a8;
}

.completed-form-submission__grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 16px;
    margin: 0;
}

.completed-form-detail dt {
    font-size: 0.78rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: #6a7888;
    margin-bottom: 4px;
}

.completed-form-detail dd {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 600;
    color: #d8e0ee;
}

.completed-form-section {
    background: #151922;
    border: 1px solid #2d3340;
    border-radius: 10px;
    padding: 20px 24px;
}

.completed-form-section > h2 {
    margin: 0 0 16px;
    font-size: 1.05rem;
    font-weight: 700;
    color: #c8d0e0;
    padding-bottom: 12px;
    border-bottom: 1px solid #252d3d;
}

.completed-form-question {
    padding: 16px 0;
    border-top: 1px solid #1e2535;
}

.completed-form-question:first-of-type {
    border-top: none;
    padding-top: 0;
}

.completed-form-question__heading {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    margin-bottom: 8px;
}

.completed-form-question__heading h3 {
    margin: 0;
    font-size: 0.97rem;
    font-weight: 700;
    color: #f0f4fb;
}

.completed-form-question__required {
    font-size: 0.72rem;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 999px;
    background: #1e2840;
    color: #7abbd8;
}

.completed-form-question__prompt {
    font-size: 0.9rem;
    color: #b8c0cc;
    margin: 6px 0 0;
}

.completed-form-answer {
    margin-top: 12px;
    padding: 12px 14px;
    background: #0f1116;
    border: 1px solid #252d3d;
    border-radius: 7px;
}

.completed-form-answer h4 {
    margin: 0 0 8px;
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #6a7888;
}

.completed-form-answer p,
.completed-form-answer li {
    margin: 0;
    font-size: 0.93rem;
    color: #c8d0e0;
}

.completed-form-answer ul {
    margin: 0;
    padding-left: 18px;
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.completed-form-answer__missing {
    color: #6a7888 !important;
    font-style: italic;
}

.completed-form-answer__status {
    font-weight: 700;
    color: #7abbd8;
}

.completed-form-answer__meta {
    margin-top: 8px !important;
    font-size: 0.78rem !important;
    color: #4a5568 !important;
}

.completed-form-answer__comment {
    margin-top: 10px;
    padding: 8px 12px;
    border-left: 3px solid #3a4352;
    background: #151922;
    border-radius: 0 6px 6px 0;
}

.completed-form-answer__comment strong {
    display: block;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: #6a7888;
    margin-bottom: 4px;
}

.completed-form-answer__comment p {
    font-size: 0.9rem;
    color: #b8c0cc;
}
"#;

#[component]
pub fn CompletedFormViewer(completed_form: CompletedForm) -> Element {
    let form = &completed_form.form;

    rsx! {
        document::Style { {VIEWER_CSS} }
        div { class: "completed-form-viewer",
            div { class: "completed-form-viewer__header",
                h1 { "{form.title}" }

                if let Some(description) = &form.description_markdown {
                    MarkdownDescription { markdown: description.clone() }
                }
            }

            SubmissionSummary { submission: completed_form.submission.clone() }

            for section in form.sections.iter() {
                section {
                    key: "{section.section_id}",
                    class: "completed-form-section",

                    h2 { "{section.number}. {section.title}" }

                    if let Some(description) = &section.description_markdown {
                        MarkdownDescription { markdown: description.clone() }
                    }

                    for question in section.questions.iter() {
                        QuestionAnswer {
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

#[component]
fn SubmissionSummary(submission: FormSubmission) -> Element {
    rsx! {
        section { class: "completed-form-submission",
            h2 { "Submission" }

            dl { class: "completed-form-submission__grid",
                DetailItem { label: "Company", value: submission.company_name }

                DetailItem { label: "Signer", value: submission.signer_name }

                DetailItem { label: "Title", value: submission.signer_title }

                DetailItem {
                    label: "Submitted",
                    value: submission.submitted_at.get(..10).unwrap_or(&submission.submitted_at).to_string(),
                }
            }
        }
    }
}

#[component]
fn DetailItem(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "completed-form-detail",
            dt { "{label}" }
            dd { "{display_or_empty(&value)}" }
        }
    }
}

#[component]
fn QuestionAnswer(question: Question, response: Option<QuestionResponse>) -> Element {
    rsx! {
        article {
            class: "completed-form-question",
            id: "question-{question.question_id}",

            div { class: "completed-form-question__heading",
                h3 { "{question.number}. {question.title}" }

                if question.required {
                    span { class: "completed-form-question__required", "Required" }
                }
            }

            QuestionDescription { question: question.clone() }

            div { class: "completed-form-answer",
                h4 { "Response" }

                match response.as_ref().map(|response| &response.response) {
                    Some(response) => rsx! {
                        ResponseValue { question: question.clone(), response: response.clone() }
                    },
                    None => rsx! {
                        p { class: "completed-form-answer__missing", "No response provided." }
                    },
                }

                if let Some(answered_at) = response
                    .as_ref()
                    .and_then(|response| response.answered_at.as_ref())
                {
                    p { class: "completed-form-answer__meta", "Answered: {answered_at}" }
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

                    p { class: "completed-form-question__prompt",
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
fn ResponseValue(question: Question, response: Response) -> Element {
    rsx! {
        match response {
            Response::Validation { status, comment } => rsx! {
                p {
                    span { class: "completed-form-answer__status", "{validation_status_label(&status)}" }
                }

                if let Some(comment) = comment {
                    AnswerComment { comment }
                }
            },
            Response::Text { value } => rsx! {
                p { "{display_or_empty(&value)}" }
            },
            Response::Choice { selected_option_id, comment } => rsx! {
                p { "{choice_label(&question, &selected_option_id)}" }

                if let Some(comment) = comment {
                    AnswerComment { comment }
                }
            },
            Response::MultiChoice { selected_option_ids, comment } => rsx! {
                if selected_option_ids.is_empty() {
                    p { class: "completed-form-answer__missing", "No options selected." }
                } else {
                    ul {
                        for option_id in selected_option_ids.iter() {
                            li { key: "{option_id}", "{choice_label(&question, option_id)}" }
                        }
                    }
                }

                if let Some(comment) = comment {
                    AnswerComment { comment }
                }
            },
            Response::RankedList { ranked_option_ids } => rsx! {
                if ranked_option_ids.is_empty() {
                    p { class: "completed-form-answer__missing", "Not ranked." }
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
fn AnswerComment(comment: String) -> Element {
    rsx! {
        div { class: "completed-form-answer__comment",
            strong { "Comment" }
            p { "{comment}" }
        }
    }
}
