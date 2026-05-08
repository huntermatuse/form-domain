use super::choices::{
    ChoiceQuestionInput, DropdownQuestionInput, MultiChoiceQuestionInput,
    MultiDropdownQuestionInput,
};
use super::ranked_list::RankedListQuestionInput;
use super::state::SubmissionDraft;
use super::text_inputs::{NumberQuestionInput, SimpleTextInput, TextQuestionInput};
use super::validation_input::ValidationQuestionInput;
use crate::forms::model::{Question, QuestionKind};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

#[component]
pub(super) fn QuestionInput(
    question: Question,
    draft: Signal<SubmissionDraft>,
    is_missing_required: bool,
) -> Element {
    let question_class = if is_missing_required {
        "form-question form-question--missing"
    } else {
        "form-question"
    };

    rsx! {
        div { class: "{question_class}", id: "question-{question.question_id}",

            div { class: "form-question__heading",
                h3 { "{question.number}. {question.title}" }

                if question.required {
                    span { class: "form-question__required", "Required" }
                }
            }

            if is_missing_required {
                p { class: "form-question__error", "This question is required." }
            }

            match &question.kind {
                QuestionKind::Validation { description_markdown, confirm_prompt, .. } => {
                    rsx! {
                        ValidationQuestionInput {
                            question_id: question.question_id.clone(),
                            description_markdown: description_markdown.clone(),
                            confirm_prompt: confirm_prompt.clone(),
                            draft,
                        }
                    }
                }
                QuestionKind::Text {
                    description_markdown,
                    placeholder,
                    multiline,
                    max_length,
                } => rsx! {
                    TextQuestionInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        placeholder: placeholder.clone(),
                        multiline: *multiline,
                        max_length: *max_length,
                        draft,
                    }
                },
                QuestionKind::Choice { description_markdown, options, allow_comment } => {
                    rsx! {
                        ChoiceQuestionInput {
                            question_id: question.question_id.clone(),
                            description_markdown: description_markdown.clone(),
                            options: options.clone(),
                            allow_comment: *allow_comment,
                            draft,
                        }
                    }
                }
                QuestionKind::MultiChoice {
                    description_markdown,
                    options,
                    min_selected: _,
                    max_selected: _,
                    allow_comment,
                } => rsx! {
                    MultiChoiceQuestionInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        options: options.clone(),
                        allow_comment: *allow_comment,
                        draft,
                    }
                },
                QuestionKind::Email { description_markdown, placeholder } => rsx! {
                    SimpleTextInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        placeholder: placeholder.clone(),
                        input_type: "email",
                        draft,
                    }
                },
                QuestionKind::Phone { description_markdown, placeholder } => rsx! {
                    SimpleTextInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        placeholder: placeholder.clone(),
                        input_type: "tel",
                        draft,
                    }
                },
                QuestionKind::Date { description_markdown } => rsx! {
                    SimpleTextInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        placeholder: None,
                        input_type: "date",
                        draft,
                    }
                },
                QuestionKind::Number { description_markdown, placeholder, min, max } => {
                    rsx! {
                        NumberQuestionInput {
                            question_id: question.question_id.clone(),
                            description_markdown: description_markdown.clone(),
                            placeholder: placeholder.clone(),
                            min: *min,
                            max: *max,
                            draft,
                        }
                    }
                }
                QuestionKind::Dropdown { description_markdown, options, allow_comment } => {
                    rsx! {
                        DropdownQuestionInput {
                            question_id: question.question_id.clone(),
                            description_markdown: description_markdown.clone(),
                            options: options.clone(),
                            allow_comment: *allow_comment,
                            draft,
                        }
                    }
                }
                QuestionKind::MultiDropdown {
                    description_markdown,
                    options,
                    min_selected: _,
                    max_selected: _,
                    allow_comment,
                } => rsx! {
                    MultiDropdownQuestionInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        options: options.clone(),
                        allow_comment: *allow_comment,
                        draft,
                    }
                },
                QuestionKind::RankedList {
                    description_markdown,
                    options,
                    randomize_initial_order,
                } => rsx! {
                    RankedListQuestionInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        options: options.clone(),
                        randomize_initial_order: *randomize_initial_order,
                        draft,
                    }
                },
                QuestionKind::ContentBlock { content_markdown } => rsx! {
                    MarkdownDescription { markdown: content_markdown.clone() }
                },
            }
        }
    }
}
