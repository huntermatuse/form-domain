use crate::api;
use crate::features::admin::shared::{AdminError, AdminFrame, StatusPill};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminFormListPage() -> Element {
    let forms = use_resource(move || async move { api::admin::fetch_forms().await });
    let navigator = use_navigator();
    let new_form_navigator = navigator.clone();

    rsx! {
        AdminFrame { title: "Form management".to_string(),
            div { class: "admin-page-actions",
                button {
                    class: "admin-primary-button",
                    r#type: "button",
                    onclick: move |_| {
                        new_form_navigator.push(Route::AdminFormBuilderPage {});
                    },
                    "New form"
                }
            }

            match forms.read().as_ref() {
                None => rsx! {
                    p { class: "admin-muted", "Loading forms..." }
                },
                Some(Err(err)) => rsx! {
                    AdminError { err: err.clone() }
                },
                Some(Ok(forms)) => rsx! {
                    table { class: "admin-table",
                        thead {
                            tr {
                                th { "Name" }
                                th { "Status" }
                                th { "Created" }
                                th { "Prepared by" }
                            }
                        }
                        tbody {
                            for form in forms.iter() {
                                tr {
                                    key: "{form.form_id}-{form.version}",
                                    onclick: {
                                        let form_id = form.form_id.clone();
                                        let version = form.version;
                                        let navigator = navigator.clone();
                                        move |_| {
                                            navigator
                                                .push(Route::AdminFormDetailPage {
                                                    form_id: form_id.clone(),
                                                    version,
                                                });
                                        }
                                    },
                                    td { "{form.title}" }
                                    td {
                                        StatusPill { active: form.active }
                                    }
                                    td { "{form.created_at}" }
                                    td { "{form.created_by}" }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
