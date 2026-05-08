use crate::api;
use crate::api::http::{clear_auth_token, has_auth_token, ApiError};
use crate::forms::model::{
    CompletedForm, Form, FormMeta, Question, QuestionKind, QuestionOption, Response, Section,
    ValidationStatus,
};
use crate::forms::render::markdown::MarkdownDescription;
use crate::forms::render::viewer::CompletedFormViewer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AdminLoginPage() -> Element {
    let navigator = use_navigator();
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut is_submitting = use_signal(|| false);

    rsx! {
        main { class: "admin-shell admin-auth-shell",
            section { class: "admin-panel admin-login-panel",
                h1 { "Admin" }
                p { class: "admin-muted", "Enter the site password to continue." }

                if let Some(message) = error.read().as_ref() {
                    p { class: "admin-error", "{message}" }
                }

                form {
                    class: "admin-form-stack",
                    onsubmit: move |event| {
                        event.prevent_default();
                        if *is_submitting.read() {
                            return;
                        }
                        is_submitting.set(true);
                        error.set(None);
                        let password_value = password.read().clone();
                        let navigator = navigator.clone();
                        spawn(async move {
                            match api::admin::login(password_value).await {
                                Ok(_) => {
                                    navigator.push(Route::AdminLandingPage {});
                                }
                                Err(err) => error.set(Some(err.to_string())),
                            }
                            is_submitting.set(false);
                        });
                    },
                    label {
                        "Password"
                        input {
                            r#type: "password",
                            autocomplete: "current-password",
                            value: "{password}",
                            oninput: move |event| password.set(event.value()),
                        }
                    }
                    button {
                        class: "admin-primary-button",
                        r#type: "submit",
                        disabled: *is_submitting.read(),
                        if *is_submitting.read() {
                            "Signing in..."
                        } else {
                            "Sign in"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AdminLandingPage() -> Element {
    let navigator = use_navigator();
    let create_navigator = navigator.clone();
    let manage_navigator = navigator.clone();

    if !has_auth_token() {
        navigator.replace(Route::AdminLoginPage {});
    }

    rsx! {
        AdminFrame {
            title: "Forms".to_string(),
            div { class: "admin-action-grid",
                button {
                    class: "admin-action-card",
                    r#type: "button",
                    onclick: move |_| {
                        create_navigator.push(Route::AdminFormBuilderPage {});
                    },
                    strong { "Create a new form" }
                    span { "Build and publish an immutable form definition." }
                }
                button {
                    class: "admin-action-card",
                    r#type: "button",
                    onclick: move |_| {
                        manage_navigator.push(Route::AdminFormListPage {});
                    },
                    strong { "Manage existing forms" }
                    span { "Review forms, tokens, and submissions." }
                }
            }
        }
    }
}

#[component]
pub fn AdminFormListPage() -> Element {
    let forms = use_resource(move || async move { api::admin::fetch_forms().await });
    let navigator = use_navigator();
    let new_form_navigator = navigator.clone();

    rsx! {
        AdminFrame {
            title: "Form management".to_string(),
            div { class: "admin-page-actions",
                button {
                    class: "admin-primary-button",
                    r#type: "button",
                    onclick: move |_| {
                        new_form_navigator.push(Route::AdminFormBuilderPage {});
                    },
                    "New form"
                }
            }

            match forms.read().as_ref() {
                None => rsx! { p { class: "admin-muted", "Loading forms..." } },
                Some(Err(err)) => rsx! { AdminError { err: err.clone() } },
                Some(Ok(forms)) => rsx! {
                    table { class: "admin-table",
                        thead {
                            tr {
                                th { "Name" }
                                th { "Status" }
                                th { "Created" }
                                th { "Prepared by" }
                            }
                        }
                        tbody {
                            for form in forms.iter() {
                                tr {
                                    key: "{form.form_id}-{form.version}",
                                    onclick: {
                                        let form_id = form.form_id.clone();
                                        let version = form.version;
                                        let navigator = navigator.clone();
                                        move |_| {
                                            navigator.push(Route::AdminFormDetailPage { form_id: form_id.clone(), version });
                                        }
                                    },
                                    td { "{form.title}" }
                                    td { StatusPill { active: form.active } }
                                    td { "{form.created_at}" }
                                    td { "{form.created_by}" }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub fn AdminFormDetailPage(form_id: String, version: i32) -> Element {
    let detail_refresh = use_signal(|| 0_u32);
    let tokens_refresh = use_signal(|| 0_u32);
    let detail = use_resource({
        let form_id = form_id.clone();
        move || {
            let _ = *detail_refresh.read();
            let form_id = form_id.clone();
            async move { api::admin::fetch_form(&form_id, version).await }
        }
    });
    let tokens = use_resource({
        let form_id = form_id.clone();
        move || {
            let _ = *tokens_refresh.read();
            let form_id = form_id.clone();
            async move { api::admin::fetch_share_tokens(&form_id, version).await }
        }
    });
    let navigator = use_navigator();
    let submissions_navigator = navigator.clone();
    let submissions_form_id = form_id.clone();
    let mut notes = use_signal(String::new);
    let mut expires_at = use_signal(String::new);
    let mut new_token = use_signal(|| None::<String>);
    let mut action_error = use_signal(|| None::<String>);

    rsx! {
        AdminFrame {
            title: "Form detail".to_string(),
            div { class: "admin-page-actions",
                button {
                    class: "admin-secondary-button",
                    r#type: "button",
                    onclick: move |_| {
                        submissions_navigator.push(Route::AdminSubmissionListPage { form_id: submissions_form_id.clone(), version });
                    },
                    "Submissions"
                }
            }

            if let Some(message) = action_error.read().as_ref() {
                p { class: "admin-error", "{message}" }
            }

            match detail.read().as_ref() {
                None => rsx! { p { class: "admin-muted", "Loading form..." } },
                Some(Err(err)) => rsx! { AdminError { err: err.clone() } },
                Some(Ok(form_detail)) => rsx! {
                    section { class: "admin-panel",
                        div { class: "admin-detail-header",
                            div {
                                h2 { "{form_detail.form.title}" }
                                p { class: "admin-muted", "Version {form_detail.form.version} prepared by {form_detail.form.meta.created_by}" }
                            }
                            StatusPill { active: form_detail.active }
                        }
                        p { class: "admin-notice", "This form is immutable. Create a new form to make changes." }
                        button {
                            class: "admin-secondary-button",
                            r#type: "button",
                            onclick: {
                                let form_id = form_id.clone();
                                let active = !form_detail.active;
                                move |_| {
                                    let form_id = form_id.clone();
                                    spawn(async move {
                                        if let Err(err) = api::admin::set_form_active(&form_id, version, active).await {
                                            action_error.set(Some(err.to_string()));
                                        } else {
                                            action_error.set(None);
                                            increment_signal(detail_refresh);
                                        }
                                    });
                                }
                            },
                            if form_detail.active { "Deactivate form" } else { "Activate form" }
                        }
                    }

                    ReadOnlyFormDefinition { form: form_detail.form.clone() }
                },
            }

            section { class: "admin-panel",
                h2 { "Share tokens" }
                if let Some(token) = new_token.read().as_ref() {
                    div { class: "admin-token-result",
                        strong { "New token" }
                        code { "{token}" }
                    }
                }
                div { class: "admin-token-form",
                    label {
                        "Notes"
                        input {
                            value: "{notes}",
                            oninput: move |event| notes.set(event.value()),
                        }
                    }
                    label {
                        "Expiry override"
                        input {
                            r#type: "datetime-local",
                            value: "{expires_at}",
                            oninput: move |event| expires_at.set(event.value()),
                        }
                    }
                    button {
                        class: "admin-primary-button",
                        r#type: "button",
                        onclick: {
                            let form_id = form_id.clone();
                            move |_| {
                                let form_id = form_id.clone();
                                let notes_value = optional_string(notes.read().clone());
                                let expires_value = optional_datetime(expires_at.read().clone());
                                spawn(async move {
                                    let req = api::admin::CreateShareTokenRequest {
                                        notes: notes_value,
                                        expires_at: expires_value,
                                    };
                                    match api::admin::create_share_token(&form_id, version, &req).await {
                                        Ok(response) => {
                                            new_token.set(Some(response.token));
                                            action_error.set(None);
                                            increment_signal(tokens_refresh);
                                        }
                                        Err(err) => action_error.set(Some(err.to_string())),
                                    }
                                });
                            }
                        },
                        "Generate token"
                    }
                }

                match tokens.read().as_ref() {
                    None => rsx! { p { class: "admin-muted", "Loading tokens..." } },
                    Some(Err(err)) => rsx! { AdminError { err: err.clone() } },
                    Some(Ok(tokens_list)) => rsx! {
                        table { class: "admin-table",
                            thead {
                                tr {
                                    th { "Prefix" }
                                    th { "Status" }
                                    th { "Expires" }
                                    th { "Notes" }
                                    th { "Action" }
                                }
                            }
                            tbody {
                                for token in tokens_list.iter() {
                                    tr { key: "{token.share_token_id}",
                                        td { "{token.token_prefix.clone().unwrap_or_else(|| \"-\".to_string())}" }
                                        td { StatusPill { active: token.active && token.used_at.is_none() } }
                                        td { "{token.expires_at.clone().unwrap_or_else(|| \"No expiry\".to_string())}" }
                                        td { "{token.notes.clone().unwrap_or_default()}" }
                                        td {
                                            button {
                                                class: "admin-secondary-button",
                                                r#type: "button",
                                                disabled: !token.active,
                                                onclick: {
                                                    let share_token_id = token.share_token_id.clone();
                                                    move |_| {
                                                        let share_token_id = share_token_id.clone();
                                                        spawn(async move {
                                                            match api::admin::deactivate_share_token(&share_token_id).await {
                                                                Ok(_) => {
                                                                    action_error.set(None);
                                                                    increment_signal(tokens_refresh);
                                                                }
                                                                Err(err) => action_error.set(Some(err.to_string())),
                                                            }
                                                        });
                                                    }
                                                },
                                                "Deactivate"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn AdminSubmissionListPage(form_id: String, version: i32) -> Element {
    let submissions = use_resource({
        let form_id = form_id.clone();
        move || {
            let form_id = form_id.clone();
            async move { api::admin::fetch_submissions(&form_id, version).await }
        }
    });
    let navigator = use_navigator();

    rsx! {
        AdminFrame {
            title: "Submissions".to_string(),
            match submissions.read().as_ref() {
                None => rsx! { p { class: "admin-muted", "Loading submissions..." } },
                Some(Err(err)) => rsx! { AdminError { err: err.clone() } },
                Some(Ok(items)) => rsx! {
                    table { class: "admin-table",
                        thead {
                            tr {
                                th { "Submitted" }
                                th { "Company" }
                                th { "Signer" }
                                th { "Flag" }
                            }
                        }
                        tbody {
                            for item in items.iter() {
                                tr {
                                    key: "{item.completed_form_id}",
                                    onclick: {
                                        let completed_form_id = item.completed_form_id.clone();
                                        let form_id = form_id.clone();
                                        let navigator = navigator.clone();
                                        move |_| {
                                            navigator.push(Route::AdminSubmissionDetailPage {
                                                form_id: form_id.clone(),
                                                version,
                                                completed_form_id: completed_form_id.clone(),
                                            });
                                        }
                                    },
                                    td { "{item.submitted_at}" }
                                    td { "{item.company_name}" }
                                    td { "{item.signer_name} ({item.signer_title})" }
                                    td {
                                        if item.has_negative_confirmation {
                                            span { class: "admin-flag admin-flag--negative", "Action needed" }
                                        } else {
                                            span { class: "admin-flag", "Clear" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub fn AdminSubmissionDetailPage(
    form_id: String,
    version: i32,
    completed_form_id: String,
) -> Element {
    let navigator = use_navigator();
    let submission = use_resource({
        let completed_form_id = completed_form_id.clone();
        move || {
            let completed_form_id = completed_form_id.clone();
            async move { api::admin::fetch_submission(&completed_form_id).await }
        }
    });

    rsx! {
        AdminFrame {
            title: "Submission detail".to_string(),
            div { class: "admin-page-actions",
                button {
                    class: "admin-secondary-button",
                    r#type: "button",
                    onclick: move |_| {
                        navigator.push(Route::AdminSubmissionListPage {
                            form_id: form_id.clone(),
                            version,
                        });
                    },
                    "Back to submissions"
                }
            }
            match submission.read().as_ref() {
                None => rsx! { p { class: "admin-muted", "Loading submission..." } },
                Some(Err(err)) => rsx! { AdminError { err: err.clone() } },
                Some(Ok(completed_form)) => rsx! {
                    NegativeConfirmationActions { completed_form: completed_form.clone() }
                    CompletedFormViewer { completed_form: completed_form.clone() }
                },
            }
        }
    }
}

#[component]
pub fn AdminFormBuilderPage() -> Element {
    let navigator = use_navigator();
    let draft = use_signal(BuilderDraft::default);
    let mut step = use_signal(|| BuilderStep::Meta);
    let mut error = use_signal(|| None::<String>);
    let mut is_saving = use_signal(|| false);

    rsx! {
        AdminFrame {
            title: "Form builder".to_string(),
            div { class: "admin-step-tabs",
                StepButton { label: "Details", active: *step.read() == BuilderStep::Meta, onclick: move |_| step.set(BuilderStep::Meta) }
                StepButton { label: "Sections", active: *step.read() == BuilderStep::Sections, onclick: move |_| step.set(BuilderStep::Sections) }
                StepButton { label: "Preview", active: *step.read() == BuilderStep::Preview, onclick: move |_| step.set(BuilderStep::Preview) }
            }

            if let Some(message) = error.read().as_ref() {
                p { class: "admin-error", "{message}" }
            }

            match *step.read() {
                BuilderStep::Meta => rsx! { BuilderMetaStep { draft } },
                BuilderStep::Sections => rsx! { BuilderSectionsStep { draft } },
                BuilderStep::Preview => rsx! { BuilderPreviewStep { draft: draft.read().clone() } },
            }

            div { class: "admin-builder-actions",
                button {
                    class: "admin-secondary-button",
                    r#type: "button",
                    onclick: move |_| {
                        let next = {
                            let current = step.read();
                            previous_step(*current)
                        };
                        step.set(next);
                    },
                    disabled: *step.read() == BuilderStep::Meta,
                    "Back"
                }
                button {
                    class: "admin-secondary-button",
                    r#type: "button",
                    onclick: move |_| {
                        let next = {
                            let current = step.read();
                            next_step(*current)
                        };
                        step.set(next);
                    },
                    disabled: *step.read() == BuilderStep::Preview,
                    "Next"
                }
                button {
                    class: "admin-primary-button",
                    r#type: "button",
                    disabled: *is_saving.read(),
                    onclick: move |_| {
                        let current = draft.read().clone();
                        if let Err(message) = validate_builder_draft(&current) {
                            error.set(Some(message));
                            return;
                        }
                        error.set(None);
                        is_saving.set(true);
                        let navigator = navigator.clone();
                        spawn(async move {
                            let req = api::admin::CreateFormRequest {
                                title: current.title,
                                description_markdown: optional_string(current.description_markdown),
                                created_by: current.created_by,
                                sections: current.sections,
                            };
                            match api::admin::create_form(&req).await {
                                Ok(detail) => {
                                    navigator.push(Route::AdminFormDetailPage {
                                        form_id: detail.form.form_id,
                                        version: detail.form.version as i32,
                                    });
                                }
                                Err(err) => error.set(Some(err.to_string())),
                            }
                            is_saving.set(false);
                        });
                    },
                    if *is_saving.read() { "Saving..." } else { "Save form" }
                }
            }
        }
    }
}

#[component]
fn AdminFrame(title: String, children: Element) -> Element {
    let navigator = use_navigator();
    let home_navigator = navigator.clone();
    let forms_navigator = navigator.clone();
    let new_navigator = navigator.clone();
    let logout_navigator = navigator.clone();

    rsx! {
        main { class: "admin-shell",
            header { class: "admin-topbar",
                button {
                    class: "admin-link-button",
                    r#type: "button",
                    onclick: move |_| {
                        home_navigator.push(Route::AdminLandingPage {});
                    },
                    "Admin"
                }
                nav {
                    button {
                        class: "admin-link-button",
                        r#type: "button",
                        onclick: move |_| {
                            forms_navigator.push(Route::AdminFormListPage {});
                        },
                        "Forms"
                    }
                    button {
                        class: "admin-link-button",
                        r#type: "button",
                        onclick: move |_| {
                            new_navigator.push(Route::AdminFormBuilderPage {});
                        },
                        "New"
                    }
                    button {
                        class: "admin-link-button",
                        r#type: "button",
                        onclick: move |_| {
                            let navigator = logout_navigator.clone();
                            spawn(async move {
                                let _ = api::admin::logout().await;
                                clear_auth_token();
                                navigator.replace(Route::AdminLoginPage {});
                            });
                        },
                        "Logout"
                    }
                }
            }
            section { class: "admin-content",
                h1 { "{title}" }
                {children}
            }
        }
    }
}

#[component]
fn AdminError(err: ApiError) -> Element {
    rsx! {
        p { class: "admin-error", "{err}" }
    }
}

#[component]
fn StatusPill(active: bool) -> Element {
    let class = if active { "admin-status admin-status--active" } else { "admin-status" };
    rsx! {
        span { class, if active { "Active" } else { "Inactive" } }
    }
}

#[component]
fn StepButton(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let class = if active { "admin-step-tab is-active" } else { "admin-step-tab" };
    rsx! {
        button { class, r#type: "button", onclick: move |event| onclick.call(event), "{label}" }
    }
}

#[component]
fn BuilderMetaStep(draft: Signal<BuilderDraft>) -> Element {
    let current = draft.read().clone();
    let description_markdown = current.description_markdown.clone();

    rsx! {
        section { class: "admin-panel admin-form-stack",
            label {
                "Name"
                input {
                    value: "{current.title}",
                    oninput: move |event| draft.write().title = event.value(),
                }
            }
            label {
                "Prepared by"
                input {
                    value: "{current.created_by}",
                    oninput: move |event| draft.write().created_by = event.value(),
                }
            }
            div { class: "admin-split-editor",
                label {
                    "Description Markdown"
                    textarea {
                        value: "{current.description_markdown}",
                        oninput: move |event| draft.write().description_markdown = event.value(),
                    }
                }
                div { class: "admin-markdown-preview",
                    MarkdownDescription { markdown: description_markdown }
                }
            }
        }
    }
}

#[component]
fn BuilderSectionsStep(draft: Signal<BuilderDraft>) -> Element {
    let current = draft.read().clone();

    rsx! {
        section { class: "admin-panel",
            div { class: "admin-page-actions",
                button {
                    class: "admin-primary-button",
                    r#type: "button",
                    onclick: move |_| draft.write().add_section(),
                    "Add section"
                }
            }

            for (section_index, section) in current.sections.iter().enumerate() {
                article { class: "admin-builder-section", key: "{section.section_id}",
                    div { class: "admin-detail-header",
                        h2 { "Section {section_index + 1}" }
                        div { class: "admin-inline-actions",
                            button { class: "admin-icon-button", r#type: "button", onclick: move |_| draft.write().move_section(section_index, -1), "Up" }
                            button { class: "admin-icon-button", r#type: "button", onclick: move |_| draft.write().move_section(section_index, 1), "Down" }
                            button { class: "admin-icon-button", r#type: "button", onclick: move |_| draft.write().remove_section(section_index), "Remove" }
                        }
                    }
                    label {
                        "Title"
                        input {
                            value: "{section.title}",
                            oninput: move |event| draft.write().sections[section_index].title = event.value(),
                        }
                    }
                    div { class: "admin-split-editor",
                        label {
                            "Description Markdown"
                            textarea {
                                value: "{section.description_markdown.clone().unwrap_or_default()}",
                                oninput: move |event| draft.write().sections[section_index].description_markdown = optional_string(event.value()),
                            }
                        }
                        div { class: "admin-markdown-preview",
                            MarkdownDescription { markdown: section.description_markdown.clone().unwrap_or_default() }
                        }
                    }
                    div { class: "admin-page-actions",
                        button {
                            class: "admin-secondary-button",
                            r#type: "button",
                            onclick: move |_| draft.write().add_question(section_index),
                            "Add field"
                        }
                    }
                    for (question_index, question) in section.questions.iter().enumerate() {
                        BuilderQuestionEditor {
                            draft,
                            section_index,
                            question_index,
                            question: question.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BuilderQuestionEditor(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    question: Question,
) -> Element {
    rsx! {
        article { class: "admin-builder-question",
            div { class: "admin-detail-header",
                h3 { "Field {question_index + 1}" }
                div { class: "admin-inline-actions",
                    button { class: "admin-icon-button", r#type: "button", onclick: move |_| draft.write().move_question(section_index, question_index, -1), "Up" }
                    button { class: "admin-icon-button", r#type: "button", onclick: move |_| draft.write().move_question(section_index, question_index, 1), "Down" }
                    button { class: "admin-icon-button", r#type: "button", onclick: move |_| draft.write().remove_question(section_index, question_index), "Remove" }
                }
            }
            label {
                "Label"
                input {
                    value: "{question.title}",
                    oninput: move |event| draft.write().sections[section_index].questions[question_index].title = event.value(),
                }
            }
            label { class: "admin-checkbox-row",
                input {
                    r#type: "checkbox",
                    checked: question.required,
                    onchange: move |_| {
                        let current = draft.read().sections[section_index].questions[question_index].required;
                        draft.write().sections[section_index].questions[question_index].required = !current;
                    },
                }
                "Required"
            }
            label {
                "Field type"
                select {
                    value: "{question_kind_value(&question.kind)}",
                    onchange: move |event| {
                        draft.write().sections[section_index].questions[question_index].kind =
                            default_kind_for_value(&event.value());
                    },
                    option { value: "validation", "Confirmation" }
                    option { value: "text", "Text" }
                    option { value: "choice", "Choice" }
                    option { value: "multi_choice", "Multi choice" }
                }
            }
            BuilderQuestionKindEditor {
                draft,
                section_index,
                question_index,
                kind: question.kind.clone(),
            }
        }
    }
}

#[component]
fn BuilderQuestionKindEditor(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    kind: QuestionKind,
) -> Element {
    match kind {
        QuestionKind::Validation {
            description_markdown,
            confirm_prompt,
            summary_item,
        } => rsx! {
            label {
                "Description Markdown"
                textarea {
                    value: "{description_markdown}",
                    oninput: move |event| {
                        if let QuestionKind::Validation { description_markdown, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind {
                            *description_markdown = event.value();
                        }
                    },
                }
            }
            label {
                "Confirmation prompt"
                input {
                    value: "{confirm_prompt}",
                    oninput: move |event| {
                        if let QuestionKind::Validation { confirm_prompt, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind {
                            *confirm_prompt = event.value();
                        }
                    },
                }
            }
            label {
                "Summary item"
                input {
                    value: "{summary_item}",
                    oninput: move |event| {
                        if let QuestionKind::Validation { summary_item, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind {
                            *summary_item = event.value();
                        }
                    },
                }
            }
        },
        QuestionKind::Text {
            description_markdown,
            placeholder,
            multiline,
            max_length,
        } => rsx! {
            label {
                "Description Markdown"
                textarea {
                    value: "{description_markdown.clone().unwrap_or_default()}",
                    oninput: move |event| {
                        if let QuestionKind::Text { description_markdown, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind {
                            *description_markdown = optional_string(event.value());
                        }
                    },
                }
            }
            label {
                "Placeholder"
                input {
                    value: "{placeholder.clone().unwrap_or_default()}",
                    oninput: move |event| {
                        if let QuestionKind::Text { placeholder, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind {
                            *placeholder = optional_string(event.value());
                        }
                    },
                }
            }
            label { class: "admin-checkbox-row",
                input {
                    r#type: "checkbox",
                    checked: multiline,
                    onchange: move |_| {
                        if let QuestionKind::Text { multiline, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind {
                            *multiline = !*multiline;
                        }
                    },
                }
                "Multiline"
            }
            label {
                "Max length"
                input {
                    r#type: "number",
                    value: "{max_length.map(|value| value.to_string()).unwrap_or_default()}",
                    oninput: move |event| {
                        if let QuestionKind::Text { max_length, .. } =
                            &mut draft.write().sections[section_index].questions[question_index].kind {
                            *max_length = event.value().parse::<usize>().ok();
                        }
                    },
                }
            }
        },
        QuestionKind::Choice {
            description_markdown,
            options,
            allow_comment,
        } => rsx! {
            ChoiceOptionsEditor {
                draft,
                section_index,
                question_index,
                description_markdown,
                options,
                allow_comment,
                multi: false,
            }
        },
        QuestionKind::MultiChoice {
            description_markdown,
            options,
            min_selected,
            max_selected,
            allow_comment,
        } => rsx! {
            ChoiceOptionsEditor {
                draft,
                section_index,
                question_index,
                description_markdown,
                options,
                allow_comment,
                multi: true,
            }
            div { class: "admin-two-col",
                label {
                    "Minimum selected"
                    input {
                        r#type: "number",
                        value: "{min_selected.map(|value| value.to_string()).unwrap_or_default()}",
                        oninput: move |event| {
                            if let QuestionKind::MultiChoice { min_selected, .. } =
                                &mut draft.write().sections[section_index].questions[question_index].kind {
                                *min_selected = event.value().parse::<usize>().ok();
                            }
                        },
                    }
                }
                label {
                    "Maximum selected"
                    input {
                        r#type: "number",
                        value: "{max_selected.map(|value| value.to_string()).unwrap_or_default()}",
                        oninput: move |event| {
                            if let QuestionKind::MultiChoice { max_selected, .. } =
                                &mut draft.write().sections[section_index].questions[question_index].kind {
                                *max_selected = event.value().parse::<usize>().ok();
                            }
                        },
                    }
                }
            }
        },
    }
}

#[component]
fn ChoiceOptionsEditor(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    description_markdown: Option<String>,
    options: Vec<QuestionOption>,
    allow_comment: bool,
    multi: bool,
) -> Element {
    let description_markdown_value = description_markdown.clone().unwrap_or_default();

    rsx! {
        label {
            "Description Markdown"
            textarea {
                value: "{description_markdown_value}",
                oninput: move |event| {
                    update_choice_kind(draft, section_index, question_index, |description, _, _| {
                        *description = optional_string(event.value());
                    });
                },
            }
        }
        label { class: "admin-checkbox-row",
            input {
                r#type: "checkbox",
                checked: allow_comment,
                onchange: move |_| {
                    update_choice_kind(draft, section_index, question_index, |_, _, allow_comment| {
                        *allow_comment = !*allow_comment;
                    });
                },
            }
            "Allow comment"
        }
        div { class: "admin-option-list",
            for (option_index, option) in options.iter().enumerate() {
                div { class: "admin-option-row", key: "{option.question_option_id}",
                    input {
                        value: "{option.label}",
                        oninput: move |event| {
                            update_choice_options(draft, section_index, question_index, |options| {
                                options[option_index].label = event.value();
                            });
                        },
                    }
                    input {
                        value: "{option.description.clone().unwrap_or_default()}",
                        placeholder: "Description",
                        oninput: move |event| {
                            update_choice_options(draft, section_index, question_index, |options| {
                                options[option_index].description = optional_string(event.value());
                            });
                        },
                    }
                    button {
                        class: "admin-icon-button",
                        r#type: "button",
                        onclick: move |_| update_choice_options(draft, section_index, question_index, |options| {
                            if options.len() > 1 {
                                options.remove(option_index);
                            }
                        }),
                        "Remove"
                    }
                }
            }
        }
        button {
            class: "admin-secondary-button",
            r#type: "button",
            onclick: move |_| update_choice_options(draft, section_index, question_index, |options| {
                options.push(default_option(options.len()));
            }),
            if multi { "Add multi-choice option" } else { "Add choice option" }
        }
    }
}

#[component]
fn BuilderPreviewStep(draft: BuilderDraft) -> Element {
    rsx! {
        section { class: "admin-panel",
            ReadOnlyFormDefinition { form: draft.preview_form() }
        }
    }
}

#[component]
fn ReadOnlyFormDefinition(form: Form) -> Element {
    rsx! {
        div { class: "admin-readonly-form",
            header {
                h2 { "{form.title}" }
                if let Some(description) = form.description_markdown.as_ref() {
                    MarkdownDescription { markdown: description.clone() }
                }
            }
            for section in form.sections.iter() {
                section { class: "admin-readonly-section", key: "{section.section_id}",
                    h3 { "{section.number}. {section.title}" }
                    if let Some(description) = section.description_markdown.as_ref() {
                        MarkdownDescription { markdown: description.clone() }
                    }
                    for question in section.questions.iter() {
                        article { class: "admin-readonly-question", key: "{question.question_id}",
                            h4 { "{question.number}. {question.title}" }
                            if question.required {
                                span { class: "admin-status admin-status--active", "Required" }
                            }
                            QuestionKindSummary { question: question.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QuestionKindSummary(question: Question) -> Element {
    match question.kind {
        QuestionKind::Validation {
            description_markdown,
            confirm_prompt,
            summary_item,
        } => rsx! {
            MarkdownDescription { markdown: description_markdown }
            p { strong { "Prompt: " } "{confirm_prompt}" }
            p { strong { "Summary: " } "{summary_item}" }
        },
        QuestionKind::Text {
            description_markdown,
            placeholder,
            multiline,
            max_length,
        } => {
            let field_label = if multiline {
                "Text field (multiline)"
            } else {
                "Text field"
            };

            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{field_label}" }
                if let Some(placeholder) = placeholder {
                    p { strong { "Placeholder: " } "{placeholder}" }
                }
                if let Some(max_length) = max_length {
                    p { strong { "Max length: " } "{max_length}" }
                }
            }
        },
        QuestionKind::Choice {
            description_markdown,
            options,
            allow_comment,
        } => {
            let choice_label = if allow_comment {
                "Single choice with comment"
            } else {
                "Single choice"
            };

            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{choice_label}" }
                OptionList { options }
            }
        },
        QuestionKind::MultiChoice {
            description_markdown,
            options,
            min_selected,
            max_selected,
            allow_comment,
        } => {
            let choice_label = if allow_comment {
                "Multi choice with comment"
            } else {
                "Multi choice"
            };
            let min_label = min_selected
                .map(|value| value.to_string())
                .unwrap_or_else(|| "0".to_string());
            let max_label = max_selected
                .map(|value| value.to_string())
                .unwrap_or_else(|| "any".to_string());

            rsx! {
                if let Some(description) = description_markdown {
                    MarkdownDescription { markdown: description }
                }
                p { "{choice_label}" }
                if min_selected.is_some() || max_selected.is_some() {
                    p { "Selection range: {min_label} to {max_label}" }
                }
                OptionList { options }
            }
        },
    }
}

#[component]
fn OptionList(options: Vec<QuestionOption>) -> Element {
    rsx! {
        ul {
            for option in options.iter() {
                li { key: "{option.question_option_id}", "{option.label}" }
            }
        }
    }
}

#[component]
fn NegativeConfirmationActions(completed_form: CompletedForm) -> Element {
    let actions: Vec<_> = completed_form
        .responses
        .iter()
        .filter_map(|response| match &response.response {
            Response::Validation {
                status: ValidationStatus::NotCorrect,
                comment,
            } => Some((response.question_id.clone(), comment.clone())),
            _ => None,
        })
        .collect();

    if actions.is_empty() {
        return rsx! {};
    }

    rsx! {
        section { class: "admin-panel admin-action-items",
            h2 { "Action items" }
            for (question_id, comment) in actions.iter() {
                article { class: "admin-action-item", key: "{question_id}",
                    strong { "{question_title(&completed_form.form, question_id)}" }
                    p { "{comment.clone().unwrap_or_else(|| \"No feedback provided.\".to_string())}" }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BuilderStep {
    Meta,
    Sections,
    Preview,
}

#[derive(Clone, Debug, PartialEq)]
struct BuilderDraft {
    title: String,
    description_markdown: String,
    created_by: String,
    sections: Vec<Section>,
}

impl Default for BuilderDraft {
    fn default() -> Self {
        Self {
            title: String::new(),
            description_markdown: String::new(),
            created_by: String::new(),
            sections: vec![default_section(0)],
        }
    }
}

impl BuilderDraft {
    fn add_section(&mut self) {
        self.sections.push(default_section(self.sections.len()));
        self.renumber();
    }

    fn remove_section(&mut self, index: usize) {
        if self.sections.len() > 1 && index < self.sections.len() {
            self.sections.remove(index);
            self.renumber();
        }
    }

    fn move_section(&mut self, index: usize, direction: isize) {
        let new_index = shifted_index(index, direction, self.sections.len());
        if new_index != index {
            self.sections.swap(index, new_index);
            self.renumber();
        }
    }

    fn add_question(&mut self, section_index: usize) {
        if let Some(section) = self.sections.get_mut(section_index) {
            section.questions.push(default_question(section.questions.len()));
            self.renumber();
        }
    }

    fn remove_question(&mut self, section_index: usize, question_index: usize) {
        if let Some(section) = self.sections.get_mut(section_index) {
            if section.questions.len() > 1 && question_index < section.questions.len() {
                section.questions.remove(question_index);
                self.renumber();
            }
        }
    }

    fn move_question(&mut self, section_index: usize, question_index: usize, direction: isize) {
        if let Some(section) = self.sections.get_mut(section_index) {
            let new_index = shifted_index(question_index, direction, section.questions.len());
            if new_index != question_index {
                section.questions.swap(question_index, new_index);
                self.renumber();
            }
        }
    }

    fn renumber(&mut self) {
        for (section_index, section) in self.sections.iter_mut().enumerate() {
            section.number = (section_index + 1) as u32;
            for (question_index, question) in section.questions.iter_mut().enumerate() {
                question.number = (question_index + 1) as u32;
            }
        }
    }

    fn preview_form(&self) -> Form {
        Form {
            form_id: "new-form".to_string(),
            version: 1,
            title: display_or_untitled(&self.title),
            description_markdown: optional_string(self.description_markdown.clone()),
            meta: FormMeta {
                created_at: String::new(),
                created_by: self.created_by.clone(),
                updated_at: None,
                updated_by: None,
            },
            sections: self.sections.clone(),
        }
    }
}

fn default_section(index: usize) -> Section {
    Section {
        section_id: generated_id("section"),
        number: (index + 1) as u32,
        title: String::new(),
        description_markdown: None,
        questions: vec![default_question(0)],
    }
}

fn default_question(index: usize) -> Question {
    Question {
        question_id: generated_id("question"),
        number: (index + 1) as u32,
        title: String::new(),
        required: true,
        kind: default_kind_for_value("text"),
    }
}

fn default_kind_for_value(value: &str) -> QuestionKind {
    match value {
        "validation" => QuestionKind::Validation {
            description_markdown: String::new(),
            confirm_prompt: String::new(),
            summary_item: String::new(),
        },
        "choice" => QuestionKind::Choice {
            description_markdown: None,
            options: vec![default_option(0), default_option(1)],
            allow_comment: false,
        },
        "multi_choice" => QuestionKind::MultiChoice {
            description_markdown: None,
            options: vec![default_option(0), default_option(1)],
            min_selected: None,
            max_selected: None,
            allow_comment: false,
        },
        _ => QuestionKind::Text {
            description_markdown: None,
            placeholder: None,
            multiline: false,
            max_length: None,
        },
    }
}

fn default_option(index: usize) -> QuestionOption {
    QuestionOption {
        question_option_id: generated_id("option"),
        label: format!("Option {}", index + 1),
        description: None,
    }
}

fn generated_id(prefix: &str) -> String {
    let random = js_sys::Math::random().to_string().replace("0.", "");
    format!("{prefix}-{random}")
}

fn question_kind_value(kind: &QuestionKind) -> &'static str {
    match kind {
        QuestionKind::Validation { .. } => "validation",
        QuestionKind::Text { .. } => "text",
        QuestionKind::Choice { .. } => "choice",
        QuestionKind::MultiChoice { .. } => "multi_choice",
    }
}

fn shifted_index(index: usize, direction: isize, len: usize) -> usize {
    if len == 0 {
        return index;
    }

    let shifted = index as isize + direction;
    shifted.clamp(0, len.saturating_sub(1) as isize) as usize
}

fn update_choice_kind(
    mut draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    update: impl FnOnce(&mut Option<String>, &mut Vec<QuestionOption>, &mut bool),
) {
    let mut draft = draft.write();
    let kind = &mut draft.sections[section_index].questions[question_index].kind;
    match kind {
        QuestionKind::Choice {
            description_markdown,
            options,
            allow_comment,
        }
        | QuestionKind::MultiChoice {
            description_markdown,
            options,
            allow_comment,
            ..
        } => update(description_markdown, options, allow_comment),
        _ => {}
    }
}

fn update_choice_options(
    draft: Signal<BuilderDraft>,
    section_index: usize,
    question_index: usize,
    update: impl FnOnce(&mut Vec<QuestionOption>),
) {
    update_choice_kind(draft, section_index, question_index, |_, options, _| update(options));
}

fn increment_signal(mut signal: Signal<u32>) {
    let next = {
        let current = signal.read();
        *current + 1
    };
    signal.set(next);
}

fn validate_builder_draft(draft: &BuilderDraft) -> Result<(), String> {
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
                    return Err("Confirmation fields need description, prompt, and summary.".to_string());
                }
                QuestionKind::Choice { options, .. } | QuestionKind::MultiChoice { options, .. } => {
                    if options.iter().any(|option| option.label.trim().is_empty()) {
                        return Err("Choice options need labels.".to_string());
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn optional_string(value: String) -> Option<String> {
    let value = value.trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn optional_datetime(value: String) -> Option<String> {
    optional_string(value).map(|value| format!("{value}:00Z"))
}

fn display_or_untitled(value: &str) -> String {
    if value.trim().is_empty() {
        "Untitled form".to_string()
    } else {
        value.trim().to_string()
    }
}

fn previous_step(step: BuilderStep) -> BuilderStep {
    match step {
        BuilderStep::Meta => BuilderStep::Meta,
        BuilderStep::Sections => BuilderStep::Meta,
        BuilderStep::Preview => BuilderStep::Sections,
    }
}

fn next_step(step: BuilderStep) -> BuilderStep {
    match step {
        BuilderStep::Meta => BuilderStep::Sections,
        BuilderStep::Sections => BuilderStep::Preview,
        BuilderStep::Preview => BuilderStep::Preview,
    }
}

fn question_title(form: &Form, question_id: &str) -> String {
    form.sections
        .iter()
        .flat_map(|section| section.questions.iter())
        .find(|question| question.question_id == question_id)
        .map(|question| question.title.clone())
        .unwrap_or_else(|| question_id.to_string())
}
