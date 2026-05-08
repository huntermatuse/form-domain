use reqwest::{Client, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    Unauthorized,
    Forbidden,
    NotFound,
    Gone,
    BadRequest(String),
    Server(String),
    Network(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Not authenticated"),
            ApiError::Forbidden => write!(f, "Access denied"),
            ApiError::NotFound => write!(f, "Not found"),
            ApiError::Gone => write!(f, "This link is no longer valid"),
            ApiError::BadRequest(msg) => write!(f, "{msg}"),
            ApiError::Server(msg) => write!(f, "Server error: {msg}"),
            ApiError::Network(msg) => write!(f, "Network error: {msg}"),
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

fn url(path: &str) -> String {
    format!("{}{}", api_base().trim_end_matches('/'), path)
}

fn api_base() -> String {
    runtime_api_base()
        .or_else(compiled_api_base)
        .or_else(dev_server_api_base)
        .or_else(window_origin)
        .unwrap_or_else(|| "http://127.0.0.1:13000".to_string())
}

fn runtime_api_base() -> Option<String> {
    let window = web_sys::window()?;
    runtime_global(&window, "FORM_API_BASE")
        .or_else(|| runtime_global(&window, "API_URL"))
        .map(|base| resolve_api_base(&base))
}

fn compiled_api_base() -> Option<String> {
    option_env!("FORM_API_BASE")
        .or(option_env!("API_URL"))
        .map(str::trim)
        .filter(|base| !base.is_empty())
        .map(resolve_api_base)
}

fn runtime_global(window: &web_sys::Window, name: &str) -> Option<String> {
    js_sys::Reflect::get(window, &name.into())
        .ok()?
        .as_string()
        .filter(|base| !base.trim().is_empty())
}

fn resolve_api_base(base: &str) -> String {
    let base = base.trim();
    if base.is_empty() {
        window_origin().unwrap_or_default()
    } else {
        base.to_string()
    }
}

fn window_origin() -> Option<String> {
    web_sys::window()
        .and_then(|window| window.location().origin().ok())
        .filter(|origin| !origin.trim().is_empty())
}

fn dev_server_api_base() -> Option<String> {
    let location = web_sys::window()?.location();
    let port = location.port().ok()?;

    if port != "8080" {
        return None;
    }

    let protocol = location.protocol().ok()?;
    let hostname = location.hostname().ok()?;

    if protocol.trim().is_empty() || hostname.trim().is_empty() {
        return None;
    }

    Some(format!("{protocol}//{hostname}:13000"))
}

fn client() -> Client {
    Client::new()
}

pub fn save_auth_token(token: &str) {
    if let Some(storage) = web_sys::window().and_then(|w| w.session_storage().ok().flatten()) {
        let _ = storage.set_item("auth_token", token);
    }
}

pub fn clear_auth_token() {
    if let Some(storage) = web_sys::window().and_then(|w| w.session_storage().ok().flatten()) {
        let _ = storage.remove_item("auth_token");
    }
}

pub fn has_auth_token() -> bool {
    token_from_storage().is_some()
}

fn token_from_storage() -> Option<String> {
    web_sys::window()
        .and_then(|w| w.session_storage().ok().flatten())
        .and_then(|s| s.get_item("auth_token").ok().flatten())
}

async fn handle_response<T: DeserializeOwned>(resp: reqwest::Response) -> ApiResult<T> {
    match resp.status() {
        StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized),
        StatusCode::FORBIDDEN => Err(ApiError::Forbidden),
        StatusCode::NOT_FOUND => Err(ApiError::NotFound),
        StatusCode::GONE => Err(ApiError::Gone),
        s if s.is_success() => resp
            .json::<T>()
            .await
            .map_err(|e| ApiError::Network(e.to_string())),
        s if s.is_client_error() => {
            let msg = response_text(resp, "Bad request").await;
            Err(ApiError::BadRequest(msg))
        }
        _ => {
            let msg = response_text(resp, "Server error").await;
            Err(ApiError::Server(msg))
        }
    }
}

async fn handle_empty_response(resp: reqwest::Response) -> ApiResult<()> {
    match resp.status() {
        StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized),
        StatusCode::FORBIDDEN => Err(ApiError::Forbidden),
        StatusCode::NOT_FOUND => Err(ApiError::NotFound),
        StatusCode::GONE => Err(ApiError::Gone),
        s if s.is_success() => Ok(()),
        s if s.is_client_error() => Err(ApiError::BadRequest(
            response_text(resp, "Bad request").await,
        )),
        _ => Err(ApiError::Server(response_text(resp, "Server error").await)),
    }
}

async fn response_text(resp: reqwest::Response, fallback: &str) -> String {
    resp.text()
        .await
        .ok()
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

pub async fn get<T: DeserializeOwned>(path: &str) -> ApiResult<T> {
    let resp = client()
        .get(url(path))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn post<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> ApiResult<T> {
    let resp = client()
        .post(url(path))
        .json(body)
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn post_empty_response<B: Serialize>(path: &str, body: &B) -> ApiResult<()> {
    let resp = client()
        .post(url(path))
        .json(body)
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_empty_response(resp).await
}

pub async fn authed_get<T: DeserializeOwned>(path: &str) -> ApiResult<T> {
    let token = token_from_storage().ok_or(ApiError::Unauthorized)?;
    let resp = client()
        .get(url(path))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn authed_post<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> ApiResult<T> {
    let token = token_from_storage().ok_or(ApiError::Unauthorized)?;
    let resp = client()
        .post(url(path))
        .header("Authorization", format!("Bearer {token}"))
        .json(body)
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn authed_post_empty<T: DeserializeOwned>(path: &str) -> ApiResult<T> {
    let token = token_from_storage().ok_or(ApiError::Unauthorized)?;
    let resp = client()
        .post(url(path))
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Length", "0")
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn authed_post_empty_response(path: &str) -> ApiResult<()> {
    let token = token_from_storage().ok_or(ApiError::Unauthorized)?;
    let resp = client()
        .post(url(path))
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Length", "0")
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_empty_response(resp).await
}
