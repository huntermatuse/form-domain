use crate::api;
use crate::features::admin::shared::{AdminError, AdminFrame};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminSubmissionListPage(form_id: String, version: i32) -> Element {
    let submissions = use_resource({
        let form_id = form_id.clone();
        move || {
            let form_id = form_id.clone();
            async move { api::admin::fetch_submissions(&form_id, version).await }
        }
    });
    let navigator = use_navigator();

    rsx! {
        AdminFrame { title: "Submissions".to_string(),
            match submissions.read().as_ref() {
                None => rsx! {
                    p { class: "admin-muted", "Loading submissions..." }
                },
                Some(Err(err)) => rsx! {
                    AdminError { err: err.clone() }
                },
                Some(Ok(items)) => rsx! {
                    table { class: "admin-table",
                        thead {
                            tr {
                                th { "Submitted" }
                                th { "Company" }
                                th { "Signer" }
                                th { "Flag" }
                            }
                        }
                        tbody {
                            for item in items.iter() {
                                tr {
                                    key: "{item.completed_form_id}",
                                    onclick: {
                                        let completed_form_id = item.completed_form_id.clone();
                                        let form_id = form_id.clone();
                                        let navigator = navigator.clone();
                                        move |_| {
                                            navigator
                                                .push(Route::AdminSubmissionDetailPage {
                                                    form_id: form_id.clone(),
                                                    version,
                                                    completed_form_id: completed_form_id.clone(),
                                                });
                                        }
                                    },
                                    td { "{item.submitted_at}" }
                                    td { "{item.company_name}" }
                                    td { "{item.signer_name} ({item.signer_title})" }
                                    td {
                                        if item.has_negative_confirmation {
                                            span { class: "admin-flag admin-flag--negative", "Action needed" }
                                        } else {
                                            span { class: "admin-flag", "Clear" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
