use super::state::{BuilderDraft, Selection};
use dioxus::prelude::*;

const FIELD_TYPES: &[(&str, &str)] = &[
    ("Short Text", "text"),
    ("Long Text", "text_multiline"),
    ("Choice", "choice"),
    ("Multi Choice", "multi_choice"),
    ("Confirmation", "validation"),
    ("Email", "email"),
    ("Phone", "phone"),
    ("Date", "date"),
    ("Number", "number"),
    ("Dropdown", "dropdown"),
    ("Multi Dropdown", "multi_dropdown"),
    ("Ranked List", "ranked_list"),
    ("Content Block", "content_block"),
];

#[component]
pub(super) fn FieldPalette(
    mut draft: Signal<BuilderDraft>,
    mut selected: Signal<Option<Selection>>,
) -> Element {
    rsx! {
        aside { class: "builder-palette",
            h2 { "Add field" }
            for (label, kind) in FIELD_TYPES.iter().copied() {
                FieldTypeButton {
                    key: "{kind}",
                    label,
                    onclick: move |_| {
                        let idx = draft.write().push_question_to_last_section(kind);
                        selected.set(Some(idx));
                    },
                }
            }
            FieldTypeButton {
                label: "Section",
                onclick: move |_| {
                    draft.write().add_section();
                    selected.set(None);
                },
            }
        }
    }
}

#[component]
fn FieldTypeButton(label: &'static str, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "builder-palette-item",
            r#type: "button",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}
