use crate::api;
use crate::features::admin::shared::{AdminError, AdminFrame};
use crate::forms::model::{CompletedForm, Response, ValidationStatus};
use crate::forms::render::print::{print_current_document, PrintCompletedFormMount};
use crate::forms::render::viewer::CompletedFormViewer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminSubmissionDetailPage(
    form_id: String,
    version: i32,
    completed_form_id: String,
) -> Element {
    let navigator = use_navigator();
    let submission = use_resource({
        let completed_form_id = completed_form_id.clone();
        move || {
            let completed_form_id = completed_form_id.clone();
            async move { api::admin::fetch_submission(&completed_form_id).await }
        }
    });

    rsx! {
        AdminFrame { title: "Submission detail".to_string(),
            div { class: "admin-page-actions",
                button {
                    class: "admin-secondary-button",
                    r#type: "button",
                    onclick: move |_| {
                        navigator
                            .push(Route::AdminSubmissionListPage {
                                form_id: form_id.clone(),
                                version,
                            });
                    },
                    "Back to submissions"
                }

                if let Some(Ok(_)) = submission.read().as_ref() {
                    button {
                        class: "admin-secondary-button",
                        r#type: "button",
                        onclick: move |_| {
                            print_current_document();
                        },
                        "Download / Print PDF"
                    }
                }
            }
            match submission.read().as_ref() {
                None => rsx! {
                    p { class: "admin-muted", "Loading submission..." }
                },
                Some(Err(err)) => rsx! {
                    AdminError { err: err.clone() }
                },
                Some(Ok(completed_form)) => rsx! {
                    NegativeConfirmationActions { completed_form: completed_form.clone() }
                    CompletedFormViewer { completed_form: completed_form.clone() }
                    PrintCompletedFormMount { completed_form: completed_form.clone() }
                },
            }
        }
    }
}

#[component]
fn NegativeConfirmationActions(completed_form: CompletedForm) -> Element {
    let actions: Vec<_> = completed_form
        .responses
        .iter()
        .filter_map(|response| match &response.response {
            Response::Validation {
                status: ValidationStatus::NotCorrect,
                comment,
            } => Some((response.question_id.clone(), comment.clone())),
            _ => None,
        })
        .collect();

    if actions.is_empty() {
        return rsx! {};
    }

    rsx! {
        section { class: "admin-panel admin-action-items",
            h2 { "Action items" }
            for (question_id, comment) in actions.iter() {
                article { class: "admin-action-item", key: "{question_id}",
                    strong { "{question_title(&completed_form.form, question_id)}" }
                    p { "{comment.clone().unwrap_or_else(|| \"No feedback provided.\".to_string())}" }
                }
            }
        }
    }
}

fn question_title(form: &crate::forms::model::Form, question_id: &str) -> String {
    form.sections
        .iter()
        .flat_map(|section| section.questions.iter())
        .find(|question| question.question_id == question_id)
        .map(|question| question.title.clone())
        .unwrap_or_else(|| question_id.to_string())
}
