use crate::forms::model::{
    CompletedForm, FormSubmission, Question, QuestionKind, QuestionOption, QuestionResponse,
    Response, ValidationStatus,
};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

#[component]
pub fn CompletedFormViewer(completed_form: CompletedForm) -> Element {
    let form = &completed_form.form;

    rsx! {
        div { class: "completed-form-viewer",
            header { class: "completed-form-viewer__header",
                h1 { "{form.title}" }

                if let Some(description) = &form.description_markdown {
                    MarkdownDescription {
                        markdown: description.clone(),
                    }
                }
            }

            SubmissionSummary {
                submission: completed_form.submission.clone(),
            }

            for section in form.sections.iter() {
                section {
                    key: "{section.section_id}",
                    class: "completed-form-section",

                    h2 { "{section.number}. {section.title}" }

                    if let Some(description) = &section.description_markdown {
                        MarkdownDescription {
                            markdown: description.clone(),
                        }
                    }

                    for question in section.questions.iter() {
                        QuestionAnswer {
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

#[component]
fn SubmissionSummary(submission: FormSubmission) -> Element {
    rsx! {
        section { class: "completed-form-submission",
            h2 { "Submission" }

            dl { class: "completed-form-submission__grid",
                DetailItem {
                    label: "Company",
                    value: submission.company_name,
                }

                DetailItem {
                    label: "Signer",
                    value: submission.signer_name,
                }

                DetailItem {
                    label: "Title",
                    value: submission.signer_title,
                }

                DetailItem {
                    label: "Submitted",
                    value: submission.submitted_at,
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

            QuestionDescription {
                question: question.clone(),
            }

            div { class: "completed-form-answer",
                h4 { "Response" }

                match response.as_ref().map(|response| &response.response) {
                    Some(response) => rsx! {
                        ResponseValue {
                            question: question.clone(),
                            response: response.clone(),
                        }
                    },
                    None => rsx! {
                        p { class: "completed-form-answer__missing", "No response provided." }
                    },
                }

                if let Some(answered_at) = response.as_ref().and_then(|response| response.answered_at.as_ref()) {
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
            QuestionKind::Validation {
                description_markdown,
                confirm_prompt,
                ..
            } => rsx! {
                MarkdownDescription {
                    markdown: description_markdown.clone(),
                }

                p { class: "completed-form-question__prompt",
                    strong { "Confirmation prompt: " }
                    "{confirm_prompt}"
                }
            },
            QuestionKind::Text {
                description_markdown,
                ..
            }
            | QuestionKind::Choice {
                description_markdown,
                ..
            }
            | QuestionKind::MultiChoice {
                description_markdown,
                ..
            } => rsx! {
                if let Some(description_markdown) = description_markdown {
                    MarkdownDescription {
                        markdown: description_markdown.clone(),
                    }
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
                    AnswerComment {
                        comment,
                    }
                }
            },
            Response::Text { value } => rsx! {
                p { "{display_or_empty(&value)}" }
            },
            Response::Choice {
                selected_option_id,
                comment,
            } => rsx! {
                p { "{choice_label(&question, &selected_option_id)}" }

                if let Some(comment) = comment {
                    AnswerComment {
                        comment,
                    }
                }
            },
            Response::MultiChoice {
                selected_option_ids,
                comment,
            } => rsx! {
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
                    AnswerComment {
                        comment,
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

fn response_for<'a>(
    responses: &'a [QuestionResponse],
    question_id: &str,
) -> Option<&'a QuestionResponse> {
    responses
        .iter()
        .find(|response| response.question_id == question_id)
}

fn validation_status_label(status: &ValidationStatus) -> &'static str {
    match status {
        ValidationStatus::Confirmed => "Confirmed",
        ValidationStatus::NotCorrect => "Not correct",
    }
}

fn choice_label(question: &Question, option_id: &str) -> String {
    question_options(question)
        .iter()
        .find(|option| option.question_option_id == option_id)
        .map(|option| option.label.clone())
        .unwrap_or_else(|| option_id.to_string())
}

fn question_options(question: &Question) -> &[QuestionOption] {
    match &question.kind {
        QuestionKind::Choice { options, .. } | QuestionKind::MultiChoice { options, .. } => options,
        _ => &[],
    }
}

fn display_or_empty(value: &str) -> &str {
    if value.trim().is_empty() {
        "Not provided"
    } else {
        value
    }
}
