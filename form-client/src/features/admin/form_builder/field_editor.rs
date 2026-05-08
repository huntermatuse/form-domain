use super::field_settings::QuestionKindProperties;
use super::helpers::{default_kind_for_value, question_kind_value};
use super::state::BuilderDraft;
use crate::features::admin::shared::optional_string;
use dioxus::prelude::*;

#[component]
pub(super) fn SectionProperties(draft: Signal<BuilderDraft>, section_index: usize) -> Element {
    let section = draft.read().sections[section_index].clone();

    rsx! {
        div { class: "builder-properties-inner",
            h2 { "Section" }
            label {
                "Title"
                input {
                    value: "{section.title}",
                    oninput: move |e| draft.write().sections[section_index].title = e.value(),
                }
            }
            label {
                "Description (Markdown)"
                textarea {
                    value: "{section.description_markdown.clone().unwrap_or_default()}",
                    oninput: move |e| {
                        draft.write().sections[section_index].description_markdown = optional_string(
                            e.value(),
                        );
                    },
                }
            }
            div { class: "builder-section-order",
                button {
                    class: "admin-icon-button",
                    r#type: "button",
                    onclick: move |_| draft.write().move_section(section_index, -1),
                    "Move up"
                }
                button {
                    class: "admin-icon-button",
                    r#type: "button",
                    onclick: move |_| draft.write().move_section(section_index, 1),
                    "Move down"
                }
                button {
                    class: "admin-icon-button",
                    r#type: "button",
                    onclick: move |_| draft.write().remove_section(section_index),
                    "Remove section"
                }
            }
        }
    }
}

#[component]
pub(super) fn QuestionProperties(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
) -> Element {
    let question = draft.read().sections[section_index].questions[question_index].clone();
    let kind_value = question_kind_value(&question.kind);

    rsx! {
        div { class: "builder-properties-inner",
            h2 { "Field properties" }

            label {
                "Label"
                input {
                    value: "{question.title}",
                    oninput: move |e| {
                        draft.write().sections[section_index].questions[question_index].title = e
                            .value();
                    },
                }
            }

            label { class: "admin-checkbox-row",
                input {
                    r#type: "checkbox",
                    checked: question.required,
                    onchange: move |_| {
                        let current = draft
                            .read()
                            .sections[section_index]
                            .questions[question_index]
                            .required;
                        draft.write().sections[section_index].questions[question_index].required = !current;
                    },
                }
                "Required"
            }

            label {
                "Field type"
                select {
                    value: "{kind_value}",
                    onchange: move |e| {
                        draft.write().sections[section_index].questions[question_index].kind =
                            default_kind_for_value(&e.value());
                    },
                    option { value: "text", "Short Text" }
                    option { value: "text_multiline", "Long Text" }
                    option { value: "email", "Email" }
                    option { value: "phone", "Phone" }
                    option { value: "date", "Date" }
                    option { value: "number", "Number" }
                    option { value: "choice", "Choice" }
                    option { value: "multi_choice", "Multi Choice" }
                    option { value: "dropdown", "Dropdown" }
                    option { value: "multi_dropdown", "Multi Dropdown" }
                    option { value: "ranked_list", "Ranked List" }
                    option { value: "validation", "Confirmation" }
                    option { value: "content_block", "Content Block" }
                }
            }

            QuestionKindProperties {
                draft,
                section_index,
                question_index,
                kind: question.kind,
            }
        }
    }
}
