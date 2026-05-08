use super::question::QuestionInput;
use super::response_helpers::build_completed_form;
use super::state::default_submission_draft;
use super::submission_fields::SubmissionFields;
use crate::forms::model::{CompletedForm, Form};
use crate::forms::render::markdown::MarkdownDescription;
use crate::forms::validation::{missing_required_questions, MissingRequiredQuestion};
use dioxus::prelude::*;

#[component]
pub fn FormSubmissionRenderer(
    form: Form,
    is_submitting: bool,
    on_submit: Option<EventHandler<CompletedForm>>,
) -> Element {
    let draft = use_signal(default_submission_draft);
    let mut missing_required = use_signal(Vec::<MissingRequiredQuestion>::new);

    rsx! {
        div { class: "form-submission",
            header { class: "form-submission__header",
                h1 { "{form.title}" }

                if let Some(description) = &form.description_markdown {
                    MarkdownDescription { markdown: description.clone() }
                }
            }

            if !missing_required.read().is_empty() {
                div { class: "form-submission__errors",
                    strong { "Please complete the required questions before submitting." }

                    ul {
                        for missing in missing_required.read().iter() {
                            li { key: "{missing.question_id}", "{missing.title}" }
                        }
                    }
                }
            }

            for section in form.sections.iter() {
                section { key: "{section.section_id}", class: "form-section",

                    h2 { "{section.number}. {section.title}" }

                    if let Some(description) = &section.description_markdown {
                        MarkdownDescription { markdown: description.clone() }
                    }

                    for question in section.questions.iter() {
                        QuestionInput {
                            key: "{question.question_id}",
                            question: question.clone(),
                            draft,
                            is_missing_required: missing_required
                                .read()
                                .iter()
                                .any(|missing| missing.question_id == question.question_id),
                        }
                    }
                }
            }

            SubmissionFields { draft }

            div { class: "form-actions",
                button {
                    r#type: "button",
                    disabled: is_submitting,
                    onclick: move |_| {
                        if is_submitting {
                            return;
                        }

                        let missing = missing_required_questions(&form, &draft.read().responses);

                        if !missing.is_empty() {
                            missing_required.set(missing);
                            return;
                        }

                        missing_required.set(Vec::new());

                        let completed_form = build_completed_form(&form, &draft.read());

                        if let Some(on_submit) = &on_submit {
                            on_submit.call(completed_form);
                        }
                    },
                    if is_submitting {
                        "Submitting..."
                    } else {
                        "Submit"
                    }
                }
            }
        }
    }
}
