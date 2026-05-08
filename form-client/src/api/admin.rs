use crate::api::http::{
    authed_get, authed_post, authed_post_empty_response, clear_auth_token, post, save_auth_token,
    ApiResult,
};
use crate::forms::model::{CompletedForm, Form, Section};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionResponse {
    pub authenticated: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormListItem {
    pub form_id: String,
    pub version: i32,
    pub title: String,
    pub active: bool,
    pub created_at: String,
    pub created_by: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormDetail {
    pub active: bool,
    pub form: Form,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFormRequest {
    pub title: String,
    pub description_markdown: Option<String>,
    pub created_by: String,
    pub sections: Vec<Section>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetActiveRequest {
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateShareTokenRequest {
    pub notes: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareTokenItem {
    pub share_token_id: String,
    pub token_prefix: Option<String>,
    pub form_id: String,
    pub form_version: i32,
    pub active: bool,
    pub expires_at: Option<String>,
    pub used_at: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub created_by: String,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateShareTokenResponse {
    pub token: String,
    pub share_token: ShareTokenItem,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateViewerTokenRequest {
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewerTokenItem {
    pub viewer_token_id: String,
    pub token_prefix: Option<String>,
    pub completed_form_id: String,
    pub active: bool,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub created_by: String,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateViewerTokenResponse {
    pub token: String,
    pub viewer_token: ViewerTokenItem,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmissionListItem {
    pub completed_form_id: String,
    pub company_name: String,
    pub signer_name: String,
    pub signer_title: String,
    pub submitted_at: String,
    pub has_negative_confirmation: bool,
}

pub async fn login(password: String) -> ApiResult<LoginResponse> {
    let response: LoginResponse = post("/api/v1/auth/login", &LoginRequest { password }).await?;
    save_auth_token(&response.token);
    Ok(response)
}

pub async fn fetch_session() -> ApiResult<SessionResponse> {
    authed_get("/api/v1/auth/session").await
}

pub async fn logout() -> ApiResult<()> {
    let result = authed_post_empty_response("/api/v1/auth/logout").await;
    clear_auth_token();
    result
}

pub async fn fetch_forms() -> ApiResult<Vec<FormListItem>> {
    authed_get("/api/v1/admin/forms").await
}

pub async fn check_form_title_available(title: &str) -> ApiResult<bool> {
    #[derive(serde::Deserialize)]
    struct Resp {
        available: bool,
    }
    let encoded = js_sys::encode_uri_component(title)
        .as_string()
        .unwrap_or_default();
    let resp: Resp =
        authed_get(&format!("/api/v1/admin/forms/check-title?title={encoded}")).await?;
    Ok(resp.available)
}

pub async fn create_form(req: &CreateFormRequest) -> ApiResult<FormDetail> {
    authed_post("/api/v1/admin/forms", req).await
}

pub async fn fetch_form(form_id: &str, version: i32) -> ApiResult<FormDetail> {
    authed_get(&format!("/api/v1/admin/forms/{form_id}/versions/{version}")).await
}

pub async fn set_form_active(form_id: &str, version: i32, active: bool) -> ApiResult<FormDetail> {
    authed_post(
        &format!("/api/v1/admin/forms/{form_id}/versions/{version}/active"),
        &SetActiveRequest { active },
    )
    .await
}

pub async fn fetch_share_tokens(form_id: &str, version: i32) -> ApiResult<Vec<ShareTokenItem>> {
    authed_get(&format!(
        "/api/v1/admin/forms/{form_id}/versions/{version}/share-tokens"
    ))
    .await
}

pub async fn create_share_token(
    form_id: &str,
    version: i32,
    req: &CreateShareTokenRequest,
) -> ApiResult<CreateShareTokenResponse> {
    authed_post(
        &format!("/api/v1/admin/forms/{form_id}/versions/{version}/share-tokens"),
        req,
    )
    .await
}

pub async fn deactivate_share_token(share_token_id: &str) -> ApiResult<()> {
    authed_post_empty_response(&format!(
        "/api/v1/admin/share-tokens/{share_token_id}/deactivate"
    ))
    .await
}

pub async fn fetch_viewer_tokens(completed_form_id: &str) -> ApiResult<Vec<ViewerTokenItem>> {
    authed_get(&format!(
        "/api/v1/admin/submissions/{completed_form_id}/viewer-tokens"
    ))
    .await
}

pub async fn create_viewer_token(
    completed_form_id: &str,
    req: &CreateViewerTokenRequest,
) -> ApiResult<CreateViewerTokenResponse> {
    authed_post(
        &format!("/api/v1/admin/submissions/{completed_form_id}/viewer-tokens"),
        req,
    )
    .await
}

pub async fn deactivate_viewer_token(viewer_token_id: &str) -> ApiResult<()> {
    authed_post_empty_response(&format!(
        "/api/v1/admin/viewer-tokens/{viewer_token_id}/deactivate"
    ))
    .await
}

pub async fn fetch_submissions(form_id: &str, version: i32) -> ApiResult<Vec<SubmissionListItem>> {
    authed_get(&format!(
        "/api/v1/admin/forms/{form_id}/versions/{version}/submissions"
    ))
    .await
}

pub async fn fetch_submission(completed_form_id: &str) -> ApiResult<CompletedForm> {
    authed_get(&format!("/api/v1/admin/submissions/{completed_form_id}")).await
}
