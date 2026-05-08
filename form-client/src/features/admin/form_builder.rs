use crate::api;
use crate::features::admin::shared::{optional_string, AdminFrame};
use crate::forms::model::{Form, FormMeta, Question, QuestionKind, QuestionOption, Section};
use crate::forms::render::markdown::MarkdownDescription;
use crate::Route;
use dioxus::prelude::*;

pub static BUILDER_PREFILL: GlobalSignal<Option<BuilderDraft>> = Signal::global(|| None);

const BUILDER_CSS: &str = r#"
.builder-shell {
    display: grid;
    grid-template-columns: 200px minmax(0, 1fr) 280px;
    gap: 0;
    height: calc(100dvh - 120px);
    overflow: hidden;
    border: 1px solid #2d3340;
    border-radius: 8px;
}

/* ── Left palette ── */
.builder-palette {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 16px 12px;
    border-right: 1px solid #2d3340;
    background: #10131a;
    overflow-y: auto;
}

.builder-palette h2 {
    margin: 0 0 12px 0;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: #8a95a8;
}

.builder-palette-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 8px 10px;
    border: 1px solid #2a3040;
    border-radius: 6px;
    background: transparent;
    color: #d0d8e8;
    font: inherit;
    font-size: 13px;
    text-align: left;
    cursor: pointer;
    transition: background 0.12s, border-color 0.12s;
}

.builder-palette-item:hover {
    background: #1a2030;
    border-color: #3cc4dc;
}

/* ── Center canvas ── */
.builder-canvas {
    display: flex;
    flex-direction: column;
    gap: 0;
    padding: 20px 24px;
    overflow-y: auto;
    background: #0f1116;
}

.builder-form-meta {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 20px;
}

.builder-form-title {
    font-size: 20px !important;
    font-weight: 700 !important;
    border-color: transparent !important;
    background: transparent !important;
    padding: 4px 0 !important;
}

.builder-form-title:focus {
    border-color: #3cc4dc !important;
    background: #0f131b !important;
    padding: 4px 10px !important;
}

.builder-form-author {
    font-size: 13px !important;
    color: #8a95a8 !important;
    border-color: transparent !important;
    background: transparent !important;
    padding: 4px 0 !important;
}

.builder-form-author:focus {
    border-color: #3cc4dc !important;
    background: #0f131b !important;
    padding: 4px 10px !important;
}

/* ── Canvas scroll + preview ── */
.builder-canvas-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
}

/* Selection ring on preview question cards */
.canvas-question-wrap {
    position: relative;
    border-radius: 8px;
    transition: box-shadow 0.12s;
    cursor: pointer;
}

.canvas-question-wrap:hover {
    box-shadow: 0 0 0 2px #93c5fd;
}

.canvas-question-wrap--selected {
    box-shadow: 0 0 0 2px #3cc4dc !important;
}

/* Section selection ring */
.canvas-section-wrap {
    border-radius: 16px;
    transition: box-shadow 0.12s;
    cursor: pointer;
    margin-bottom: 20px;
}

.canvas-section-wrap:hover {
    box-shadow: 0 0 0 2px #93c5fd;
}

.canvas-section-wrap--selected {
    box-shadow: 0 0 0 2px #3cc4dc !important;
}

/* Controls row shown on hover / selection */
.canvas-question-controls {
    display: none;
    align-items: center;
    gap: 4px;
    position: absolute;
    top: 6px;
    right: 8px;
    z-index: 10;
    background: rgba(15,17,22,0.82);
    border-radius: 6px;
    padding: 3px 4px;
    backdrop-filter: blur(4px);
}

.canvas-question-wrap:hover .canvas-question-controls,
.canvas-question-wrap--selected .canvas-question-controls {
    display: flex;
}

.canvas-ctrl-btn {
    background: #1a2030;
    border: 1px solid #2d3340;
    border-radius: 4px;
    color: #b8c0cc;
    font-size: 11px;
    cursor: pointer;
    padding: 2px 7px;
    line-height: 1.5;
    transition: background 0.1s, color 0.1s;
}

.canvas-ctrl-btn:hover {
    background: #253050;
    color: #f7f8fb;
}

.canvas-ctrl-btn--remove:hover {
    background: #3b1f24;
    color: #f08a8a;
    border-color: #81404d;
}

/* Dim the submit button and submission fields — not interactive in preview */
.canvas-preview-form .form-actions,
.canvas-preview-form .form-submission-details {
    opacity: 0.45;
    pointer-events: none;
}

.builder-save-row {
    display: flex;
    justify-content: flex-end;
    padding: 16px 0 4px;
    margin-top: auto;
}

/* ── Right properties ── */
.builder-properties {
    display: flex;
    flex-direction: column;
    border-left: 1px solid #2d3340;
    background: #10131a;
    overflow-y: auto;
}

.builder-properties-inner {
    display: flex;
    flex-direction: column;
    gap: 14px;
    padding: 16px;
}

.builder-properties-inner h2 {
    margin: 0 0 4px;
    font-size: 13px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: #8a95a8;
}

.builder-properties-inner h3 {
    margin: 4px 0 0;
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #6a7888;
}

.builder-properties-inner label {
    display: flex;
    flex-direction: column;
    gap: 5px;
    font-size: 12px;
    font-weight: 600;
    color: #a0aab8;
}

.builder-properties-inner label.admin-checkbox-row {
    flex-direction: row;
    align-items: center;
    gap: 8px;
}

.builder-section-order {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding-top: 8px;
    border-top: 1px solid #2d3340;
}

/* Overrides for the narrow properties panel */
.builder-properties-inner .admin-two-col {
    grid-template-columns: 1fr 1fr;
    gap: 8px;
}

.builder-properties-inner textarea {
    min-height: 80px;
}

/* Option rows: stack label+description vertically, remove button on the right */
.builder-properties-inner .admin-option-row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    grid-template-rows: auto auto;
    gap: 4px 6px;
    align-items: start;
}

.builder-properties-inner .admin-option-row input:first-child {
    grid-column: 1;
    grid-row: 1;
}

.builder-properties-inner .admin-option-row input:nth-child(2) {
    grid-column: 1;
    grid-row: 2;
}

.builder-properties-inner .admin-option-row button {
    grid-column: 2;
    grid-row: 1 / 3;
    align-self: center;
    font-size: 11px;
    padding: 5px 8px;
    min-height: 28px;
    white-space: nowrap;
}

.builder-properties-inner .admin-option-row input {
    padding: 6px 8px;
    font-size: 12px;
}

.builder-properties-inner .admin-icon-button {
    min-height: 32px;
    padding: 5px 10px;
    font-size: 12px;
}
"#;

// ── Public page ──────────────────────────────────────────────────────────────

#[component]
pub fn AdminFormBuilderPage() -> Element {
    let navigator = use_navigator();
    let prefill = BUILDER_PREFILL.write().take();
    let has_prefill = prefill.is_some();
    let mut draft = use_signal(move || prefill.unwrap_or_default());
    let mut show_modal = use_signal(move || !has_prefill);
    let mut selected = use_signal(|| None::<Selection>);
    let mut error = use_signal(|| None::<String>);
    let mut is_saving = use_signal(|| false);

    rsx! {
        document::Style { {BUILDER_CSS} }
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

                    // ── Left: field palette ──────────────────────────────────────
                    aside { class: "builder-palette",
                        h2 { "Add field" }
                        FieldTypeButton {
                            label: "Short Text",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("text");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Long Text",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("text_multiline");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Choice",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("choice");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Multi Choice",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("multi_choice");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Confirmation",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("validation");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Email",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("email");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Phone",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("phone");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Date",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("date");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Number",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("number");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Dropdown",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("dropdown");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Multi Dropdown",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("multi_dropdown");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Ranked List",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("ranked_list");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Content Block",
                            onclick: move |_| {
                                let idx = draft.write().push_question_to_last_section("content_block");
                                selected.set(Some(idx));
                            },
                        }
                        FieldTypeButton {
                            label: "Section",
                            onclick: move |_| {
                                draft.write().add_section();
                                selected.set(None);
                            },
                        }
                    }

                    // ── Center: form preview canvas ──────────────────────────────
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

                    // ── Right: properties panel ──────────────────────────────────
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

// ── New form modal ────────────────────────────────────────────────────────────

#[component]
fn NewFormModal(
    on_confirm: EventHandler<BuilderDraft>,
    on_cancel: EventHandler<MouseEvent>,
) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut created_by = use_signal(String::new);
    let mut title_error = use_signal(|| None::<String>);
    let mut checking = use_signal(|| false);
    let mut title_available = use_signal(|| true);

    rsx! {
        div { class: "modal-backdrop",
            div { class: "modal",
                h2 { class: "modal-title", "New form" }

                label { class: "modal-label",
                    "Form name"
                    div { class: "modal-field-row",
                        input {
                            class: "modal-input",
                            r#type: "text",
                            placeholder: "e.g. Vendor Onboarding 2025",
                            value: "{title.read()}",
                            oninput: move |e| {
                                let v = e.value();
                                title.set(v.clone());
                                title_error.set(None);
                                title_available.set(true);
                                if !v.trim().is_empty() {
                                    checking.set(true);
                                    spawn(async move {
                                        match api::admin::check_form_title_available(v.trim()).await {
                                            Ok(available) => {
                                                title_available.set(available);
                                                if !available {
                                                    title_error
                                                        .set(Some("That name is already taken.".to_string()));
                                                }
                                            }
                                            Err(_) => {}
                                        }
                                        checking.set(false);
                                    });
                                }
                            },
                        }
                        if *checking.read() {
                            span { class: "modal-field-hint", "Checking..." }
                        } else if *title_available.read() && !title.read().trim().is_empty() {
                            span { class: "modal-field-hint modal-field-hint--ok", "Available" }
                        }
                    }
                    if let Some(err) = title_error.read().as_ref() {
                        span { class: "modal-field-error", "{err}" }
                    }
                }

                label { class: "modal-label",
                    "Version"
                    input {
                        class: "modal-input modal-input--readonly",
                        r#type: "text",
                        value: "1",
                        readonly: true,
                    }
                }

                label { class: "modal-label",
                    "Description (optional)"
                    textarea {
                        class: "modal-textarea",
                        placeholder: "Short description shown at the top of the form",
                        value: "{description.read()}",
                        oninput: move |e| description.set(e.value()),
                    }
                }

                label { class: "modal-label",
                    "Prepared by"
                    input {
                        class: "modal-input",
                        r#type: "text",
                        placeholder: "Team or person creating this form",
                        value: "{created_by.read()}",
                        oninput: move |e| created_by.set(e.value()),
                    }
                }

                div { class: "modal-actions",
                    button {
                        class: "admin-secondary-button",
                        r#type: "button",
                        onclick: move |e| on_cancel.call(e),
                        "Cancel"
                    }
                    button {
                        class: "admin-primary-button",
                        r#type: "button",
                        disabled: !*title_available.read() || title.read().trim().is_empty()
                            || created_by.read().trim().is_empty(),
                        onclick: move |_| {
                            let t = title.read().trim().to_string();
                            let cb = created_by.read().trim().to_string();
                            if t.is_empty() {
                                title_error.set(Some("Name is required.".to_string()));
                                return;
                            }
                            if cb.is_empty() {
                                return;
                            }
                            on_confirm
                                .call(BuilderDraft {
                                    title: t,
                                    description_markdown: description.read().clone(),
                                    created_by: cb,
                                    sections: vec![default_section(0)],
                                });
                        },
                        "Create form"
                    }
                }
            }
        }
    }
}

// ── Center panel: live form preview ──────────────────────────────────────────

const PREVIEW_FORM_CSS: &str = r#"
.canvas-preview-form {
  font-family: inherit; color: #f7f8fb; line-height: 1.5;
}
.canvas-preview-form *, .canvas-preview-form *::before, .canvas-preview-form *::after { box-sizing: border-box; }
.canvas-preview-form h1 { margin: 0 0 8px; font-size: 1.6rem; font-weight: 700; }
.canvas-preview-form h2 { margin: 0 0 12px; font-size: 1.05rem; font-weight: 700; color: #c8d0e0; }
.canvas-preview-form h3 { margin: 14px 0 6px; font-size: .95rem; font-weight: 600; color: #f7f8fb; }
.canvas-preview-form p, .canvas-preview-form li { font-size: .93rem; color: #b8c0cc; }
.canvas-preview-form .form-submission__header,
.canvas-preview-form .form-section,
.canvas-preview-form .form-submission-details {
  background: #151922; border: 1px solid #2d3340;
  border-radius: 10px; padding: 20px; margin-bottom: 16px;
}
.canvas-preview-form .form-question { padding: 14px 0; border-top: 1px solid #252d3d; }
.canvas-preview-form .form-question:first-of-type { border-top: none; padding-top: 0; }
.canvas-preview-form .form-question__heading { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }
.canvas-preview-form .form-question__required {
  display: inline-block; font-size: .75rem; font-weight: 700;
  padding: 3px 8px; border-radius: 999px; background: #1e2840; color: #7abbd8;
}
.canvas-preview-form .form-question__prompt { margin-top: 10px; color: #c8d0e0; }
.canvas-preview-form .radio-row, .canvas-preview-form .choice-list {
  display: flex; flex-wrap: wrap; gap: 16px; margin: 10px 0 12px;
}
.canvas-preview-form .radio-row label, .canvas-preview-form .choice-list label {
  display: flex; align-items: center; gap: 7px; font-size: .9rem; color: #c8d0e0; cursor: default;
}
.canvas-preview-form input[type='radio'],
.canvas-preview-form input[type='checkbox'] {
  width: auto; flex-shrink: 0;
}
.canvas-preview-form input[type='text'],
.canvas-preview-form input[type='date'],
.canvas-preview-form textarea {
  width: 100%; border: 1px solid #3a4352; border-radius: 7px;
  padding: 9px 11px; font: inherit; background: #0f131b; color: #f7f8fb; pointer-events: none;
}
.canvas-preview-form textarea { min-height: 88px; resize: none; }
.canvas-preview-form .form-submission-details__grid {
  display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px;
}
.canvas-preview-form .field label { display: block; font-weight: 600; margin-bottom: 5px; font-size: .85rem; color: #a0aab8; }
.canvas-preview-form .form-submission-details { opacity: 0.5; pointer-events: none; }
.canvas-preview-form .markdown blockquote {
  border-left: 3px solid #3cc4dc; background: #10242c;
  padding: 10px 14px; border-radius: 6px; margin: 12px 0; color: #dff8ff;
}
.canvas-preview-form select {
  width: 100%; border: 1px solid #3a4352; border-radius: 7px;
  padding: 9px 11px; font: inherit; background: #0f131b; color: #f7f8fb;
  pointer-events: none;
}
.canvas-preview-form select[multiple] { min-height: 88px; }
.ranked-list-preview { display: flex; flex-direction: column; gap: 6px; margin-top: 8px; }
.ranked-list-preview__item {
  display: flex; align-items: center; gap: 10px;
  padding: 8px 12px; background: #0f131b; border: 1px solid #2d3340;
  border-radius: 6px; color: #c8d0e0; font-size: .9rem;
}
.ranked-list-preview__rank {
  width: 22px; height: 22px; border-radius: 50%; background: #1e2840;
  color: #7abbd8; font-size: .75rem; font-weight: 700;
  display: flex; align-items: center; justify-content: center; flex-shrink: 0;
}
"#;

#[component]
fn FormPreviewCanvas(
    draft: BuilderDraft,
    selected: Option<Selection>,
    on_select_section: EventHandler<usize>,
    on_select_question: EventHandler<(usize, usize)>,
    on_remove_question: EventHandler<(usize, usize)>,
    on_move_question: EventHandler<(usize, usize, isize)>,
) -> Element {
    let form = draft_to_form(&draft);

    rsx! {
        document::Style { {PREVIEW_FORM_CSS} }

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

#[component]
fn PreviewQuestion(question: Question) -> Element {
    rsx! {
        div { class: "form-question",
            div { class: "form-question__heading",
                h3 {
                    "{question.number}. "
                    if question.title.is_empty() {
                        em { style: "opacity:0.4", "Untitled question" }
                    } else {
                        "{question.title}"
                    }
                }
                if question.required {
                    span { class: "form-question__required", "Required" }
                }
            }
            PreviewQuestionBody { kind: question.kind.clone() }
        }
    }
}

#[component]
fn PreviewQuestionBody(kind: QuestionKind) -> Element {
    rsx! {
        match kind {
            QuestionKind::Text { description_markdown, placeholder, multiline, .. } => {
                let ph = placeholder.unwrap_or_default();
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    if multiline {
                        textarea { disabled: true, placeholder: "{ph}" }
                    } else {
                        input { r#type: "text", disabled: true, placeholder: "{ph}" }
                    }
                }
            }
            QuestionKind::Choice { description_markdown, options, .. } => rsx! {
                if let Some(desc) = description_markdown {
                    MarkdownDescription { markdown: desc }
                }
                div { class: "choice-list",
                    for option in options.iter() {
                        label { key: "{option.question_option_id}",
                            input { r#type: "radio", disabled: true }
                            "{option.label}"
                        }
                    }
                }
            },
            QuestionKind::MultiChoice { description_markdown, options, .. } => {
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    div { class: "choice-list",
                        for option in options.iter() {
                            label { key: "{option.question_option_id}",
                                input { r#type: "checkbox", disabled: true }
                                "{option.label}"
                            }
                        }
                    }
                }
            }
            QuestionKind::Validation { description_markdown, confirm_prompt, .. } => {
                rsx! {
                    MarkdownDescription { markdown: description_markdown }
                    div { class: "form-question__prompt",
                        strong { "Please confirm: " }
                        "{confirm_prompt}"
                    }
                    div { class: "radio-row",
                        label {
                            input { r#type: "radio", disabled: true }
                            "Confirmed"
                        }
                        label {
                            input { r#type: "radio", disabled: true }
                            "Not correct"
                        }
                    }
                }
            }
            QuestionKind::Email { description_markdown, placeholder } => {
                let ph = placeholder.unwrap_or_else(|| "email@example.com".to_string());
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    input { r#type: "text", disabled: true, placeholder: "{ph}" }
                }
            }
            QuestionKind::Phone { description_markdown, placeholder } => {
                let ph = placeholder.unwrap_or_else(|| "+1 (555) 000-0000".to_string());
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    input { r#type: "text", disabled: true, placeholder: "{ph}" }
                }
            }
            QuestionKind::Date { description_markdown } => rsx! {
                if let Some(desc) = description_markdown {
                    MarkdownDescription { markdown: desc }
                }
                input { r#type: "date", disabled: true }
            },
            QuestionKind::Number { description_markdown, placeholder, .. } => {
                let ph = placeholder.unwrap_or_default();
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    input { r#type: "text", disabled: true, placeholder: "{ph}" }
                }
            }
            QuestionKind::Dropdown { description_markdown, options, .. } => rsx! {
                if let Some(desc) = description_markdown {
                    MarkdownDescription { markdown: desc }
                }
                select { disabled: true,
                    option { value: "", "— Select —" }
                    for option in options.iter() {
                        option {
                            key: "{option.question_option_id}",
                            value: "{option.question_option_id}",
                            "{option.label}"
                        }
                    }
                }
            },
            QuestionKind::MultiDropdown { description_markdown, options, .. } => {
                rsx! {
                    if let Some(desc) = description_markdown {
                        MarkdownDescription { markdown: desc }
                    }
                    select { disabled: true, multiple: true,
                        for option in options.iter() {
                            option {
                                key: "{option.question_option_id}",
                                value: "{option.question_option_id}",
                                "{option.label}"
                            }
                        }
                    }
                }
            }
            QuestionKind::RankedList {
                description_markdown,
                options,
                randomize_initial_order,
            } => rsx! {
                if let Some(desc) = description_markdown {
                    MarkdownDescription { markdown: desc }
                }
                if randomize_initial_order {
                    p { class: "form-question__prompt", "Initial order randomized for respondents." }
                }
                div { class: "ranked-list-preview",
                    for (i, option) in options.iter().enumerate() {
                        div {
                            key: "{option.question_option_id}",
                            class: "ranked-list-preview__item",
                            span { class: "ranked-list-preview__rank", "{i + 1}" }
                            span { "{option.label}" }
                        }
                    }
                }
            },
            QuestionKind::ContentBlock { content_markdown } => rsx! {
                MarkdownDescription { markdown: content_markdown }
            },
        }
    }
}

fn draft_to_form(draft: &BuilderDraft) -> Form {
    Form {
        form_id: "preview".to_string(),
        version: 1,
        title: draft.title.clone(),
        description_markdown: {
            let d = draft.description_markdown.trim().to_string();
            if d.is_empty() {
                None
            } else {
                Some(d)
            }
        },
        meta: FormMeta {
            created_at: String::new(),
            created_by: draft.created_by.clone(),
            updated_at: None,
            updated_by: None,
        },
        sections: draft.sections.clone(),
    }
}

// ── Left panel: palette button ────────────────────────────────────────────────

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

// ── Right panel: section properties ──────────────────────────────────────────

#[component]
fn SectionProperties(draft: Signal<BuilderDraft>, section_index: usize) -> Element {
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

// ── Right panel: question properties ─────────────────────────────────────────

#[component]
fn QuestionProperties(
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
                        draft.write().sections[section_index].questions[question_index].title = e.value();
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

// ── Kind-specific property fields ────────────────────────────────────────────

#[component]
fn QuestionKindProperties(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    kind: QuestionKind,
) -> Element {
    match kind {
        QuestionKind::Validation {
            description_markdown,
            confirm_prompt,
            summary_item,
        } => rsx! {
            label {
                "Description (Markdown)"
                textarea {
                    value: "{description_markdown}",
                    oninput: move |e| {
                        if let QuestionKind::Validation { description_markdown, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *description_markdown = e.value();
                        }
                    },
                }
            }
            label {
                "Confirmation prompt"
                input {
                    value: "{confirm_prompt}",
                    oninput: move |e| {
                        if let QuestionKind::Validation { confirm_prompt, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *confirm_prompt = e.value();
                        }
                    },
                }
            }
            label {
                "Summary item"
                input {
                    value: "{summary_item}",
                    oninput: move |e| {
                        if let QuestionKind::Validation { summary_item, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *summary_item = e.value();
                        }
                    },
                }
            }
        },

        QuestionKind::Text {
            description_markdown,
            placeholder,
            multiline,
            max_length,
        } => rsx! {
            label {
                "Description (Markdown)"
                textarea {
                    value: "{description_markdown.clone().unwrap_or_default()}",
                    oninput: move |e| {
                        if let QuestionKind::Text { description_markdown, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *description_markdown = optional_string(e.value());
                        }
                    },
                }
            }
            label {
                "Placeholder"
                input {
                    value: "{placeholder.clone().unwrap_or_default()}",
                    oninput: move |e| {
                        if let QuestionKind::Text { placeholder, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *placeholder = optional_string(e.value());
                        }
                    },
                }
            }
            label { class: "admin-checkbox-row",
                input {
                    r#type: "checkbox",
                    checked: multiline,
                    onchange: move |_| {
                        if let QuestionKind::Text { multiline, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *multiline = !*multiline;
                        }
                    },
                }
                "Multiline"
            }
            label {
                "Max length"
                input {
                    r#type: "number",
                    value: "{max_length.map(|v| v.to_string()).unwrap_or_default()}",
                    oninput: move |e| {
                        if let QuestionKind::Text { max_length, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *max_length = e.value().parse::<usize>().ok();
                        }
                    },
                }
            }
        },

        QuestionKind::Choice {
            description_markdown,
            options,
            allow_comment,
        } => rsx! {
            ChoiceKindProperties {
                draft,
                section_index,
                question_index,
                description_markdown,
                options,
                allow_comment,
                multi: false,
                min_selected: None,
                max_selected: None,
            }
        },

        QuestionKind::MultiChoice {
            description_markdown,
            options,
            min_selected,
            max_selected,
            allow_comment,
        } => rsx! {
            ChoiceKindProperties {
                draft,
                section_index,
                question_index,
                description_markdown,
                options,
                allow_comment,
                multi: true,
                min_selected,
                max_selected,
            }
        },

        QuestionKind::Email {
            description_markdown,
            placeholder,
        } => rsx! {
            SimpleTextKindProperties {
                draft,
                section_index,
                question_index,
                description_markdown,
                placeholder,
                kind_key: "email",
            }
        },

        QuestionKind::Phone {
            description_markdown,
            placeholder,
        } => rsx! {
            SimpleTextKindProperties {
                draft,
                section_index,
                question_index,
                description_markdown,
                placeholder,
                kind_key: "phone",
            }
        },

        QuestionKind::Date {
            description_markdown,
        } => rsx! {
            label {
                "Description (Markdown)"
                textarea {
                    value: "{description_markdown.clone().unwrap_or_default()}",
                    oninput: move |e| {
                        if let QuestionKind::Date { description_markdown } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *description_markdown = optional_string(e.value());
                        }
                    },
                }
            }
        },

        QuestionKind::Number {
            description_markdown,
            placeholder,
            min,
            max,
        } => rsx! {
            label {
                "Description (Markdown)"
                textarea {
                    value: "{description_markdown.clone().unwrap_or_default()}",
                    oninput: move |e| {
                        if let QuestionKind::Number { description_markdown, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *description_markdown = optional_string(e.value());
                        }
                    },
                }
            }
            label {
                "Placeholder"
                input {
                    value: "{placeholder.clone().unwrap_or_default()}",
                    oninput: move |e| {
                        if let QuestionKind::Number { placeholder, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *placeholder = optional_string(e.value());
                        }
                    },
                }
            }
            div { class: "admin-two-col",
                label {
                    "Min"
                    input {
                        r#type: "number",
                        value: "{min.map(|v| v.to_string()).unwrap_or_default()}",
                        oninput: move |e| {
                            if let QuestionKind::Number { min, .. } =
                                &mut draft.write().sections[section_index].questions[question_index].kind
                            {
                                *min = e.value().parse::<f64>().ok();
                            }
                        },
                    }
                }
                label {
                    "Max"
                    input {
                        r#type: "number",
                        value: "{max.map(|v| v.to_string()).unwrap_or_default()}",
                        oninput: move |e| {
                            if let QuestionKind::Number { max, .. } =
                                &mut draft.write().sections[section_index].questions[question_index].kind
                            {
                                *max = e.value().parse::<f64>().ok();
                            }
                        },
                    }
                }
            }
        },

        QuestionKind::Dropdown {
            description_markdown,
            options,
            allow_comment,
        } => rsx! {
            ChoiceKindProperties {
                draft,
                section_index,
                question_index,
                description_markdown,
                options,
                allow_comment,
                multi: false,
                min_selected: None,
                max_selected: None,
            }
        },

        QuestionKind::MultiDropdown {
            description_markdown,
            options,
            min_selected,
            max_selected,
            allow_comment,
        } => rsx! {
            ChoiceKindProperties {
                draft,
                section_index,
                question_index,
                description_markdown,
                options,
                allow_comment,
                multi: true,
                min_selected,
                max_selected,
            }
        },

        QuestionKind::RankedList {
            description_markdown,
            options,
            randomize_initial_order,
        } => rsx! {
            label {
                "Description (Markdown)"
                textarea {
                    value: "{description_markdown.clone().unwrap_or_default()}",
                    oninput: move |e| {
                        if let QuestionKind::RankedList { description_markdown, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *description_markdown = optional_string(e.value());
                        }
                    },
                }
            }
            label { class: "admin-checkbox-row",
                input {
                    r#type: "checkbox",
                    checked: randomize_initial_order,
                    onchange: move |_| {
                        if let QuestionKind::RankedList { randomize_initial_order, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *randomize_initial_order = !*randomize_initial_order;
                        }
                    },
                }
                "Randomize initial order"
            }
            h3 { "Items to rank" }
            div { class: "admin-option-list",
                for (option_index, option) in options.iter().enumerate() {
                    div {
                        class: "admin-option-row",
                        key: "{option.question_option_id}",
                        input {
                            value: "{option.label}",
                            oninput: move |e| {
                                if let QuestionKind::RankedList { options, .. } =
                                    &mut draft.write().sections[section_index].questions[question_index].kind
                                {
                                    options[option_index].label = e.value();
                                }
                            },
                        }
                        input {
                            placeholder: "Description",
                            value: "{option.description.clone().unwrap_or_default()}",
                            oninput: move |e| {
                                if let QuestionKind::RankedList { options, .. } =
                                    &mut draft.write().sections[section_index].questions[question_index].kind
                                {
                                    options[option_index].description = optional_string(e.value());
                                }
                            },
                        }
                        button {
                            class: "admin-icon-button",
                            r#type: "button",
                            onclick: move |_| {
                                if let QuestionKind::RankedList { options, .. } =
                                    &mut draft.write().sections[section_index].questions[question_index].kind
                                {
                                    if options.len() > 2 {
                                        options.remove(option_index);
                                    }
                                }
                            },
                            "Remove"
                        }
                    }
                }
            }
            button {
                class: "admin-secondary-button",
                r#type: "button",
                onclick: move |_| {
                    if let QuestionKind::RankedList { options, .. } =
                        &mut draft.write().sections[section_index].questions[question_index].kind
                    {
                        options.push(default_option(options.len()));
                    }
                },
                "Add item"
            }
        },

        QuestionKind::ContentBlock { content_markdown } => rsx! {
            label {
                "Content (Markdown)"
                textarea {
                    value: "{content_markdown}",
                    oninput: move |e| {
                        if let QuestionKind::ContentBlock { content_markdown } =
                            &mut draft.write().sections[section_index].questions[question_index].kind
                        {
                            *content_markdown = e.value();
                        }
                    },
                }
            }
        },
    }
}

#[component]
fn SimpleTextKindProperties(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    description_markdown: Option<String>,
    placeholder: Option<String>,
    kind_key: &'static str,
) -> Element {
    rsx! {
        label {
            "Description (Markdown)"
            textarea {
                value: "{description_markdown.clone().unwrap_or_default()}",
                oninput: move |e| {
                    let val = optional_string(e.value());
                    let kind = &mut draft
                        .write()
                        .sections[section_index]
                        .questions[question_index]
                        .kind;
                    match kind {
                        QuestionKind::Email { description_markdown, .. } => {
                            *description_markdown = val;
                        }
                        QuestionKind::Phone { description_markdown, .. } => {
                            *description_markdown = val;
                        }
                        _ => {}
                    }
                },
            }
        }
        label {
            "Placeholder"
            input {
                value: "{placeholder.clone().unwrap_or_default()}",
                oninput: move |e| {
                    let val = optional_string(e.value());
                    let kind = &mut draft
                        .write()
                        .sections[section_index]
                        .questions[question_index]
                        .kind;
                    match kind {
                        QuestionKind::Email { placeholder, .. } => *placeholder = val,
                        QuestionKind::Phone { placeholder, .. } => *placeholder = val,
                        _ => {}
                    }
                },
            }
        }
    }
}

#[component]
fn ChoiceKindProperties(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    description_markdown: Option<String>,
    options: Vec<QuestionOption>,
    allow_comment: bool,
    multi: bool,
    min_selected: Option<usize>,
    max_selected: Option<usize>,
) -> Element {
    rsx! {
        label {
            "Description (Markdown)"
            textarea {
                value: "{description_markdown.clone().unwrap_or_default()}",
                oninput: move |e| {
                    update_choice_kind(
                        draft,
                        section_index,
                        question_index,
                        |desc, _, _| {
                            *desc = optional_string(e.value());
                        },
                    );
                },
            }
        }
        label { class: "admin-checkbox-row",
            input {
                r#type: "checkbox",
                checked: allow_comment,
                onchange: move |_| {
                    update_choice_kind(
                        draft,
                        section_index,
                        question_index,
                        |_, _, allow| {
                            *allow = !*allow;
                        },
                    );
                },
            }
            "Allow comment"
        }

        if multi {
            div { class: "admin-two-col",
                label {
                    "Min selected"
                    input {
                        r#type: "number",
                        value: "{min_selected.map(|v| v.to_string()).unwrap_or_default()}",
                        oninput: move |e| {
                            if let QuestionKind::MultiChoice { min_selected, .. } =
                                &mut draft.write().sections[section_index].questions[question_index].kind
                            {
                                *min_selected = e.value().parse::<usize>().ok();
                            }
                        },
                    }
                }
                label {
                    "Max selected"
                    input {
                        r#type: "number",
                        value: "{max_selected.map(|v| v.to_string()).unwrap_or_default()}",
                        oninput: move |e| {
                            if let QuestionKind::MultiChoice { max_selected, .. } =
                                &mut draft.write().sections[section_index].questions[question_index].kind
                            {
                                *max_selected = e.value().parse::<usize>().ok();
                            }
                        },
                    }
                }
            }
        }

        h3 { "Options" }
        div { class: "admin-option-list",
            for (option_index, option) in options.iter().enumerate() {
                div {
                    class: "admin-option-row",
                    key: "{option.question_option_id}",
                    input {
                        value: "{option.label}",
                        oninput: move |e| {
                            update_choice_options(
                                draft,
                                section_index,
                                question_index,
                                |opts| {
                                    opts[option_index].label = e.value();
                                },
                            );
                        },
                    }
                    input {
                        placeholder: "Description",
                        value: "{option.description.clone().unwrap_or_default()}",
                        oninput: move |e| {
                            update_choice_options(
                                draft,
                                section_index,
                                question_index,
                                |opts| {
                                    opts[option_index].description = optional_string(e.value());
                                },
                            );
                        },
                    }
                    button {
                        class: "admin-icon-button",
                        r#type: "button",
                        onclick: move |_| {
                            update_choice_options(
                                draft,
                                section_index,
                                question_index,
                                |opts| {
                                    if opts.len() > 1 {
                                        opts.remove(option_index);
                                    }
                                },
                            );
                        },
                        "Remove"
                    }
                }
            }
        }
        button {
            class: "admin-secondary-button",
            r#type: "button",
            onclick: move |_| {
                update_choice_options(
                    draft,
                    section_index,
                    question_index,
                    |opts| {
                        opts.push(default_option(opts.len()));
                    },
                );
            },
            if multi {
                "Add option"
            } else {
                "Add option"
            }
        }
    }
}

// ── Read-only form definition (used in form_detail) ───────────────────────────

#[component]
pub fn ReadOnlyFormDefinition(form: Form) -> Element {
    rsx! {
        div { class: "admin-readonly-form",
            header {
                h2 { "{form.title}" }
                if let Some(description) = form.description_markdown.as_ref() {
                    MarkdownDescription { markdown: description.clone() }
                }
            }
            for section in form.sections.iter() {
                section {
                    class: "admin-readonly-section",
                    key: "{section.section_id}",
                    h3 { "{section.number}. {section.title}" }
                    if let Some(description) = section.description_markdown.as_ref() {
                        MarkdownDescription { markdown: description.clone() }
                    }
                    for question in section.questions.iter() {
                        article {
                            class: "admin-readonly-question",
                            key: "{question.question_id}",
                            h4 { "{question.number}. {question.title}" }
                            if question.required {
                                span { class: "admin-status admin-status--active", "Required" }
                            }
                            QuestionKindSummary { question: question.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionKindSummary(question: Question) -> Element {
    match question.kind {
        QuestionKind::Validation {
            description_markdown,
            confirm_prompt,
            summary_item,
        } => rsx! {
            MarkdownDescription { markdown: description_markdown }
            p {
                strong { "Prompt: " }
                "{confirm_prompt}"
            }
            p {
                strong { "Summary: " }
                "{summary_item}"
            }
        },
        QuestionKind::Text {
            description_markdown,
            placeholder,
            multiline,
            max_length,
        } => {
            let field_label = if multiline {
                "Text field (multiline)"
            } else {
                "Text field"
            };
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{field_label}" }
                if let Some(placeholder) = placeholder {
                    p {
                        strong { "Placeholder: " }
                        "{placeholder}"
                    }
                }
                if let Some(max_length) = max_length {
                    p {
                        strong { "Max length: " }
                        "{max_length}"
                    }
                }
            }
        }
        QuestionKind::Choice {
            description_markdown,
            options,
            allow_comment,
        } => {
            let label = if allow_comment {
                "Single choice with comment"
            } else {
                "Single choice"
            };
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{label}" }
                OptionList { options }
            }
        }
        QuestionKind::MultiChoice {
            description_markdown,
            options,
            min_selected,
            max_selected,
            allow_comment,
        } => {
            let label = if allow_comment {
                "Multi choice with comment"
            } else {
                "Multi choice"
            };
            let min = min_selected
                .map(|v| v.to_string())
                .unwrap_or_else(|| "0".to_string());
            let max = max_selected
                .map(|v| v.to_string())
                .unwrap_or_else(|| "any".to_string());
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{label}" }
                if min_selected.is_some() || max_selected.is_some() {
                    p { "Selection range: {min} to {max}" }
                }
                OptionList { options }
            }
        }
        QuestionKind::Email {
            description_markdown,
            placeholder,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            p { "Email field" }
            if let Some(ph) = placeholder {
                p {
                    strong { "Placeholder: " }
                    "{ph}"
                }
            }
        },
        QuestionKind::Phone {
            description_markdown,
            placeholder,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            p { "Phone field" }
            if let Some(ph) = placeholder {
                p {
                    strong { "Placeholder: " }
                    "{ph}"
                }
            }
        },
        QuestionKind::Date {
            description_markdown,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            p { "Date field" }
        },
        QuestionKind::Number {
            description_markdown,
            placeholder,
            min,
            max,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            p { "Number field" }
            if let Some(ph) = placeholder {
                p {
                    strong { "Placeholder: " }
                    "{ph}"
                }
            }
            if min.is_some() || max.is_some() {
                p {
                    strong { "Range: " }
                    "{min.map(|v| v.to_string()).unwrap_or_else(|| \"any\".to_string())} to {max.map(|v| v.to_string()).unwrap_or_else(|| \"any\".to_string())}"
                }
            }
        },
        QuestionKind::Dropdown {
            description_markdown,
            options,
            allow_comment,
        } => {
            let label = if allow_comment {
                "Dropdown with comment"
            } else {
                "Dropdown"
            };
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{label}" }
                OptionList { options }
            }
        }
        QuestionKind::MultiDropdown {
            description_markdown,
            options,
            min_selected,
            max_selected,
            allow_comment,
        } => {
            let label = if allow_comment {
                "Multi dropdown with comment"
            } else {
                "Multi dropdown"
            };
            let min = min_selected
                .map(|v| v.to_string())
                .unwrap_or_else(|| "0".to_string());
            let max = max_selected
                .map(|v| v.to_string())
                .unwrap_or_else(|| "any".to_string());
            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{label}" }
                if min_selected.is_some() || max_selected.is_some() {
                    p { "Selection range: {min} to {max}" }
                }
                OptionList { options }
            }
        }
        QuestionKind::RankedList {
            description_markdown,
            options,
            randomize_initial_order,
        } => rsx! {
            if let Some(description) = description_markdown {
                MarkdownDescription { markdown: description }
            }
            if randomize_initial_order {
                p { "Ranked list. Initial order randomized." }
            } else {
                p { "Ranked list. Initial order uses item order." }
            }
            OptionList { options }
        },
        QuestionKind::ContentBlock { content_markdown } => rsx! {
            MarkdownDescription { markdown: content_markdown }
        },
    }
}

#[component]
fn OptionList(options: Vec<QuestionOption>) -> Element {
    rsx! {
        ul {
            for option in options.iter() {
                li { key: "{option.question_option_id}", "{option.label}" }
            }
        }
    }
}

// ── Draft model ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Selection {
    Section(usize),
    Question(usize, usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuilderDraft {
    pub title: String,
    pub description_markdown: String,
    pub created_by: String,
    pub sections: Vec<Section>,
}

impl Default for BuilderDraft {
    fn default() -> Self {
        Self {
            title: String::new(),
            description_markdown: String::new(),
            created_by: String::new(),
            sections: vec![default_section(0)],
        }
    }
}

impl BuilderDraft {
    pub(super) fn push_question_to_last_section(&mut self, kind: &str) -> Selection {
        let section_index = self.sections.len().saturating_sub(1);
        let question_index = self.sections[section_index].questions.len();
        let question = Question {
            question_id: generated_id("question"),
            number: (question_index + 1) as u32,
            title: String::new(),
            required: true,
            kind: default_kind_for_value(kind),
        };
        self.sections[section_index].questions.push(question);
        self.renumber();
        Selection::Question(section_index, question_index)
    }

    pub fn add_section(&mut self) {
        self.sections.push(default_section(self.sections.len()));
        self.renumber();
    }

    fn remove_section(&mut self, index: usize) {
        if self.sections.len() > 1 && index < self.sections.len() {
            self.sections.remove(index);
            self.renumber();
        }
    }

    fn move_section(&mut self, index: usize, direction: isize) {
        let new_index = shifted_index(index, direction, self.sections.len());
        if new_index != index {
            self.sections.swap(index, new_index);
            self.renumber();
        }
    }

    fn remove_question(&mut self, section_index: usize, question_index: usize) {
        if let Some(section) = self.sections.get_mut(section_index) {
            if question_index < section.questions.len() {
                section.questions.remove(question_index);
                self.renumber();
            }
        }
    }

    fn move_question(&mut self, section_index: usize, question_index: usize, direction: isize) {
        if let Some(section) = self.sections.get_mut(section_index) {
            let new_index = shifted_index(question_index, direction, section.questions.len());
            if new_index != question_index {
                section.questions.swap(question_index, new_index);
                self.renumber();
            }
        }
    }

    fn renumber(&mut self) {
        for (si, section) in self.sections.iter_mut().enumerate() {
            section.number = (si + 1) as u32;
            for (qi, question) in section.questions.iter_mut().enumerate() {
                question.number = (qi + 1) as u32;
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn default_section(index: usize) -> Section {
    Section {
        section_id: generated_id("section"),
        number: (index + 1) as u32,
        title: String::new(),
        description_markdown: None,
        questions: vec![],
    }
}

pub fn default_kind_for_value(value: &str) -> QuestionKind {
    match value {
        "validation" => QuestionKind::Validation {
            description_markdown: String::new(),
            confirm_prompt: String::new(),
            summary_item: String::new(),
        },
        "choice" => QuestionKind::Choice {
            description_markdown: None,
            options: vec![default_option(0), default_option(1)],
            allow_comment: false,
        },
        "multi_choice" => QuestionKind::MultiChoice {
            description_markdown: None,
            options: vec![default_option(0), default_option(1)],
            min_selected: None,
            max_selected: None,
            allow_comment: false,
        },
        "text_multiline" => QuestionKind::Text {
            description_markdown: None,
            placeholder: None,
            multiline: true,
            max_length: None,
        },
        "email" => QuestionKind::Email {
            description_markdown: None,
            placeholder: None,
        },
        "phone" => QuestionKind::Phone {
            description_markdown: None,
            placeholder: None,
        },
        "date" => QuestionKind::Date {
            description_markdown: None,
        },
        "number" => QuestionKind::Number {
            description_markdown: None,
            placeholder: None,
            min: None,
            max: None,
        },
        "dropdown" => QuestionKind::Dropdown {
            description_markdown: None,
            options: vec![default_option(0), default_option(1)],
            allow_comment: false,
        },
        "multi_dropdown" => QuestionKind::MultiDropdown {
            description_markdown: None,
            options: vec![default_option(0), default_option(1)],
            min_selected: None,
            max_selected: None,
            allow_comment: false,
        },
        "ranked_list" => QuestionKind::RankedList {
            description_markdown: None,
            options: vec![default_option(0), default_option(1)],
            randomize_initial_order: true,
        },
        "content_block" => QuestionKind::ContentBlock {
            content_markdown: String::new(),
        },
        _ => QuestionKind::Text {
            description_markdown: None,
            placeholder: None,
            multiline: false,
            max_length: None,
        },
    }
}

fn default_option(index: usize) -> QuestionOption {
    QuestionOption {
        question_option_id: generated_id("option"),
        label: format!("Option {}", index + 1),
        description: None,
    }
}

fn generated_id(prefix: &str) -> String {
    let random = js_sys::Math::random().to_string().replace("0.", "");
    format!("{prefix}-{random}")
}

fn question_kind_value(kind: &QuestionKind) -> &'static str {
    match kind {
        QuestionKind::Validation { .. } => "validation",
        QuestionKind::Text {
            multiline: true, ..
        } => "text_multiline",
        QuestionKind::Text { .. } => "text",
        QuestionKind::Choice { .. } => "choice",
        QuestionKind::MultiChoice { .. } => "multi_choice",
        QuestionKind::Email { .. } => "email",
        QuestionKind::Phone { .. } => "phone",
        QuestionKind::Date { .. } => "date",
        QuestionKind::Number { .. } => "number",
        QuestionKind::Dropdown { .. } => "dropdown",
        QuestionKind::MultiDropdown { .. } => "multi_dropdown",
        QuestionKind::RankedList { .. } => "ranked_list",
        QuestionKind::ContentBlock { .. } => "content_block",
    }
}

fn shifted_index(index: usize, direction: isize, len: usize) -> usize {
    if len == 0 {
        return index;
    }
    let shifted = index as isize + direction;
    shifted.clamp(0, len.saturating_sub(1) as isize) as usize
}

fn update_choice_kind(
    mut draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    update: impl FnOnce(&mut Option<String>, &mut Vec<QuestionOption>, &mut bool),
) {
    let mut draft = draft.write();
    let kind = &mut draft.sections[section_index].questions[question_index].kind;
    match kind {
        QuestionKind::Choice {
            description_markdown,
            options,
            allow_comment,
        }
        | QuestionKind::MultiChoice {
            description_markdown,
            options,
            allow_comment,
            ..
        } => update(description_markdown, options, allow_comment),
        _ => {}
    }
}

fn update_choice_options(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    update: impl FnOnce(&mut Vec<QuestionOption>),
) {
    update_choice_kind(draft, section_index, question_index, |_, opts, _| {
        update(opts)
    });
}

fn validate_builder_draft(draft: &BuilderDraft) -> Result<(), String> {
    if draft.title.trim().is_empty() {
        return Err("Name is required.".to_string());
    }
    if draft.created_by.trim().is_empty() {
        return Err("Prepared by is required.".to_string());
    }
    if draft.sections.is_empty() {
        return Err("Add at least one section.".to_string());
    }
    for section in &draft.sections {
        if section.title.trim().is_empty() {
            return Err("Every section needs a title.".to_string());
        }
        if section.questions.is_empty() {
            return Err("Every section needs at least one field.".to_string());
        }
        for question in &section.questions {
            if question.title.trim().is_empty() {
                return Err("Every field needs a label.".to_string());
            }
            match &question.kind {
                QuestionKind::Validation {
                    description_markdown,
                    confirm_prompt,
                    summary_item,
                } if description_markdown.trim().is_empty()
                    || confirm_prompt.trim().is_empty()
                    || summary_item.trim().is_empty() =>
                {
                    return Err(
                        "Confirmation fields need description, prompt, and summary.".to_string()
                    );
                }
                QuestionKind::Choice { options, .. }
                | QuestionKind::MultiChoice { options, .. }
                | QuestionKind::Dropdown { options, .. }
                | QuestionKind::MultiDropdown { options, .. } => {
                    if options.iter().any(|o| o.label.trim().is_empty()) {
                        return Err("Choice options need labels.".to_string());
                    }
                }
                QuestionKind::RankedList { options, .. } => {
                    if options.len() < 2 {
                        return Err("Ranked list needs at least 2 items.".to_string());
                    }
                    if options.iter().any(|o| o.label.trim().is_empty()) {
                        return Err("Ranked list items need labels.".to_string());
                    }
                }
                QuestionKind::ContentBlock { content_markdown } => {
                    if content_markdown.trim().is_empty() {
                        return Err("Content blocks need content.".to_string());
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
