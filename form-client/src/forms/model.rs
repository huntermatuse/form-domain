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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Confirmed,
    NotCorrect,
}
