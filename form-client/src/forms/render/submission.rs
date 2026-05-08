use crate::forms::model::{
    CompletedForm, Form, FormSubmission, Question, QuestionKind, QuestionOption, QuestionResponse,
    Response, ValidationStatus,
};
use crate::forms::render::markdown::MarkdownDescription;
use crate::forms::validation::{missing_required_questions, MissingRequiredQuestion};
use dioxus::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
struct SubmissionDraft {
    company_name: String,
    signer_name: String,
    signer_title: String,
    submitted_at: String,
    responses: Vec<QuestionResponse>,
}

#[component]
pub fn FormSubmissionRenderer(
    form: Form,
    is_submitting: bool,
    on_submit: Option<EventHandler<CompletedForm>>,
) -> Element {
    let draft = use_signal(SubmissionDraft::default);
    let mut missing_required = use_signal(Vec::<MissingRequiredQuestion>::new);

    rsx! {
        div { class: "form-submission",
            header { class: "form-submission__header",
                h1 { "{form.title}" }

                if let Some(description) = &form.description_markdown {
                    MarkdownDescription {
                        markdown: description.clone(),
                    }
                }
            }

            if !missing_required.read().is_empty() {
                div { class: "form-submission__errors",
                    strong { "Please complete the required questions before submitting." }

                    ul {
                        for missing in missing_required.read().iter() {
                            li { key: "{missing.question_id}", "{missing.title}" }
                        }
                    }
                }
            }

            for section in form.sections.iter() {
                section {
                    key: "{section.section_id}",
                    class: "form-section",

                    h2 { "{section.number}. {section.title}" }

                    if let Some(description) = &section.description_markdown {
                        MarkdownDescription {
                            markdown: description.clone(),
                        }
                    }

                    for question in section.questions.iter() {
                        QuestionInput {
                            key: "{question.question_id}",
                            question: question.clone(),
                            draft,
                            is_missing_required: missing_required
                                .read()
                                .iter()
                                .any(|missing| missing.question_id == question.question_id),
                        }
                    }
                }
            }

            SubmissionFields {
                draft,
            }

            div { class: "form-actions",
                button {
                    r#type: "button",
                    disabled: is_submitting,
                    onclick: move |_| {
                        if is_submitting {
                            return;
                        }

                        let missing = missing_required_questions(&form, &draft.read().responses);

                        if !missing.is_empty() {
                            missing_required.set(missing);
                            return;
                        }

                        missing_required.set(Vec::new());

                        let completed_form = build_completed_form(&form, &draft.read());

                        if let Some(on_submit) = &on_submit {
                            on_submit.call(completed_form);
                        }
                    },
                    if is_submitting {
                        "Submitting..."
                    } else {
                        "Submit"
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionInput(
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
        div {
            class: "{question_class}",
            id: "question-{question.question_id}",

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
                QuestionKind::Validation {
                    description_markdown,
                    confirm_prompt,
                    ..
                } => rsx! {
                    ValidationQuestionInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        confirm_prompt: confirm_prompt.clone(),
                        draft,
                    }
                },
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
                QuestionKind::Choice {
                    description_markdown,
                    options,
                    allow_comment,
                } => rsx! {
                    ChoiceQuestionInput {
                        question_id: question.question_id.clone(),
                        description_markdown: description_markdown.clone(),
                        options: options.clone(),
                        allow_comment: *allow_comment,
                        draft,
                    }
                },
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
            }
        }
    }
}

#[component]
fn ValidationQuestionInput(
    question_id: String,
    description_markdown: String,
    confirm_prompt: String,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let current = response_for(&draft.read().responses, &question_id).cloned();
    let status = match current.as_ref().map(|response| &response.response) {
        Some(Response::Validation { status, .. }) => Some(status.clone()),
        _ => None,
    };
    let comment = match current.as_ref().map(|response| &response.response) {
        Some(Response::Validation { comment, .. }) => comment.clone().unwrap_or_default(),
        _ => String::new(),
    };

    rsx! {
        MarkdownDescription {
            markdown: description_markdown,
        }

        div { class: "form-question__prompt",
            strong { "Please confirm:" }
            " {confirm_prompt}"
        }

        div { class: "radio-row",
            label {
                input {
                    r#type: "radio",
                    name: "{question_id}",
                    checked: status == Some(ValidationStatus::Confirmed),
                    onchange: {
                        let question_id = question_id.clone();
                        move |_| {
                            let comment =
                                current_validation_comment(&draft.read().responses, &question_id);

                            set_validation_response(
                                draft,
                                question_id.clone(),
                                ValidationStatus::Confirmed,
                                comment,
                            );
                        }
                    },
                }

                "Confirmed"
            }

            label {
                input {
                    r#type: "radio",
                    name: "{question_id}",
                    checked: status == Some(ValidationStatus::NotCorrect),
                    onchange: {
                        let question_id = question_id.clone();
                        move |_| {
                            let comment =
                                current_validation_comment(&draft.read().responses, &question_id);

                            set_validation_response(
                                draft,
                                question_id.clone(),
                                ValidationStatus::NotCorrect,
                                comment,
                            );
                        }
                    },
                }

                "Not correct"
            }
        }

        textarea {
            placeholder: "Comments or corrections",
            value: "{comment}",
            oninput: {
                let question_id = question_id.clone();
                move |event| {
                    let status = current_validation_status(&draft.read().responses, &question_id);

                    if let Some(status) = status {
                        set_validation_response(
                            draft,
                            question_id.clone(),
                            status,
                            empty_string_as_none(event.value()),
                        );
                    }
                }
            },
        }
    }
}

#[component]
fn TextQuestionInput(
    question_id: String,
    description_markdown: Option<String>,
    placeholder: Option<String>,
    multiline: bool,
    max_length: Option<usize>,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let value = match response_for(&draft.read().responses, &question_id)
        .map(|response| &response.response)
    {
        Some(Response::Text { value }) => value.clone(),
        _ => String::new(),
    };
    let placeholder = placeholder.unwrap_or_default();

    rsx! {
        if let Some(description_markdown) = description_markdown {
            MarkdownDescription {
                markdown: description_markdown,
            }
        }

        if multiline {
            textarea {
                placeholder: "{placeholder}",
                value: "{value}",
                maxlength: max_length.map(|length| length.to_string()),
                oninput: move |event| {
                    upsert_response(
                        draft,
                        question_id.clone(),
                        Response::Text {
                            value: event.value(),
                        },
                    );
                },
            }
        } else {
            input {
                r#type: "text",
                placeholder: "{placeholder}",
                value: "{value}",
                maxlength: max_length.map(|length| length.to_string()),
                oninput: move |event| {
                    upsert_response(
                        draft,
                        question_id.clone(),
                        Response::Text {
                            value: event.value(),
                        },
                    );
                },
            }
        }
    }
}

#[component]
fn ChoiceQuestionInput(
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
            MarkdownDescription {
                markdown: description_markdown,
            }
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
fn MultiChoiceQuestionInput(
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
            MarkdownDescription {
                markdown: description_markdown,
            }
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
fn SubmissionFields(draft: Signal<SubmissionDraft>) -> Element {
    let current = draft.read().clone();

    rsx! {
        section { class: "form-submission-details",
            h2 { "Submission" }

            div { class: "form-submission-details__grid",
                TextInput {
                    id: "company_name",
                    label: "Company",
                    input_type: "text",
                    value: current.company_name,
                    oninput: move |value| {
                        draft.write().company_name = value;
                    },
                }

                TextInput {
                    id: "signer_name",
                    label: "Signer name",
                    input_type: "text",
                    value: current.signer_name,
                    oninput: move |value| {
                        draft.write().signer_name = value;
                    },
                }

                TextInput {
                    id: "signer_title",
                    label: "Signer title",
                    input_type: "text",
                    value: current.signer_title,
                    oninput: move |value| {
                        draft.write().signer_title = value;
                    },
                }

                TextInput {
                    id: "submitted_at",
                    label: "Submission date",
                    input_type: "date",
                    value: current.submitted_at,
                    oninput: move |value| {
                        draft.write().submitted_at = value;
                    },
                }
            }
        }
    }
}

#[component]
fn TextInput(
    id: &'static str,
    label: &'static str,
    input_type: &'static str,
    value: String,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "field",
            label {
                r#for: "{id}",
                "{label}"
            }

            input {
                id: "{id}",
                r#type: "{input_type}",
                value: "{value}",
                oninput: move |event| {
                    oninput.call(event.value());
                },
            }
        }
    }
}

fn build_completed_form(form: &Form, draft: &SubmissionDraft) -> CompletedForm {
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

fn response_for<'a>(
    responses: &'a [QuestionResponse],
    question_id: &str,
) -> Option<&'a QuestionResponse> {
    responses
        .iter()
        .find(|response| response.question_id == question_id)
}

fn upsert_response(mut draft: Signal<SubmissionDraft>, question_id: String, response: Response) {
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

fn set_validation_response(
    draft: Signal<SubmissionDraft>,
    question_id: String,
    status: ValidationStatus,
    comment: Option<String>,
) {
    upsert_response(draft, question_id, Response::Validation { status, comment });
}

fn current_validation_status(
    responses: &[QuestionResponse],
    question_id: &str,
) -> Option<ValidationStatus> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::Validation { status, .. }) => Some(status.clone()),
        _ => None,
    }
}

fn current_validation_comment(responses: &[QuestionResponse], question_id: &str) -> Option<String> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::Validation { comment, .. }) => comment.clone(),
        _ => None,
    }
}

fn current_choice_option(responses: &[QuestionResponse], question_id: &str) -> Option<String> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::Choice {
            selected_option_id, ..
        }) => Some(selected_option_id.clone()),
        _ => None,
    }
}

fn current_choice_comment(responses: &[QuestionResponse], question_id: &str) -> Option<String> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::Choice { comment, .. }) => comment.clone(),
        _ => None,
    }
}

fn current_multi_choice_options(responses: &[QuestionResponse], question_id: &str) -> Vec<String> {
    match response_for(responses, question_id).map(|response| &response.response) {
        Some(Response::MultiChoice {
            selected_option_ids,
            ..
        }) => selected_option_ids.clone(),
        _ => Vec::new(),
    }
}

fn toggle_multi_choice_response(
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

fn empty_string_as_none(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}
