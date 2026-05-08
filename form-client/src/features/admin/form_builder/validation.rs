use super::state::BuilderDraft;
use crate::forms::model::QuestionKind;

pub(super) fn validate_builder_draft(draft: &BuilderDraft) -> Result<(), String> {
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
