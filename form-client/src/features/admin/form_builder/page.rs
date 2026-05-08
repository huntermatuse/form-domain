use super::field_editor::{QuestionProperties, SectionProperties};
use super::field_list::FormPreviewCanvas;
use super::new_form_modal::NewFormModal;
use super::save_actions::SaveActions;
use super::state::{BuilderDraft, Selection, BUILDER_PREFILL};
use super::toolbar::FieldPalette;
use crate::features::admin::shared::AdminFrame;
use crate::Route;
use dioxus::prelude::*;

const BUILDER_CSS: Asset = asset!("/assets/css/pages/builder.css", AssetOptions::css());

#[component]
pub fn AdminFormBuilderPage() -> Element {
    let navigator = use_navigator();
    let prefill = BUILDER_PREFILL.write().take();
    let has_prefill = prefill.is_some();
    let mut draft = use_signal(move || prefill.unwrap_or_default());
    let mut show_modal = use_signal(move || !has_prefill);
    let mut selected = use_signal(|| None::<Selection>);
    let error = use_signal(|| None::<String>);
    let is_saving = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: BUILDER_CSS }
        AdminFrame { title: "Form builder".to_string(),
            if *show_modal.read() {
                NewFormModal {
                    on_confirm: move |confirmed: BuilderDraft| {
                        draft.set(confirmed);
                        show_modal.set(false);
                    },
                    on_cancel: {
                        let navigator = navigator.clone();
                        move |_| {
                            navigator.push(Route::AdminFormListPage {});
                        }
                    },
                }
            }

            if !*show_modal.read() {
                div { class: "builder-shell",
                    FieldPalette { draft, selected }

                    main { class: "builder-canvas",
                        if let Some(message) = error.read().as_ref() {
                            p { class: "admin-error", "{message}" }
                        }

                        div { class: "builder-canvas-scroll",
                            FormPreviewCanvas {
                                draft: draft.read().clone(),
                                selected: selected.read().clone(),
                                on_select_section: move |i| selected.set(Some(Selection::Section(i))),
                                on_select_question: move |(si, qi)| selected.set(Some(Selection::Question(si, qi))),
                                on_remove_question: move |(si, qi)| {
                                    draft.write().remove_question(si, qi);
                                    selected.set(None);
                                },
                                on_move_question: move |(si, qi, dir)| draft.write().move_question(si, qi, dir),
                            }
                        }

                        SaveActions { draft, error, is_saving }
                    }

                    aside { class: "builder-properties",
                        match *selected.read() {
                            None => rsx! {
                                p { class: "admin-muted", "Select a field to edit its properties" }
                            },
                            Some(Selection::Section(section_index)) => rsx! {
                                SectionProperties { draft, section_index }
                            },
                            Some(Selection::Question(section_index, question_index)) => rsx! {
                                QuestionProperties { draft, section_index, question_index }
                            },
                        }
                    }
                }
            }
        }
    }
}
