use crate::forms::model::{Form, Question, QuestionKind, QuestionResponse, Response};

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
        .map(|response| response_satisfies_required(question, &response.response))
        .unwrap_or(false)
}

fn response_satisfies_required(question: &Question, response: &Response) -> bool {
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
        Response::RankedList { ranked_option_ids } => {
            ranked_list_is_complete(question, ranked_option_ids)
        }
    }
}

fn ranked_list_is_complete(question: &Question, ranked_option_ids: &[String]) -> bool {
    let QuestionKind::RankedList { options, .. } = &question.kind else {
        return !ranked_option_ids.is_empty();
    };

    ranked_option_ids.len() == options.len()
        && options
            .iter()
            .all(|option| ranked_option_ids.contains(&option.question_option_id))
}

#[cfg(test)]
mod tests {
    use super::has_required_response;
    use crate::forms::model::{Question, QuestionKind, QuestionOption, QuestionResponse, Response};

    #[test]
    fn required_ranked_list_requires_all_options() {
        let question = ranked_list_question();

        assert!(!has_required_response(
            &question,
            &[ranked_response(vec!["first"])]
        ));

        assert!(has_required_response(
            &question,
            &[ranked_response(vec!["second", "first"])]
        ));
    }

    fn ranked_list_question() -> Question {
        Question {
            question_id: "priority".to_string(),
            number: 1,
            title: "Priority".to_string(),
            required: true,
            kind: QuestionKind::RankedList {
                description_markdown: None,
                options: vec![ranked_option("first"), ranked_option("second")],
                randomize_initial_order: true,
            },
        }
    }

    fn ranked_option(id: &str) -> QuestionOption {
        QuestionOption {
            question_option_id: id.to_string(),
            label: id.to_string(),
            description: None,
        }
    }

    fn ranked_response(ids: Vec<&str>) -> QuestionResponse {
        QuestionResponse {
            question_id: "priority".to_string(),
            response: Response::RankedList {
                ranked_option_ids: ids.into_iter().map(str::to_string).collect(),
            },
            answered_at: None,
        }
    }
}
