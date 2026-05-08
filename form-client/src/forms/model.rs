use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Form {
    pub form_id: String,
    pub version: u32,
    pub title: String,
    pub description_markdown: Option<String>,
    pub meta: FormMeta,
    pub sections: Vec<Section>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormMeta {
    pub created_at: String,
    pub created_by: String,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub section_id: String,
    pub number: u32,
    pub title: String,
    pub description_markdown: Option<String>,
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub question_id: String,
    pub number: u32,
    pub title: String,
    pub required: bool,
    pub kind: QuestionKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QuestionKind {
    Validation {
        description_markdown: String,
        confirm_prompt: String,
        summary_item: String,
    },
    Text {
        description_markdown: Option<String>,
        placeholder: Option<String>,
        multiline: bool,
        max_length: Option<usize>,
    },
    Choice {
        description_markdown: Option<String>,
        options: Vec<QuestionOption>,
        allow_comment: bool,
    },
    MultiChoice {
        description_markdown: Option<String>,
        options: Vec<QuestionOption>,
        min_selected: Option<usize>,
        max_selected: Option<usize>,
        allow_comment: bool,
    },
    Email {
        description_markdown: Option<String>,
        placeholder: Option<String>,
    },
    Phone {
        description_markdown: Option<String>,
        placeholder: Option<String>,
    },
    Date {
        description_markdown: Option<String>,
    },
    Number {
        description_markdown: Option<String>,
        placeholder: Option<String>,
        min: Option<f64>,
        max: Option<f64>,
    },
    Dropdown {
        description_markdown: Option<String>,
        options: Vec<QuestionOption>,
        allow_comment: bool,
    },
    MultiDropdown {
        description_markdown: Option<String>,
        options: Vec<QuestionOption>,
        min_selected: Option<usize>,
        max_selected: Option<usize>,
        allow_comment: bool,
    },
    RankedList {
        description_markdown: Option<String>,
        options: Vec<QuestionOption>,
        #[serde(default = "default_true")]
        randomize_initial_order: bool,
    },
    ContentBlock {
        content_markdown: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionOption {
    pub question_option_id: String,
    pub label: String,
    pub description: Option<String>, // tooltip in ui
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompletedForm {
    pub completed_form_id: String,
    pub form: Form,
    pub submission: FormSubmission,
    pub responses: Vec<QuestionResponse>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormSubmission {
    pub company_name: String,
    pub signer_name: String,
    pub signer_title: String,
    pub submitted_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionResponse {
    pub question_id: String,
    pub response: Response,
    pub answered_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Validation {
        status: ValidationStatus,
        comment: Option<String>,
    },
    Text {
        value: String,
    },
    Choice {
        selected_option_id: String,
        comment: Option<String>,
    },
    MultiChoice {
        selected_option_ids: Vec<String>,
        comment: Option<String>,
    },
    RankedList {
        ranked_option_ids: Vec<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Confirmed,
    NotCorrect,
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::{QuestionKind, QuestionOption};
    use serde_json::json;

    #[test]
    fn legacy_ranked_list_defaults_to_randomized_initial_order() {
        let kind: QuestionKind = serde_json::from_value(json!({
            "type": "ranked_list",
            "description_markdown": null,
            "options": [
                {
                    "question_option_id": "first",
                    "label": "First",
                    "description": null
                },
                {
                    "question_option_id": "second",
                    "label": "Second",
                    "description": null
                }
            ]
        }))
        .expect("legacy ranked list should deserialize");

        assert_eq!(
            kind,
            QuestionKind::RankedList {
                description_markdown: None,
                options: vec![
                    QuestionOption {
                        question_option_id: "first".to_string(),
                        label: "First".to_string(),
                        description: None,
                    },
                    QuestionOption {
                        question_option_id: "second".to_string(),
                        label: "Second".to_string(),
                        description: None,
                    },
                ],
                randomize_initial_order: true,
            }
        );
    }
}
