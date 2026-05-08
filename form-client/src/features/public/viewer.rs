use crate::api::http::ApiError;
use crate::api::public::fetch_completed_form;
use crate::forms::model::CompletedForm;
use crate::forms::render::print::{print_current_document, PrintCompletedFormMount};
use crate::forms::render::viewer::CompletedFormViewer;
use crate::ui::not_found::token_unavailable_page;
use dioxus::prelude::*;

const VIEWER_CSS: Asset = asset!("/assets/css/pages/public-viewer.css", AssetOptions::css());


#[component]
pub fn PublicCompletedFormViewerPage(token: String) -> Element {
    let token_for_fetch = token.clone();
    let completed_form_resource = use_resource(move || {
        let token = token_for_fetch.clone();
        async move { fetch_completed_form(&token).await }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: VIEWER_CSS }

        div { class: "wrap",
            match &*completed_form_resource.read() {
                Some(Ok(completed_form)) => rsx! {
                    div { class: "public-viewer-screen",
                        PublicViewerToolbar { completed_form: completed_form.clone() }
                        CompletedFormViewer { completed_form: completed_form.clone() }
                    }
                    PrintCompletedFormMount { completed_form: completed_form.clone() }
                },
                Some(Err(error)) if is_public_token_error(error) => rsx! {
                    {token_unavailable_page()}
                },
                Some(Err(error)) => rsx! {
                    section { class: "completed-form-viewer__header",
                        h1 { "Unable to load completed form" }
                        p { "{error}" }
                    }
                },
                None => rsx! {
                    section { class: "completed-form-viewer__header",
                        h1 { "Loading completed form..." }
                    }
                },
            }
        }
    }
}

fn is_public_token_error(error: &ApiError) -> bool {
    matches!(
        error,
        ApiError::NotFound | ApiError::Gone | ApiError::BadRequest(_)
    )
}

#[component]
fn PublicViewerToolbar(completed_form: CompletedForm) -> Element {
    let title = completed_form.form.title;
    let submitted_at = completed_form.submission.submitted_at;
    let submitted_date = submitted_at.get(..10).unwrap_or(&submitted_at).to_string();

    rsx! {
        div { class: "public-viewer-toolbar",
            div { class: "public-viewer-toolbar__title",
                strong { "{title}" }
                span { "Submitted {submitted_date}" }
            }

            button {
                r#type: "button",
                onclick: move |_| {
                    print_current_document();
                },
                "Download / Print PDF"
            }
        }
    }
}
