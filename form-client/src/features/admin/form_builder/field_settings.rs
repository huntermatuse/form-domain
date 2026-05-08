use super::helpers::default_option;
use super::state::BuilderDraft;
use crate::features::admin::shared::optional_string;
use crate::forms::model::{QuestionKind, QuestionOption};
use dioxus::prelude::*;

#[component]
pub(super) fn QuestionKindProperties(
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
                            match &mut draft.write().sections[section_index].questions[question_index].kind {
                                QuestionKind::MultiChoice { min_selected, .. }
                                | QuestionKind::MultiDropdown { min_selected, .. } => {
                                    *min_selected = e.value().parse::<usize>().ok();
                                }
                                _ => {}
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
                            match &mut draft.write().sections[section_index].questions[question_index].kind {
                                QuestionKind::MultiChoice { max_selected, .. }
                                | QuestionKind::MultiDropdown { max_selected, .. } => {
                                    *max_selected = e.value().parse::<usize>().ok();
                                }
                                _ => {}
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
            "Add option"
        }
    }
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
        }
        | QuestionKind::Dropdown {
            description_markdown,
            options,
            allow_comment,
        }
        | QuestionKind::MultiDropdown {
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
