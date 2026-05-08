use super::state::BuilderDraft;
use crate::forms::model::{Form, FormMeta, QuestionKind, QuestionOption, Section};

pub(super) fn draft_to_form(draft: &BuilderDraft) -> Form {
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

pub(super) fn default_section(index: usize) -> Section {
    Section {
        section_id: generated_id("section"),
        number: (index + 1) as u32,
        title: String::new(),
        description_markdown: None,
        questions: vec![],
    }
}

pub(super) fn default_kind_for_value(value: &str) -> QuestionKind {
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

pub(super) fn default_option(index: usize) -> QuestionOption {
    QuestionOption {
        question_option_id: generated_id("option"),
        label: format!("Option {}", index + 1),
        description: None,
    }
}

pub(super) fn generated_id(prefix: &str) -> String {
    let random = js_sys::Math::random().to_string().replace("0.", "");
    format!("{prefix}-{random}")
}

pub(super) fn question_kind_value(kind: &QuestionKind) -> &'static str {
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

pub(super) fn shifted_index(index: usize, direction: isize, len: usize) -> usize {
    if len == 0 {
        return index;
    }
    let shifted = index as isize + direction;
    shifted.clamp(0, len.saturating_sub(1) as isize) as usize
}
