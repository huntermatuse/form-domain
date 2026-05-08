use super::response_helpers::{
    current_choice_comment, current_choice_option, current_multi_choice_options,
    empty_string_as_none, response_for, toggle_multi_choice_response, upsert_response,
};
use super::state::SubmissionDraft;
use crate::forms::model::{QuestionOption, Response};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

#[component]
pub(super) fn ChoiceQuestionInput(
    question_id: String,
    description_markdown: Option<String>,
    options: Vec<QuestionOption>,
    allow_comment: bool,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let current = response_for(&draft.read().responses, &question_id).cloned();
    let selected_option_id = match current.as_ref().map(|response| &response.response) {
        Some(Response::Choice {
            selected_option_id, ..
        }) => Some(selected_option_id.clone()),
        _ => None,
    };
    let comment = match current.as_ref().map(|response| &response.response) {
        Some(Response::Choice { comment, .. }) => comment.clone().unwrap_or_default(),
        _ => String::new(),
    };

    rsx! {
        if let Some(description_markdown) = description_markdown {
            MarkdownDescription { markdown: description_markdown }
        }

        div { class: "choice-list",
            for option in options.iter() {
                label {
                    key: "{option.question_option_id}",
                    title: option.description.clone().unwrap_or_default(),

                    input {
                        r#type: "radio",
                        name: "{question_id}",
                        checked: selected_option_id.as_deref() == Some(option.question_option_id.as_str()),
                        onchange: {
                            let question_id = question_id.clone();
                            let option_id = option.question_option_id.clone();
                            move |_| {
                                let comment =
                                    current_choice_comment(&draft.read().responses, &question_id);

                                upsert_response(
                                    draft,
                                    question_id.clone(),
                                    Response::Choice {
                                        selected_option_id: option_id.clone(),
                                        comment,
                                    },
                                );
                            }
                        },
                    }

                    "{option.label}"
                }
            }
        }

        if allow_comment {
            textarea {
                placeholder: "Comment",
                value: "{comment}",
                oninput: {
                    let question_id = question_id.clone();
                    move |event| {
                        let selected_option_id =
                            current_choice_option(&draft.read().responses, &question_id);
                        if let Some(selected_option_id) = selected_option_id {
                            upsert_response(
                                draft,
                                question_id.clone(),
                                Response::Choice {
                                    selected_option_id,
                                    comment: empty_string_as_none(event.value()),
                                },
                            );
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub(super) fn MultiChoiceQuestionInput(
    question_id: String,
    description_markdown: Option<String>,
    options: Vec<QuestionOption>,
    allow_comment: bool,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let current = response_for(&draft.read().responses, &question_id).cloned();
    let selected_option_ids = match current.as_ref().map(|response| &response.response) {
        Some(Response::MultiChoice {
            selected_option_ids,
            ..
        }) => selected_option_ids.clone(),
        _ => Vec::new(),
    };
    let comment = match current.as_ref().map(|response| &response.response) {
        Some(Response::MultiChoice { comment, .. }) => comment.clone().unwrap_or_default(),
        _ => String::new(),
    };

    rsx! {
        if let Some(description_markdown) = description_markdown {
            MarkdownDescription { markdown: description_markdown }
        }

        div { class: "choice-list",
            for option in options.iter() {
                label {
                    key: "{option.question_option_id}",
                    title: option.description.clone().unwrap_or_default(),

                    input {
                        r#type: "checkbox",
                        checked: selected_option_ids.contains(&option.question_option_id),
                        onchange: {
                            let question_id = question_id.clone();
                            let option_id = option.question_option_id.clone();
                            move |_| {
                                toggle_multi_choice_response(draft, question_id.clone(), option_id.clone());
                            }
                        },
                    }

                    "{option.label}"
                }
            }
        }

        if allow_comment {
            textarea {
                placeholder: "Comment",
                value: "{comment}",
                oninput: {
                    let question_id = question_id.clone();
                    move |event| {
                        let selected_option_ids =
                            current_multi_choice_options(&draft.read().responses, &question_id);
                        let comment = empty_string_as_none(event.value());
                        upsert_response(
                            draft,
                            question_id.clone(),
                            Response::MultiChoice {
                                selected_option_ids,
                                comment,
                            },
                        );
                    }
                },
            }
        }
    }
}

#[component]
pub(super) fn DropdownQuestionInput(
    question_id: String,
    description_markdown: Option<String>,
    options: Vec<QuestionOption>,
    allow_comment: bool,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let current = response_for(&draft.read().responses, &question_id).cloned();
    let selected_option_id = match current.as_ref().map(|r| &r.response) {
        Some(Response::Choice {
            selected_option_id, ..
        }) => Some(selected_option_id.clone()),
        _ => None,
    };
    let comment = match current.as_ref().map(|r| &r.response) {
        Some(Response::Choice { comment, .. }) => comment.clone().unwrap_or_default(),
        _ => String::new(),
    };

    rsx! {
        if let Some(md) = description_markdown {
            MarkdownDescription { markdown: md }
        }
        select {
            onchange: {
                let question_id = question_id.clone();
                move |event: Event<FormData>| {
                    let val = event.value();
                    if val.is_empty() {
                        return;
                    }
                    let comment = current_choice_comment(&draft.read().responses, &question_id);
                    upsert_response(
                        draft,
                        question_id.clone(),
                        Response::Choice {
                            selected_option_id: val,
                            comment,
                        },
                    );
                }
            },
            option { value: "", "— Select —" }
            for option in options.iter() {
                option {
                    key: "{option.question_option_id}",
                    value: "{option.question_option_id}",
                    selected: selected_option_id.as_deref() == Some(option.question_option_id.as_str()),
                    "{option.label}"
                }
            }
        }
        if allow_comment {
            textarea {
                placeholder: "Comment",
                value: "{comment}",
                oninput: {
                    let question_id = question_id.clone();
                    move |event| {
                        let selected_option_id = current_choice_option(
                            &draft.read().responses,
                            &question_id,
                        );
                        if let Some(selected_option_id) = selected_option_id {
                            upsert_response(
                                draft,
                                question_id.clone(),
                                Response::Choice {
                                    selected_option_id,
                                    comment: empty_string_as_none(event.value()),
                                },
                            );
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub(super) fn MultiDropdownQuestionInput(
    question_id: String,
    description_markdown: Option<String>,
    options: Vec<QuestionOption>,
    allow_comment: bool,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let current = response_for(&draft.read().responses, &question_id).cloned();
    let selected_option_ids = match current.as_ref().map(|r| &r.response) {
        Some(Response::MultiChoice {
            selected_option_ids,
            ..
        }) => selected_option_ids.clone(),
        _ => Vec::new(),
    };
    let comment = match current.as_ref().map(|r| &r.response) {
        Some(Response::MultiChoice { comment, .. }) => comment.clone().unwrap_or_default(),
        _ => String::new(),
    };

    rsx! {
        if let Some(md) = description_markdown {
            MarkdownDescription { markdown: md }
        }
        div { class: "dropdown-multi-list",
            for option in options.iter() {
                label {
                    key: "{option.question_option_id}",
                    class: "dropdown-multi-list__item",
                    title: option.description.clone().unwrap_or_default(),

                    input {
                        r#type: "checkbox",
                        checked: selected_option_ids.contains(&option.question_option_id),
                        onchange: {
                            let question_id = question_id.clone();
                            let option_id = option.question_option_id.clone();
                            move |_| {
                                toggle_multi_choice_response(
                                    draft,
                                    question_id.clone(),
                                    option_id.clone(),
                                );
                            }
                        },
                    }

                    "{option.label}"
                }
            }
        }
        if allow_comment {
            textarea {
                placeholder: "Comment",
                value: "{comment}",
                oninput: {
                    let question_id = question_id.clone();
                    move |event| {
                        let selected_option_ids = current_multi_choice_options(
                            &draft.read().responses,
                            &question_id,
                        );
                        upsert_response(
                            draft,
                            question_id.clone(),
                            Response::MultiChoice {
                                selected_option_ids,
                                comment: empty_string_as_none(event.value()),
                            },
                        );
                    }
                },
            }
        }
    }
}
