use crate::api::http::{get, post_empty_response, ApiResult};
use crate::forms::model::{CompletedForm, Form};

pub async fn fetch_form(token: &str) -> ApiResult<Form> {
    get(&format!("/api/v1/f/{token}")).await
}

pub async fn submit_completed_form(token: &str, completed_form: &CompletedForm) -> ApiResult<()> {
    post_empty_response(&format!("/api/v1/f/{token}/submit"), completed_form).await
}

pub async fn fetch_completed_form(token: &str) -> ApiResult<CompletedForm> {
    get(&format!("/api/v1/viewer/{token}")).await
}
