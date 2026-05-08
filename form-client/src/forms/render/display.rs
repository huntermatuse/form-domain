use crate::forms::model::{
    Question, QuestionKind, QuestionOption, QuestionResponse, ValidationStatus,
};

pub fn response_for<'a>(
    responses: &'a [QuestionResponse],
    question_id: &str,
) -> Option<&'a QuestionResponse> {
    responses
        .iter()
        .find(|response| response.question_id == question_id)
}

pub fn validation_status_label(status: &ValidationStatus) -> &'static str {
    match status {
        ValidationStatus::Confirmed => "Confirmed",
        ValidationStatus::NotCorrect => "Not correct",
    }
}

pub fn choice_label(question: &Question, option_id: &str) -> String {
    question_options(question)
        .iter()
        .find(|option| option.question_option_id == option_id)
        .map(|option| option.label.clone())
        .unwrap_or_else(|| option_id.to_string())
}

pub fn display_or_empty(value: &str) -> &str {
    if value.trim().is_empty() {
        "Not provided"
    } else {
        value
    }
}

fn question_options(question: &Question) -> &[QuestionOption] {
    match &question.kind {
        QuestionKind::Choice { options, .. }
        | QuestionKind::MultiChoice { options, .. }
        | QuestionKind::Dropdown { options, .. }
        | QuestionKind::MultiDropdown { options, .. }
        | QuestionKind::RankedList { options, .. } => options,
        _ => &[],
    }
}
