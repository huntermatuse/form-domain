use super::require_admin;
use crate::http::ApiContext;
use crate::http::error::Error;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::types::Json as SqlJson;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/v1/admin/forms",
            get(list_forms_handler).post(create_form_handler),
        )
        .route(
            "/api/v1/admin/forms/{form_id}/versions/{version}",
            get(get_form_handler),
        )
        .route(
            "/api/v1/admin/forms/{form_id}/versions/{version}/active",
            post(set_form_active_handler),
        )
        .route(
            "/api/v1/admin/forms/{form_id}/versions/{version}/share-tokens",
            get(list_share_tokens_handler).post(create_share_token_handler),
        )
        .route(
            "/api/v1/admin/share-tokens/{share_token_id}/deactivate",
            post(deactivate_share_token_handler),
        )
        .route(
            "/api/v1/admin/forms/{form_id}/versions/{version}/submissions",
            get(list_submissions_handler),
        )
        .route(
            "/api/v1/admin/submissions/{completed_form_id}",
            get(get_submission_handler),
        )
}

async fn list_forms_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
) -> Result<Json<Vec<FormListItem>>, Error> {
    require_admin(&ctx, &headers)?;

    let rows = sqlx::query_as::<_, FormListItemRow>(
        r#"
        select form_id, version, title, active, created_at, created_by
        from form.form
        order by created_at desc
        "#,
    )
    .fetch_all(&ctx.db)
    .await?;

    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

async fn create_form_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
    Json(req): Json<CreateFormRequest>,
) -> Result<Json<FormDetail>, Error> {
    require_admin(&ctx, &headers)?;
    validate_form_request(&req)?;

    let form_id = Uuid::new_v4();
    let version = 1_i32;

    let row = sqlx::query_as::<_, FormDetailRow>(
        r#"
        insert into form.form
            (form_id, version, title, description_markdown, active, form_section, created_by)
        values ($1, $2, $3, $4, true, $5, $6)
        returning form_id, version, title, description_markdown, active,
            form_section, created_at, created_by, updated_at, updated_by
        "#,
    )
    .bind(form_id)
    .bind(version)
    .bind(req.title.trim())
    .bind(empty_string_as_none(req.description_markdown))
    .bind(SqlJson(req.sections))
    .bind(req.created_by.trim())
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(form_detail_from_row(row)?))
}

async fn get_form_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
    Path((form_id, version)): Path<(Uuid, i32)>,
) -> Result<Json<FormDetail>, Error> {
    require_admin(&ctx, &headers)?;
    Ok(Json(load_form_detail(&ctx, form_id, version).await?))
}

async fn set_form_active_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
    Path((form_id, version)): Path<(Uuid, i32)>,
    Json(req): Json<SetActiveRequest>,
) -> Result<Json<FormDetail>, Error> {
    require_admin(&ctx, &headers)?;

    let row = sqlx::query_as::<_, FormDetailRow>(
        r#"
        update form.form
        set active = $3, updated_by = 'admin'
        where form_id = $1 and version = $2
        returning form_id, version, title, description_markdown, active,
            form_section, created_at, created_by, updated_at, updated_by
        "#,
    )
    .bind(form_id)
    .bind(version)
    .bind(req.active)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(form_detail_from_row(row)?))
}

async fn list_share_tokens_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
    Path((form_id, version)): Path<(Uuid, i32)>,
) -> Result<Json<Vec<ShareTokenItem>>, Error> {
    require_admin(&ctx, &headers)?;

    let rows = sqlx::query_as::<_, ShareTokenRow>(
        r#"
        select share_token_id, token_prefix, form_id, form_version, active,
            expires_at, used_at, notes, created_at, created_by, updated_at, updated_by
        from form.share_token
        where form_id = $1 and form_version = $2
        order by created_at desc
        "#,
    )
    .bind(form_id)
    .bind(version)
    .fetch_all(&ctx.db)
    .await?;

    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

async fn create_share_token_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
    Path((form_id, version)): Path<(Uuid, i32)>,
    Json(req): Json<CreateShareTokenRequest>,
) -> Result<Json<CreateShareTokenResponse>, Error> {
    require_admin(&ctx, &headers)?;

    let form = load_form_detail(&ctx, form_id, version).await?;
    if !form.active {
        return Err(Error::InactiveForm);
    }

    let raw_token = Uuid::new_v4();
    let token = raw_token.to_string();
    let prefix = token.chars().take(8).collect::<String>();

    let row = sqlx::query_as::<_, ShareTokenRow>(
        r#"
        insert into form.share_token
            (token_hash, token_prefix, form_id, form_version, expires_at, notes, created_by)
        values ($1, $2, $3, $4, $5, $6, 'admin')
        returning share_token_id, token_prefix, form_id, form_version, active,
            expires_at, used_at, notes, created_at, created_by, updated_at, updated_by
        "#,
    )
    .bind(token_hash(raw_token))
    .bind(prefix)
    .bind(form_id)
    .bind(version)
    .bind(req.expires_at)
    .bind(empty_string_as_none(req.notes))
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(CreateShareTokenResponse {
        token,
        share_token: row.into(),
    }))
}

async fn deactivate_share_token_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
    Path(share_token_id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    require_admin(&ctx, &headers)?;

    let result = sqlx::query(
        r#"
        update form.share_token
        set active = false, updated_by = 'admin'
        where share_token_id = $1
        "#,
    )
    .bind(share_token_id)
    .execute(&ctx.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn list_submissions_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
    Path((form_id, version)): Path<(Uuid, i32)>,
) -> Result<Json<Vec<SubmissionListItem>>, Error> {
    require_admin(&ctx, &headers)?;

    let rows = sqlx::query_as::<_, SubmissionListRow>(
        r#"
        select
            cf.completed_form_id,
            cf.company_name,
            cf.signer_name,
            cf.signer_title,
            cf.submitted_at,
            coalesce(
                jsonb_agg(qr.response order by qr.id) filter (where qr.id is not null),
                '[]'::jsonb
            ) as responses
        from form.completed_form cf
        left join form.question_response qr
            on qr.completed_form_id = cf.completed_form_id
        where cf.form_id = $1 and cf.form_version = $2
        group by cf.completed_form_id, cf.company_name, cf.signer_name,
            cf.signer_title, cf.submitted_at
        order by cf.submitted_at desc
        "#,
    )
    .bind(form_id)
    .bind(version)
    .fetch_all(&ctx.db)
    .await?;

    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

async fn get_submission_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
    Path(completed_form_id): Path<Uuid>,
) -> Result<Json<CompletedForm>, Error> {
    require_admin(&ctx, &headers)?;

    let row = sqlx::query_as::<_, CompletedFormRow>(
        r#"
        select
            cf.completed_form_id,
            f.form_id,
            f.version,
            f.title,
            f.description_markdown,
            f.form_section,
            f.created_at as form_created_at,
            f.created_by as form_created_by,
            f.updated_at as form_updated_at,
            f.updated_by as form_updated_by,
            cf.company_name,
            cf.signer_name,
            cf.signer_title,
            cf.submitted_at,
            coalesce(
                jsonb_agg(
                    qr.response || jsonb_build_object('answered_at', qr.answered_at)
                    order by qr.id
                ) filter (where qr.id is not null),
                '[]'::jsonb
            ) as responses
        from form.completed_form cf
        join form.form f
            on f.form_id = cf.form_id
            and f.version = cf.form_version
        left join form.question_response qr
            on qr.completed_form_id = cf.completed_form_id
        where cf.completed_form_id = $1
        group by
            cf.completed_form_id,
            f.form_id,
            f.version,
            f.title,
            f.description_markdown,
            f.form_section,
            f.created_at,
            f.created_by,
            f.updated_at,
            f.updated_by,
            cf.company_name,
            cf.signer_name,
            cf.signer_title,
            cf.submitted_at
        "#,
    )
    .bind(completed_form_id)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(completed_form_from_row(row)?))
}

async fn load_form_detail(
    ctx: &ApiContext,
    form_id: Uuid,
    version: i32,
) -> Result<FormDetail, Error> {
    let row = sqlx::query_as::<_, FormDetailRow>(
        r#"
        select form_id, version, title, description_markdown, active,
            form_section, created_at, created_by, updated_at, updated_by
        from form.form
        where form_id = $1 and version = $2
        "#,
    )
    .bind(form_id)
    .bind(version)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::NotFound)?;

    form_detail_from_row(row)
}

fn validate_form_request(req: &CreateFormRequest) -> Result<(), Error> {
    let mut errors = Vec::new();

    if req.title.trim().is_empty() {
        errors.push(("title", "A form name is required."));
    }

    if req.created_by.trim().is_empty() {
        errors.push(("created_by", "Prepared by is required."));
    }

    if req.sections.is_empty() {
        errors.push(("sections", "At least one section is required."));
    }

    for section in &req.sections {
        if section.title.trim().is_empty() {
            errors.push(("sections", "Every section needs a title."));
        }

        for question in &section.questions {
            if question.title.trim().is_empty() {
                errors.push(("questions", "Every field needs a title."));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::unprocessable_entity(errors))
    }
}

fn form_detail_from_row(row: FormDetailRow) -> Result<FormDetail, Error> {
    let form = Form {
        form_id: row.form_id.to_string(),
        version: u32::try_from(row.version)
            .map_err(|_| anyhow::anyhow!("invalid form version {}", row.version))?,
        title: row.title,
        description_markdown: row.description_markdown,
        meta: FormMeta {
            created_at: row.created_at.to_rfc3339(),
            created_by: row.created_by,
            updated_at: row.updated_at.map(|updated_at| updated_at.to_rfc3339()),
            updated_by: row.updated_by,
        },
        sections: row.form_section.0,
    };

    Ok(FormDetail {
        active: row.active,
        form,
    })
}

fn completed_form_from_row(row: CompletedFormRow) -> Result<CompletedForm, Error> {
    Ok(CompletedForm {
        completed_form_id: row.completed_form_id.to_string(),
        form: Form {
            form_id: row.form_id.to_string(),
            version: u32::try_from(row.version)
                .map_err(|_| anyhow::anyhow!("invalid form version {}", row.version))?,
            title: row.title,
            description_markdown: row.description_markdown,
            meta: FormMeta {
                created_at: row.form_created_at.to_rfc3339(),
                created_by: row.form_created_by,
                updated_at: row
                    .form_updated_at
                    .map(|updated_at| updated_at.to_rfc3339()),
                updated_by: row.form_updated_by,
            },
            sections: row.form_section.0,
        },
        submission: FormSubmission {
            company_name: row.company_name,
            signer_name: row.signer_name,
            signer_title: row.signer_title,
            submitted_at: row.submitted_at.to_rfc3339(),
        },
        responses: row.responses.0,
    })
}

fn has_negative_confirmation(responses: &[QuestionResponse]) -> bool {
    responses.iter().any(|response| {
        matches!(
            response.response,
            Response::Validation {
                status: ValidationStatus::NotCorrect,
                ..
            }
        )
    })
}

fn token_hash(token: Uuid) -> String {
    Sha256::digest(token.to_string().as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn empty_string_as_none(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim().to_string();
        if value.is_empty() { None } else { Some(value) }
    })
}

#[derive(sqlx::FromRow)]
struct FormListItemRow {
    form_id: Uuid,
    version: i32,
    title: String,
    active: bool,
    created_at: DateTime<Utc>,
    created_by: String,
}

#[derive(Debug, Serialize)]
pub struct FormListItem {
    form_id: String,
    version: i32,
    title: String,
    active: bool,
    created_at: String,
    created_by: String,
}

impl From<FormListItemRow> for FormListItem {
    fn from(row: FormListItemRow) -> Self {
        Self {
            form_id: row.form_id.to_string(),
            version: row.version,
            title: row.title,
            active: row.active,
            created_at: row.created_at.to_rfc3339(),
            created_by: row.created_by,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CreateFormRequest {
    title: String,
    description_markdown: Option<String>,
    created_by: String,
    sections: Vec<Section>,
}

#[derive(Debug, Deserialize)]
struct SetActiveRequest {
    active: bool,
}

#[derive(sqlx::FromRow)]
struct FormDetailRow {
    form_id: Uuid,
    version: i32,
    title: String,
    description_markdown: Option<String>,
    active: bool,
    form_section: SqlJson<Vec<Section>>,
    created_at: DateTime<Utc>,
    created_by: String,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<String>,
}

#[derive(Debug, Serialize)]
struct FormDetail {
    active: bool,
    form: Form,
}

#[derive(Debug, Deserialize)]
struct CreateShareTokenRequest {
    notes: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct ShareTokenRow {
    share_token_id: Uuid,
    token_prefix: Option<String>,
    form_id: Uuid,
    form_version: i32,
    active: bool,
    expires_at: Option<DateTime<Utc>>,
    used_at: Option<DateTime<Utc>>,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    created_by: String,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<String>,
}

#[derive(Debug, Serialize)]
struct ShareTokenItem {
    share_token_id: String,
    token_prefix: Option<String>,
    form_id: String,
    form_version: i32,
    active: bool,
    expires_at: Option<String>,
    used_at: Option<String>,
    notes: Option<String>,
    created_at: String,
    created_by: String,
    updated_at: Option<String>,
    updated_by: Option<String>,
}

impl From<ShareTokenRow> for ShareTokenItem {
    fn from(row: ShareTokenRow) -> Self {
        Self {
            share_token_id: row.share_token_id.to_string(),
            token_prefix: row.token_prefix,
            form_id: row.form_id.to_string(),
            form_version: row.form_version,
            active: row.active,
            expires_at: row.expires_at.map(|value| value.to_rfc3339()),
            used_at: row.used_at.map(|value| value.to_rfc3339()),
            notes: row.notes,
            created_at: row.created_at.to_rfc3339(),
            created_by: row.created_by,
            updated_at: row.updated_at.map(|value| value.to_rfc3339()),
            updated_by: row.updated_by,
        }
    }
}

#[derive(Debug, Serialize)]
struct CreateShareTokenResponse {
    token: String,
    share_token: ShareTokenItem,
}

#[derive(sqlx::FromRow)]
struct SubmissionListRow {
    completed_form_id: Uuid,
    company_name: String,
    signer_name: String,
    signer_title: String,
    submitted_at: DateTime<Utc>,
    responses: SqlJson<Vec<QuestionResponse>>,
}

#[derive(Debug, Serialize)]
struct SubmissionListItem {
    completed_form_id: String,
    company_name: String,
    signer_name: String,
    signer_title: String,
    submitted_at: String,
    has_negative_confirmation: bool,
}

impl From<SubmissionListRow> for SubmissionListItem {
    fn from(row: SubmissionListRow) -> Self {
        Self {
            completed_form_id: row.completed_form_id.to_string(),
            company_name: row.company_name,
            signer_name: row.signer_name,
            signer_title: row.signer_title,
            submitted_at: row.submitted_at.to_rfc3339(),
            has_negative_confirmation: has_negative_confirmation(&row.responses.0),
        }
    }
}

#[derive(sqlx::FromRow)]
struct CompletedFormRow {
    completed_form_id: Uuid,
    form_id: Uuid,
    version: i32,
    title: String,
    description_markdown: Option<String>,
    form_section: SqlJson<Vec<Section>>,
    form_created_at: DateTime<Utc>,
    form_created_by: String,
    form_updated_at: Option<DateTime<Utc>>,
    form_updated_by: Option<String>,
    company_name: String,
    signer_name: String,
    signer_title: String,
    submitted_at: DateTime<Utc>,
    responses: SqlJson<Vec<QuestionResponse>>,
}

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
    pub description: Option<String>,
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
