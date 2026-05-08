use crate::api;
use crate::features::admin::form_builder::{BuilderDraft, ReadOnlyFormDefinition, BUILDER_PREFILL};
use crate::features::admin::shared::{
    increment_signal, optional_datetime, optional_string, AdminError, AdminFrame, StatusPill,
};
use crate::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn AdminFormDetailPage(form_id: String, version: i32) -> Element {
    let detail_refresh = use_signal(|| 0_u32);
    let tokens_refresh = use_signal(|| 0_u32);
    let submissions_refresh = use_signal(|| 0_u32);
    let viewer_tokens_refresh = use_signal(|| 0_u32);
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
    let submissions = use_resource({
        let form_id = form_id.clone();
        move || {
            let _ = *submissions_refresh.read();
            let form_id = form_id.clone();
            async move { api::admin::fetch_submissions(&form_id, version).await }
        }
    });
    let navigator = use_navigator();
    let submissions_navigator = navigator.clone();
    let submissions_form_id = form_id.clone();
    let mut share_notes = use_signal(String::new);
    let mut share_expires_at = use_signal(String::new);
    let mut generated_share_token = use_signal(|| None::<String>);
    let mut share_token_links = use_signal(HashMap::<String, String>::new);
    let mut show_share_token_modal = use_signal(|| false);
    let mut share_token_error = use_signal(|| None::<String>);
    let mut selected_viewer_submission = use_signal(|| None::<api::admin::SubmissionListItem>);
    let mut viewer_expires_at = use_signal(String::new);
    let mut generated_viewer_token = use_signal(|| None::<String>);
    let mut viewer_token_links = use_signal(HashMap::<String, String>::new);
    let mut viewer_token_error = use_signal(|| None::<String>);
    let mut action_error = use_signal(|| None::<String>);
    let mut show_clone_modal = use_signal(|| false);
    let mut clone_title = use_signal(String::new);
    let mut clone_created_by = use_signal(String::new);
    let mut clone_error = use_signal(|| None::<String>);
    let viewer_tokens = use_resource({
        move || {
            let _ = *viewer_tokens_refresh.read();
            let selected_submission = selected_viewer_submission.read().clone();
            async move {
                match selected_submission {
                    Some(submission) => {
                        api::admin::fetch_viewer_tokens(&submission.completed_form_id)
                            .await
                            .map(Some)
                    }
                    None => Ok(None),
                }
            }
        }
    });

    rsx! {
        AdminFrame { title: "Form detail".to_string(),
            div { class: "admin-page-actions",
                button {
                    class: "admin-secondary-button",
                    r#type: "button",
                    onclick: move |_| {
                        submissions_navigator
                            .push(Route::AdminSubmissionListPage {
                                form_id: submissions_form_id.clone(),
                                version,
                            });
                    },
                    "Submissions"
                }
            }

            if let Some(message) = action_error.read().as_ref() {
                p { class: "admin-error", "{message}" }
            }

            match detail.read().as_ref() {
                None => rsx! {
                    p { class: "admin-muted", "Loading form..." }
                },
                Some(Err(err)) => rsx! {
                    AdminError { err: err.clone() }
                },
                Some(Ok(form_detail)) => rsx! {
                    div { class: "admin-form-detail-layout",
                        section { class: "admin-panel admin-form-detail-summary",
                            div { class: "admin-detail-header",
                                div {
                                    h2 { "{form_detail.form.title}" }
                                    p { class: "admin-muted",
                                        "Version {form_detail.form.version} prepared by {form_detail.form.meta.created_by}"
                                    }
                                }
                                StatusPill { active: form_detail.active }
                            }
                            p { class: "admin-notice", "This form is immutable. Create a new form to make changes." }
                            div { class: "admin-inline-actions",
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
                                    if form_detail.active {
                                        "Deactivate form"
                                    } else {
                                        "Activate form"
                                    }
                                }
                                button {
                                    class: "admin-secondary-button",
                                    r#type: "button",
                                    onclick: {
                                        let current_title = form_detail.form.title.clone();
                                        move |_| {
                                            clone_title.set(current_title.clone());
                                            clone_created_by.set(String::new());
                                            clone_error.set(None);
                                            show_clone_modal.set(true);
                                        }
                                    },
                                    "Clone to new version"
                                }
                            }
                        }

                        div { class: "admin-form-detail-grid",
                            div { class: "admin-form-detail-main",
                                ReadOnlyFormDefinition { form: form_detail.form.clone() }
                            }
                            aside { class: "admin-form-detail-sidebar",
                                ShareTokensPanel {
                                    tokens_state: tokens.read().clone(),
                                    token_links: share_token_links.read().clone(),
                                    on_open_generate: move |_| {
                                        share_notes.set(String::new());
                                        share_expires_at.set(String::new());
                                        generated_share_token.set(None);
                                        share_token_error.set(None);
                                        show_share_token_modal.set(true);
                                    },
                                    on_deactivate: move |share_token_id: String| {
                                        spawn(async move {
                                            match api::admin::deactivate_share_token(&share_token_id).await {
                                                Ok(_) => {
                                                    action_error.set(None);
                                                    increment_signal(tokens_refresh);
                                                }
                                                Err(err) => action_error.set(Some(err.to_string())),
                                            }
                                        });
                                    },
                                }

                                ViewerTokensPanel {
                                    submissions_state: submissions.read().clone(),
                                    on_manage: move |submission: api::admin::SubmissionListItem| {
                                        selected_viewer_submission.set(Some(submission));
                                        viewer_expires_at.set(String::new());
                                        generated_viewer_token.set(None);
                                        viewer_token_error.set(None);
                                        increment_signal(viewer_tokens_refresh);
                                    },
                                }
                            }
                        }
                    }

                    if *show_clone_modal.read() {
                        {
                            let sections = form_detail.form.sections.clone();
                            let description = form_detail.form.description_markdown.clone();
                            rsx! {
                                CloneModal {
                                    title: clone_title,
                                    created_by: clone_created_by,
                                    error: clone_error,
                                    on_cancel: move |_| show_clone_modal.set(false),
                                    on_confirm: move |_| {
                                        let title = clone_title.read().trim().to_string();
                                        let created_by = clone_created_by.read().trim().to_string();
                                        if title.is_empty() || created_by.is_empty() {
                                            clone_error.set(Some("Title and prepared by are required.".to_string()));
                                            return;
                                        }
                                        let sections = sections.clone();
                                        let description = description.clone();
                                        *BUILDER_PREFILL.write() = Some(BuilderDraft {
                                            title,
                                            description_markdown: description.unwrap_or_default(),
                                            created_by,
                                            sections,
                                        });
                                        show_clone_modal.set(false);
                                        navigator.push(Route::AdminFormBuilderPage {});
                                    },
                                }
                            }
                        }
                    }
                },
            }

            if *show_share_token_modal.read() {
                ShareTokenModal {
                    notes: share_notes,
                    expires_at: share_expires_at,
                    generated_token: generated_share_token,
                                    error: share_token_error,
                                    on_cancel: move |_| show_share_token_modal.set(false),
                                    on_generate: {
                                        let form_id = form_id.clone();
                                        move |_| {
                            let form_id = form_id.clone();
                            let notes_value = optional_string(share_notes.read().clone());
                            let expires_value = optional_datetime(share_expires_at.read().clone());
                            spawn(async move {
                                let req = api::admin::CreateShareTokenRequest {
                                    notes: notes_value,
                                    expires_at: expires_value,
                                };
                                match api::admin::create_share_token(&form_id, version, &req).await {
                                    Ok(response) => {
                                        let href = public_url(&format!("/f/{}", response.token));
                                        share_token_links
                                            .write()
                                            .insert(response.share_token.share_token_id.clone(), href);
                                        generated_share_token.set(Some(response.token));
                                        share_token_error.set(None);
                                        action_error.set(None);
                                        increment_signal(tokens_refresh);
                                    }
                                    Err(err) => share_token_error.set(Some(err.to_string())),
                                }
                            });
                        }
                    },
                }
            }

            if let Some(submission) = selected_viewer_submission.read().clone() {
                ViewerTokenModal {
                    submission,
                    expires_at: viewer_expires_at,
                    generated_token: generated_viewer_token,
                    error: viewer_token_error,
                    tokens_state: viewer_tokens.read().clone(),
                    token_links: viewer_token_links.read().clone(),
                    on_cancel: move |_| selected_viewer_submission.set(None),
                    on_generate: move |_| {
                        let Some(submission) = selected_viewer_submission.read().clone() else {
                            return;
                        };
                        let expires_value = optional_datetime(viewer_expires_at.read().clone());
                        spawn(async move {
                            let req = api::admin::CreateViewerTokenRequest {
                                expires_at: expires_value,
                            };
                            match api::admin::create_viewer_token(&submission.completed_form_id, &req).await {
                                Ok(response) => {
                                    let href = public_url(&format!("/viewer/{}", response.token));
                                    viewer_token_links
                                        .write()
                                        .insert(response.viewer_token.viewer_token_id.clone(), href);
                                    generated_viewer_token.set(Some(response.token));
                                    viewer_token_error.set(None);
                                    action_error.set(None);
                                    increment_signal(viewer_tokens_refresh);
                                }
                                Err(err) => viewer_token_error.set(Some(err.to_string())),
                            }
                        });
                    },
                    on_deactivate: move |viewer_token_id: String| {
                        spawn(async move {
                            match api::admin::deactivate_viewer_token(&viewer_token_id).await {
                                Ok(_) => {
                                    viewer_token_error.set(None);
                                    increment_signal(viewer_tokens_refresh);
                                }
                                Err(err) => viewer_token_error.set(Some(err.to_string())),
                            }
                        });
                    },
                }
            }
        }
    }
}

#[component]
fn ShareTokensPanel(
    tokens_state: Option<crate::api::http::ApiResult<Vec<api::admin::ShareTokenItem>>>,
    token_links: HashMap<String, String>,
    on_open_generate: EventHandler<()>,
    on_deactivate: EventHandler<String>,
) -> Element {
    rsx! {
        section { class: "admin-panel admin-access-panel",
            div { class: "admin-panel-header",
                div {
                    h2 { "Share tokens" }
                    p { class: "admin-muted", "Respondent links for this form version." }
                }
                button {
                    class: "admin-primary-button",
                    r#type: "button",
                    onclick: move |_| on_open_generate.call(()),
                    "Generate"
                }
            }

            match tokens_state.as_ref() {
                None => rsx! {
                    p { class: "admin-muted", "Loading tokens..." }
                },
                Some(Err(err)) => rsx! {
                    AdminError { err: err.clone() }
                },
                Some(Ok(tokens_list)) if tokens_list.is_empty() => rsx! {
                    p { class: "admin-muted", "No share tokens have been generated." }
                },
                Some(Ok(tokens_list)) => rsx! {
                    div { class: "admin-token-card-list",
                        for token in tokens_list.iter() {
                            {
                                let token_title = token.token_prefix.clone().unwrap_or_else(|| "-".to_string());
                                let active = token.active && token.used_at.is_none();
                                let copy_href = token_links.get(&token.share_token_id).cloned();
                                rsx! {
                            article { class: "admin-token-card", key: "{token.share_token_id}",
                                div {
                                    TokenTitle {
                                        title: token_title,
                                        copy_href,
                                        active,
                                    }
                                    span { class: "admin-muted",
                                        "{token.expires_at.clone().unwrap_or_else(|| \"No expiry\".to_string())}"
                                    }
                                }
                                StatusPill { active }
                                if let Some(notes) = token.notes.as_ref() {
                                    p { "{notes}" }
                                }
                                button {
                                    class: "admin-secondary-button",
                                    r#type: "button",
                                    disabled: !token.active,
                                    onclick: {
                                        let share_token_id = token.share_token_id.clone();
                                        move |_| on_deactivate.call(share_token_id.clone())
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

#[component]
fn ViewerTokensPanel(
    submissions_state: Option<crate::api::http::ApiResult<Vec<api::admin::SubmissionListItem>>>,
    on_manage: EventHandler<api::admin::SubmissionListItem>,
) -> Element {
    rsx! {
        section { class: "admin-panel admin-access-panel",
            div { class: "admin-panel-header",
                div {
                    h2 { "Viewer tokens" }
                    p { class: "admin-muted", "Read-only links for completed submissions." }
                }
            }

            match submissions_state.as_ref() {
                None => rsx! {
                    p { class: "admin-muted", "Loading submissions..." }
                },
                Some(Err(err)) => rsx! {
                    AdminError { err: err.clone() }
                },
                Some(Ok(submissions)) if submissions.is_empty() => rsx! {
                    p { class: "admin-muted", "No submissions are available yet." }
                },
                Some(Ok(submissions)) => rsx! {
                    div { class: "admin-submission-token-list",
                        for submission in submissions.iter() {
                            article { class: "admin-submission-token-card", key: "{submission.completed_form_id}",
                                div {
                                    strong { "{submission.company_name}" }
                                    span { class: "admin-muted", "{submission.submitted_at}" }
                                    span { class: "admin-muted", "Signed by {submission.signer_name}" }
                                }
                                button {
                                    class: "admin-secondary-button",
                                    r#type: "button",
                                    onclick: {
                                        let submission = submission.clone();
                                        move |_| on_manage.call(submission.clone())
                                    },
                                    "Manage"
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
fn ShareTokenModal(
    notes: Signal<String>,
    expires_at: Signal<String>,
    generated_token: Signal<Option<String>>,
    error: Signal<Option<String>>,
    on_cancel: EventHandler<()>,
    on_generate: EventHandler<()>,
) -> Element {
    let generated = generated_token.read().clone();

    rsx! {
        div {
            class: "modal-backdrop",
            role: "dialog",
            aria_modal: "true",
            aria_label: "Generate share token",

            div { class: "modal",
                h2 { class: "modal-title", "Generate share token" }

                if let Some(token) = generated.as_ref() {
                    TokenLinkResult {
                        label: "Respondent link",
                        token: token.clone(),
                        href: public_url(&format!("/f/{token}")),
                    }
                    div { class: "modal-actions",
                        button {
                            class: "admin-primary-button",
                            r#type: "button",
                            onclick: move |_| on_cancel.call(()),
                            "Close"
                        }
                    }
                } else {
                    label { class: "modal-label",
                        "Notes"
                        input {
                            class: "modal-input",
                            value: "{notes}",
                            oninput: move |event| notes.set(event.value()),
                        }
                    }
                    label { class: "modal-label",
                        "Expiry override"
                        input {
                            class: "modal-input",
                            r#type: "datetime-local",
                            value: "{expires_at}",
                            oninput: move |event| expires_at.set(event.value()),
                        }
                    }

                    if let Some(err) = error.read().as_ref() {
                        p { class: "admin-error", "{err}" }
                    }

                    div { class: "modal-actions",
                        button {
                            class: "admin-secondary-button",
                            r#type: "button",
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                        button {
                            class: "admin-primary-button",
                            r#type: "button",
                            onclick: move |_| on_generate.call(()),
                            "Generate token"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ViewerTokenModal(
    submission: api::admin::SubmissionListItem,
    expires_at: Signal<String>,
    generated_token: Signal<Option<String>>,
    error: Signal<Option<String>>,
    tokens_state: Option<crate::api::http::ApiResult<Option<Vec<api::admin::ViewerTokenItem>>>>,
    token_links: HashMap<String, String>,
    on_cancel: EventHandler<()>,
    on_generate: EventHandler<()>,
    on_deactivate: EventHandler<String>,
) -> Element {
    let generated = generated_token.read().clone();

    rsx! {
        div {
            class: "modal-backdrop",
            role: "dialog",
            aria_modal: "true",
            aria_label: "Manage viewer tokens",

            div { class: "modal modal--wide",
                h2 { class: "modal-title", "Manage viewer tokens" }
                div { class: "admin-modal-summary",
                    strong { "{submission.company_name}" }
                    span { class: "admin-muted", "Submitted {submission.submitted_at}" }
                    span { class: "admin-muted", "Signed by {submission.signer_name}" }
                }

                if let Some(token) = generated.as_ref() {
                    TokenLinkResult {
                        label: "Viewer link",
                        token: token.clone(),
                        href: public_url(&format!("/viewer/{token}")),
                    }
                }

                label { class: "modal-label",
                    "New viewer link expiry"
                    input {
                        class: "modal-input",
                        r#type: "datetime-local",
                        value: "{expires_at}",
                        oninput: move |event| expires_at.set(event.value()),
                    }
                }

                if let Some(err) = error.read().as_ref() {
                    p { class: "admin-error", "{err}" }
                }

                div { class: "admin-token-modal-section",
                    h3 { "Existing viewer tokens" }
                    match tokens_state.as_ref() {
                        None => rsx! {
                            p { class: "admin-muted", "Loading viewer tokens..." }
                        },
                        Some(Err(err)) => rsx! {
                            AdminError { err: err.clone() }
                        },
                        Some(Ok(None)) => rsx! {
                            p { class: "admin-muted", "Select a submission to manage viewer tokens." }
                        },
                        Some(Ok(Some(tokens))) if tokens.is_empty() => rsx! {
                            p { class: "admin-muted", "No viewer tokens have been generated for this submission." }
                        },
                        Some(Ok(Some(tokens))) => rsx! {
                            table { class: "admin-table admin-table--static",
                                thead {
                                    tr {
                                        th { "Prefix" }
                                        th { "Status" }
                                        th { "Expires" }
                                        th { "Created" }
                                        th { "Action" }
                                    }
                                }
                                tbody {
                                    for token in tokens.iter() {
                                        {
                                            let token_title = token.token_prefix.clone().unwrap_or_else(|| "-".to_string());
                                            let copy_href = token_links.get(&token.viewer_token_id).cloned();
                                            rsx! {
                                                tr { key: "{token.viewer_token_id}",
                                                    td {
                                                        TokenTitle {
                                                            title: token_title,
                                                            copy_href,
                                                            active: token.active,
                                                        }
                                                    }
                                                    td {
                                                        StatusPill { active: token.active }
                                                    }
                                                    td { "{token.expires_at.clone().unwrap_or_else(|| \"No expiry\".to_string())}" }
                                                    td { "{token.created_at}" }
                                                    td {
                                                        button {
                                                            class: "admin-secondary-button",
                                                            r#type: "button",
                                                            disabled: !token.active,
                                                            onclick: {
                                                                let viewer_token_id = token.viewer_token_id.clone();
                                                                move |_| on_deactivate.call(viewer_token_id.clone())
                                                            },
                                                            "Deactivate"
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }

                div { class: "modal-actions",
                    button {
                        class: "admin-secondary-button",
                        r#type: "button",
                        onclick: move |_| on_cancel.call(()),
                        "Close"
                    }
                    button {
                        class: "admin-primary-button",
                        r#type: "button",
                        onclick: move |_| on_generate.call(()),
                        "Generate viewer link"
                    }
                }
            }
        }
    }
}

#[component]
fn TokenLinkResult(label: &'static str, token: String, href: String) -> Element {
    rsx! {
        div { class: "admin-token-result",
            TokenTitle {
                title: label.to_string(),
                copy_href: Some(href.clone()),
                active: true,
            }
            code { "{token}" }
            div { class: "admin-token-link",
                span { class: "admin-muted", "Link:" }
                a { href: "{href}", target: "_blank", "{href}" }
            }
        }
    }
}

#[component]
fn TokenTitle(title: String, copy_href: Option<String>, active: bool) -> Element {
    if active {
        if let Some(href) = copy_href {
            return rsx! {
                button {
                    class: "admin-token-title-button",
                    r#type: "button",
                    title: "Copy token link",
                    aria_label: "Copy token link",
                    onclick: move |_| copy_to_clipboard(&href),
                    "{title}"
                }
            };
        }
    }

    rsx! {
        strong { "{title}" }
    }
}

#[component]
fn CloneModal(
    title: Signal<String>,
    created_by: Signal<String>,
    error: Signal<Option<String>>,
    on_cancel: EventHandler<()>,
    on_confirm: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "modal-backdrop",
            role: "dialog",
            aria_modal: "true",
            aria_label: "Clone to new version",

            div { class: "modal",
                div {
                    h2 { class: "modal-title", "Clone to new version" }
                    p { class: "admin-muted",
                        "Opens the form builder pre-filled with all existing questions. You can update the title and set who prepared this version."
                    }
                }

                div { class: "admin-modal-fields",
                    label { class: "modal-label",
                        "Title"
                        input {
                            id: "clone-title",
                            class: "modal-input",
                            r#type: "text",
                            value: "{title}",
                            oninput: move |e| title.set(e.value()),
                        }
                    }
                    label { class: "modal-label",
                        "Prepared by"
                        input {
                            id: "clone-created-by",
                            class: "modal-input",
                            r#type: "text",
                            placeholder: "Name or team",
                            value: "{created_by}",
                            oninput: move |e| created_by.set(e.value()),
                        }
                    }
                }

                if let Some(err) = error.read().as_ref() {
                    p { class: "admin-error", "{err}" }
                }

                div { class: "modal-actions",
                    button {
                        class: "admin-secondary-button",
                        r#type: "button",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        class: "admin-primary-button",
                        r#type: "button",
                        onclick: move |_| on_confirm.call(()),
                        "Open in builder"
                    }
                }
            }
        }
    }
}

fn public_url(path: &str) -> String {
    let path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };

    web_sys::window()
        .and_then(|window| window.location().origin().ok())
        .filter(|origin| !origin.trim().is_empty())
        .map(|origin| format!("{origin}{path}"))
        .unwrap_or(path)
}

fn copy_to_clipboard(value: &str) {
    let Ok(serialized_value) = serde_json::to_string(value) else {
        return;
    };
    let script = format!(
        r#"
        (() => {{
            const value = {serialized_value};
            const fallback = () => {{
                const textarea = document.createElement('textarea');
                textarea.value = value;
                textarea.setAttribute('readonly', '');
                textarea.style.position = 'fixed';
                textarea.style.opacity = '0';
                textarea.style.pointerEvents = 'none';
                document.body.appendChild(textarea);
                textarea.select();
                document.execCommand('copy');
                document.body.removeChild(textarea);
            }};

            if (navigator.clipboard && navigator.clipboard.writeText) {{
                navigator.clipboard.writeText(value).catch(fallback);
            }} else {{
                fallback();
            }}
        }})();
        "#
    );

    let _ = dioxus::document::eval(&script);
}
