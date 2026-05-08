use super::helpers::draft_to_form;
use super::preview::PreviewQuestion;
use super::state::{BuilderDraft, Selection};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

const PREVIEW_FORM_CSS: Asset = asset!("/assets/css/pages/builder.css", AssetOptions::css());

#[component]
pub(super) fn FormPreviewCanvas(
    draft: BuilderDraft,
    selected: Option<Selection>,
    on_select_section: EventHandler<usize>,
    on_select_question: EventHandler<(usize, usize)>,
    on_remove_question: EventHandler<(usize, usize)>,
    on_move_question: EventHandler<(usize, usize, isize)>,
) -> Element {
    let form = draft_to_form(&draft);

    rsx! {
        document::Link { rel: "stylesheet", href: PREVIEW_FORM_CSS }

        div { class: "canvas-preview-form",
            // Form header card
            header { class: "form-submission__header",
                h1 {
                    if form.title.is_empty() {
                        em { style: "opacity:0.4", "Untitled Form" }
                    } else {
                        "{form.title}"
                    }
                }
                if !form.meta.created_by.is_empty() {
                    p { style: "margin:0;color:#6b7280;font-size:.9rem",
                        "Prepared by {form.meta.created_by}"
                    }
                }
                if let Some(desc) = &form.description_markdown {
                    MarkdownDescription { markdown: desc.clone() }
                }
            }

            // Sections
            for (si, section) in draft.sections.iter().enumerate() {
                {
                    let section_id = section.section_id.clone();
                    let is_section_sel = selected == Some(Selection::Section(si));
                    rsx! {
                        div {
                            key: "{section_id}",
                            class: if is_section_sel { "canvas-section-wrap canvas-section-wrap--selected" } else { "canvas-section-wrap" },
                            onclick: move |e| {
                                e.stop_propagation();
                                on_select_section.call(si);
                            },
                            section { class: "form-section",
                                h2 {
                                    "{section.number}. "
                                    if section.title.is_empty() {
                                        em { style: "opacity:0.4", "Untitled section" }
                                    } else {
                                        "{section.title}"
                                    }
                                }
                                if let Some(desc) = &section.description_markdown {
                                    MarkdownDescription { markdown: desc.clone() }
                                }

                                for (qi, question) in section.questions.iter().enumerate() {
                                    {
                                        let question_id = question.question_id.clone();
                                        let is_q_sel = selected == Some(Selection::Question(si, qi));
                                        rsx! {
                                            div {
                                                key: "{question_id}",
                                                class: if is_q_sel { "canvas-question-wrap canvas-question-wrap--selected" } else { "canvas-question-wrap" },
                                                onclick: move |e| {
                                                    e.stop_propagation();
                                                    on_select_question.call((si, qi));
                                                },
                                                div { class: "canvas-question-controls",
                                                    button {
                                                        class: "canvas-ctrl-btn",
                                                        r#type: "button",
                                                        title: "Move up",
                                                        onclick: move |e| {
                                                            e.stop_propagation();
                                                            on_move_question.call((si, qi, -1));
                                                        },
                                                        "↑"
                                                    }
                                                    button {
                                                        class: "canvas-ctrl-btn",
                                                        r#type: "button",
                                                        title: "Move down",
                                                        onclick: move |e| {
                                                            e.stop_propagation();
                                                            on_move_question.call((si, qi, 1));
                                                        },
                                                        "↓"
                                                    }
                                                    button {
                                                        class: "canvas-ctrl-btn canvas-ctrl-btn--remove",
                                                        r#type: "button",
                                                        title: "Remove",
                                                        onclick: move |e| {
                                                            e.stop_propagation();
                                                            on_remove_question.call((si, qi));
                                                        },
                                                        "✕"
                                                    }
                                                }
                                                PreviewQuestion { question: question.clone() }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Dimmed submission fields so the form looks complete
            section { class: "form-submission-details",
                h2 { "Submission details" }
                div { class: "form-submission-details__grid",
                    div { class: "field",
                        label { "Company" }
                        input {
                            r#type: "text",
                            disabled: true,
                            placeholder: "Company name",
                        }
                    }
                    div { class: "field",
                        label { "Signer name" }
                        input {
                            r#type: "text",
                            disabled: true,
                            placeholder: "Full name",
                        }
                    }
                    div { class: "field",
                        label { "Signer title" }
                        input {
                            r#type: "text",
                            disabled: true,
                            placeholder: "Title",
                        }
                    }
                    div { class: "field",
                        label { "Date" }
                        input { r#type: "date", disabled: true }
                    }
                }
            }
        }
    }
}
