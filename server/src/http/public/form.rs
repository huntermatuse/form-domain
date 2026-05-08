use crate::http::ApiContext;
use crate::http::error::Error;
use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::types::Json as SqlJson;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/f/{token}", get(public_get_form_handler))
        .route("/api/v1/f/{token}/submit", post(public_submit_form_handler))
        .route("/api/v1/viewer/{token}", get(public_viewer_handler))
}

async fn public_get_form_handler(
    Extension(ctx): Extension<ApiContext>,
    Path(token): Path<Uuid>,
) -> Result<Json<Form>, Error> {
    let token_form = load_form_for_token(&ctx, token).await?;
    Ok(Json(token_form.form))
}

async fn public_viewer_handler(
    Extension(ctx): Extension<ApiContext>,
    Path(token): Path<Uuid>,
) -> Result<Json<CompletedForm>, Error> {
    let completed_form = load_completed_form_for_viewer_token(&ctx, token).await?;
    Ok(Json(completed_form))
}

async fn public_submit_form_handler(
    Extension(ctx): Extension<ApiContext>,
    Path(token): Path<Uuid>,
    Json(completed_form): Json<CompletedForm>,
) -> Result<StatusCode, Error> {
    let submitted_at = parse_submission_timestamp(&completed_form.submission.submitted_at)?;

    let mut tx = ctx.db.begin().await?;
    let token_form = load_form_for_token_for_update(&mut tx, token).await?;

    let completed_form_id: Uuid = sqlx::query_scalar(
        r#"
        insert into form.completed_form
            (
                form_id,
                form_version,
                share_token_id,
                company_name,
                signer_name,
                signer_title,
                submitted_at
            )
        values ($1, $2, $3, $4, $5, $6, $7)
        returning completed_form_id
        "#,
    )
    .bind(token_form.form_id)
    .bind(token_form.version)
    .bind(token_form.share_token_id)
    .bind(completed_form.submission.company_name.trim())
    .bind(completed_form.submission.signer_name.trim())
    .bind(completed_form.submission.signer_title.trim())
    .bind(submitted_at)
    .fetch_one(&mut *tx)
    .await?;

    for response in completed_form.responses {
        let answered_at = response
            .answered_at
            .as_deref()
            .map(parse_submission_timestamp)
            .transpose()?;
        let response = serde_json::to_value(response)
            .map_err(|err| anyhow::anyhow!("could not serialize question response: {err}"))?;

        sqlx::query(
            r#"
            insert into form.question_response
                (completed_form_id, response, answered_at)
            values ($1, $2, $3)
            "#,
        )
        .bind(completed_form_id)
        .bind(SqlJson(response))
        .bind(answered_at)
        .execute(&mut *tx)
        .await?;
    }

    let updated = sqlx::query(
        r#"
        update form.share_token
        set
            active = false,
            used_at = now(),
            updated_by = 'respondent'
        where share_token_id = $1
            and active = true
            and used_at is null
        "#,
    )
    .bind(token_form.share_token_id)
    .execute(&mut *tx)
    .await?;

    if updated.rows_affected() != 1 {
        return Err(Error::UsedShareToken);
    }

    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(sqlx::FromRow)]
struct TokenFormRow {
    share_token_id: Uuid,
    token_active: bool,
    expires_at: Option<DateTime<Utc>>,
    used_at: Option<DateTime<Utc>>,
    form_id: Uuid,
    version: i32,
    title: String,
    description_markdown: Option<String>,
    form_active: bool,
    form_section: SqlJson<Vec<Section>>,
    created_at: DateTime<Utc>,
    created_by: String,
    updated_at: Option<DateTime<Utc>>,
    updated_by: Option<String>,
}

struct TokenForm {
    share_token_id: Uuid,
    form_id: Uuid,
    version: i32,
    form: Form,
}

#[derive(sqlx::FromRow)]
struct ViewerCompletedFormRow {
    viewer_active: bool,
    viewer_expires_at: Option<DateTime<Utc>>,
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

async fn load_form_for_token(ctx: &ApiContext, token: Uuid) -> Result<TokenForm, Error> {
    let token_hash = token_hash(token);

    let row = sqlx::query_as::<_, TokenFormRow>(
        r#"
        select
            st.share_token_id,
            st.active as token_active,
            st.expires_at,
            st.used_at,
            f.form_id,
            f.version,
            f.title,
            f.description_markdown,
            f.active as form_active,
            f.form_section,
            f.created_at,
            f.created_by,
            f.updated_at,
            f.updated_by
        from form.share_token st
        join form.form f
            on f.form_id = st.form_id
            and f.version = st.form_version
        where st.token_hash = $1
        "#,
    )
    .bind(token_hash)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::InvalidShareToken)?;

    validate_public_token(&row)?;
    token_form_from_row(row)
}

async fn load_form_for_token_for_update(
    tx: &mut Transaction<'_, Postgres>,
    token: Uuid,
) -> Result<TokenForm, Error> {
    let token_hash = token_hash(token);

    let row = sqlx::query_as::<_, TokenFormRow>(
        r#"
        select
            st.share_token_id,
            st.active as token_active,
            st.expires_at,
            st.used_at,
            f.form_id,
            f.version,
            f.title,
            f.description_markdown,
            f.active as form_active,
            f.form_section,
            f.created_at,
            f.created_by,
            f.updated_at,
            f.updated_by
        from form.share_token st
        join form.form f
            on f.form_id = st.form_id
            and f.version = st.form_version
        where st.token_hash = $1
        for update of st
        "#,
    )
    .bind(token_hash)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or(Error::InvalidShareToken)?;

    validate_public_token(&row)?;
    token_form_from_row(row)
}

fn token_form_from_row(row: TokenFormRow) -> Result<TokenForm, Error> {
    let version = u32::try_from(row.version).map_err(|_| {
        anyhow::anyhow!(
            "form {} has invalid negative version {}",
            row.form_id,
            row.version
        )
    })?;

    let form = Form {
        form_id: row.form_id.to_string(),
        version,
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

    Ok(TokenForm {
        share_token_id: row.share_token_id,
        form_id: row.form_id,
        version: row.version,
        form,
    })
}

fn validate_public_token(row: &TokenFormRow) -> Result<(), Error> {
    if row.used_at.is_some() {
        return Err(Error::UsedShareToken);
    }

    if !row.token_active {
        return Err(Error::InactiveShareToken);
    }

    if row
        .expires_at
        .is_some_and(|expires_at| expires_at <= Utc::now())
    {
        return Err(Error::ExpiredShareToken);
    }

    if !row.form_active {
        return Err(Error::InactiveForm);
    }

    Ok(())
}

async fn load_completed_form_for_viewer_token(
    ctx: &ApiContext,
    token: Uuid,
) -> Result<CompletedForm, Error> {
    let token_hash = token_hash(token);

    let row = sqlx::query_as::<_, ViewerCompletedFormRow>(
        r#"
        select
            vt.active as viewer_active,
            vt.expires_at as viewer_expires_at,
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
                    qr.response
                    || jsonb_build_object('answered_at', qr.answered_at)
                    order by qr.id
                ) filter (where qr.id is not null),
                '[]'::jsonb
            ) as responses
        from form.viewer_token vt
        join form.completed_form cf
            on cf.completed_form_id = vt.completed_form_id
        join form.form f
            on f.form_id = cf.form_id
            and f.version = cf.form_version
        left join form.question_response qr
            on qr.completed_form_id = cf.completed_form_id
        where vt.token_hash = $1
        group by
            vt.active,
            vt.expires_at,
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
    .bind(token_hash)
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::InvalidViewerToken)?;

    validate_viewer_token(&row)?;
    completed_form_from_row(row)
}

fn validate_viewer_token(row: &ViewerCompletedFormRow) -> Result<(), Error> {
    if !row.viewer_active {
        return Err(Error::InactiveViewerToken);
    }

    if row
        .viewer_expires_at
        .is_some_and(|expires_at| expires_at <= Utc::now())
    {
        return Err(Error::ExpiredViewerToken);
    }

    Ok(())
}

fn completed_form_from_row(row: ViewerCompletedFormRow) -> Result<CompletedForm, Error> {
    let version = u32::try_from(row.version).map_err(|_| {
        anyhow::anyhow!(
            "form {} has invalid negative version {}",
            row.form_id,
            row.version
        )
    })?;

    Ok(CompletedForm {
        completed_form_id: row.completed_form_id.to_string(),
        form: Form {
            form_id: row.form_id.to_string(),
            version,
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

fn token_hash(token: Uuid) -> String {
    Sha256::digest(token.to_string().as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn parse_submission_timestamp(value: &str) -> Result<DateTime<Utc>, Error> {
    let value = value.trim();

    if value.is_empty() {
        return Ok(Utc::now());
    }

    if let Ok(datetime) = DateTime::parse_from_rfc3339(value) {
        return Ok(datetime.with_timezone(&Utc));
    }

    for format in ["%Y-%m-%d", "%m/%d/%Y", "%m/%d/%y"] {
        if let Ok(date) = NaiveDate::parse_from_str(value, format) {
            return midnight_utc(date, value);
        }
    }

    Err(Error::unprocessable_entity([(
        "submitted_at",
        "Use YYYY-MM-DD, MM/DD/YYYY, or an RFC3339 timestamp.",
    )]))
}

fn midnight_utc(date: NaiveDate, original_value: &str) -> Result<DateTime<Utc>, Error> {
    let datetime = date.and_hms_opt(0, 0, 0).ok_or_else(|| {
        anyhow::anyhow!("could not build midnight timestamp for {original_value}")
    })?;
    Ok(Utc.from_utc_datetime(&datetime))
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
