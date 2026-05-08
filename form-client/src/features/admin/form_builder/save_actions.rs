use super::state::BuilderDraft;
use super::validation::validate_builder_draft;
use crate::api;
use crate::features::admin::shared::optional_string;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub(super) fn SaveActions(
    draft: Signal<BuilderDraft>,
    mut error: Signal<Option<String>>,
    mut is_saving: Signal<bool>,
) -> Element {
    let navigator = use_navigator();

    rsx! {
        div { class: "builder-save-row",
            button {
                class: "admin-primary-button",
                r#type: "button",
                disabled: *is_saving.read(),
                onclick: move |_| {
                    let current = draft.read().clone();
                    if let Err(message) = validate_builder_draft(&current) {
                        error.set(Some(message));
                        return;
                    }
                    error.set(None);
                    is_saving.set(true);
                    let navigator = navigator.clone();
                    spawn(async move {
                        let req = api::admin::CreateFormRequest {
                            title: current.title,
                            description_markdown: optional_string(current.description_markdown),
                            created_by: current.created_by,
                            sections: current.sections,
                        };
                        match api::admin::create_form(&req).await {
                            Ok(detail) => {
                                navigator
                                    .push(Route::AdminFormDetailPage {
                                        form_id: detail.form.form_id,
                                        version: detail.form.version as i32,
                                    });
                            }
                            Err(err) => error.set(Some(err.to_string())),
                        }
                        is_saving.set(false);
                    });
                },
                if *is_saving.read() {
                    "Saving..."
                } else {
                    "Save form"
                }
            }
        }
    }
}
