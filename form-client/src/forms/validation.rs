use crate::forms::model::{Form, Question, QuestionResponse, Response};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MissingRequiredQuestion {
    pub question_id: String,
    pub title: String,
}

pub fn missing_required_questions(
    form: &Form,
    responses: &[QuestionResponse],
) -> Vec<MissingRequiredQuestion> {
    form.sections
        .iter()
        .flat_map(|section| section.questions.iter())
        .filter(|question| question.required)
        .filter(|question| !has_required_response(question, responses))
        .map(|question| MissingRequiredQuestion {
            question_id: question.question_id.clone(),
            title: question.title.clone(),
        })
        .collect()
}

pub fn has_required_response(question: &Question, responses: &[QuestionResponse]) -> bool {
    responses
        .iter()
        .find(|response| response.question_id == question.question_id)
        .map(|response| response_satisfies_required(&response.response))
        .unwrap_or(false)
}

fn response_satisfies_required(response: &Response) -> bool {
    match response {
        Response::Validation { .. } => true,
        Response::Text { value } => !value.trim().is_empty(),
        Response::Choice {
            selected_option_id, ..
        } => !selected_option_id.trim().is_empty(),
        Response::MultiChoice {
            selected_option_ids,
            ..
        } => !selected_option_ids.is_empty(),
    }
}
