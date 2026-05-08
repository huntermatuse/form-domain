use super::state::SubmissionDraft;
use crate::forms::model::{
    CompletedForm, Form, FormSubmission, QuestionResponse, Response, ValidationStatus,
};
use dioxus::prelude::*;

pub(super) fn build_completed_form(form: &Form, draft: &SubmissionDraft) -> CompletedForm {
    CompletedForm {
        completed_form_id: completed_form_id(form, draft),
        form: form.clone(),
        submission: FormSubmission {
            company_name: draft.company_name.clone(),
            signer_name: draft.signer_name.clone(),
            signer_title: draft.signer_title.clone(),
            submitted_at: draft.submitted_at.clone(),
        },
        responses: draft.responses.clone(),
    }
}

fn completed_form_id(form: &Form, draft: &SubmissionDraft) -> String {
    let submitted_at = draft.submitted_at.trim();

    if submitted_at.is_empty() {
        format!("{}-submission", form.form_id)
    } else {
        format!("{}-{submitted_at}", form.form_id)
    }
}

pub(super) fn response_for<'a>(
    responses: &'a [QuestionResponse],
    question_id: &str,
) -> Option<&'a QuestionResponse> {
    responses
        .iter()
        .find(|response| response.question_id == question_id)
}

pub(super) fn upsert_response(
    mut draft: Signal<SubmissionDraft>,
    question_id: String,
    response: Response,
) {
    let mut draft = draft.write();

    if let Some(existing) = draft
        .responses
        .iter_mut()
        .find(|existing| existing.question_id == question_id)
    {
        existing.response = response;
        return;
    }

    draft.responses.push(QuestionResponse {
        question_id,
        response,
        answered_at: None,
    });
}

pub(super) fn set_validation_response(
    draft: Signal<SubmissionDraft>,
    question_id: String,
    status: ValidationStatus,
    comment: Option<String>,
) {
    upsert_response(draft, question_id, Response::Validation { status, comment });
}

pub(super) fn current_validation_status(
    responses: &[QuestionResponse],
    question_id: &str,
) -> Option<ValidationStatus> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::Validation { status, .. }) => Some(status.clone()),
        _ => None,
    }
}

pub(super) fn current_validation_comment(
    responses: &[QuestionResponse],
    question_id: &str,
) -> Option<String> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::Validation { comment, .. }) => comment.clone(),
        _ => None,
    }
}

pub(super) fn current_choice_option(
    responses: &[QuestionResponse],
    question_id: &str,
) -> Option<String> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::Choice {
            selected_option_id, ..
        }) => Some(selected_option_id.clone()),
        _ => None,
    }
}

pub(super) fn current_choice_comment(
    responses: &[QuestionResponse],
    question_id: &str,
) -> Option<String> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::Choice { comment, .. }) => comment.clone(),
        _ => None,
    }
}

pub(super) fn current_multi_choice_options(
    responses: &[QuestionResponse],
    question_id: &str,
) -> Vec<String> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::MultiChoice {
            selected_option_ids,
            ..
        }) => selected_option_ids.clone(),
        _ => Vec::new(),
    }
}

pub(super) fn toggle_multi_choice_response(
    draft: Signal<SubmissionDraft>,
    question_id: String,
    option_id: String,
) {
    let responses = draft.read().responses.clone();
    let mut selected_option_ids = current_multi_choice_options(&responses, &question_id);
    let comment = match response_for(&responses, &question_id).map(|response| &response.response) {
        Some(Response::MultiChoice { comment, .. }) => comment.clone(),
        _ => None,
    };

    if let Some(index) = selected_option_ids
        .iter()
        .position(|selected| selected == &option_id)
    {
        selected_option_ids.remove(index);
    } else {
        selected_option_ids.push(option_id);
    }

    upsert_response(
        draft,
        question_id,
        Response::MultiChoice {
            selected_option_ids,
            comment,
        },
    );
}

pub(super) fn empty_string_as_none(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}
