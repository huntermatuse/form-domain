use crate::http::ApiContext;
use crate::http::error::Error;
use axum::http::{HeaderMap, StatusCode, header::AUTHORIZATION};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use chrono::{Duration, TimeZone, Utc};
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

mod form;

type HmacSha256 = Hmac<Sha256>;

const ADMIN_TOKEN_TTL_HOURS: i64 = 12;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/auth/session", get(session_handler))
        .route("/api/v1/auth/logout", post(logout_handler))
        .merge(form::router())
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize)]
struct SessionResponse {
    authenticated: bool,
}

async fn login_handler(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Error> {
    let expected = ctx
        .config
        .website_password
        .as_deref()
        .filter(|password| !password.is_empty())
        .ok_or(Error::Forbidden)?;

    if req.password != expected {
        return Err(Error::Unauthorized);
    }

    Ok(Json(LoginResponse {
        token: sign_admin_token(&ctx),
    }))
}

async fn session_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
) -> Result<Json<SessionResponse>, Error> {
    require_admin(&ctx, &headers)?;
    Ok(Json(SessionResponse {
        authenticated: true,
    }))
}

async fn logout_handler(
    Extension(ctx): Extension<ApiContext>,
    headers: HeaderMap,
) -> Result<StatusCode, Error> {
    require_admin(&ctx, &headers)?;
    Ok(StatusCode::NO_CONTENT)
}

pub(super) fn require_admin(ctx: &ApiContext, headers: &HeaderMap) -> Result<(), Error> {
    let token = bearer_token(headers).ok_or(Error::Unauthorized)?;
    verify_admin_token(ctx, token)
}

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(str::trim)
        .filter(|token| !token.is_empty())
}

fn sign_admin_token(ctx: &ApiContext) -> String {
    let expires_at = Utc::now() + Duration::hours(ADMIN_TOKEN_TTL_HOURS);
    let expires_at = expires_at.timestamp();
    let payload = format!("admin:{expires_at}");
    let signature = sign(ctx.config.hmac_key.as_bytes(), &payload);
    format!("{expires_at}.{signature}")
}

fn verify_admin_token(ctx: &ApiContext, token: &str) -> Result<(), Error> {
    let (expires_at, signature) = token.split_once('.').ok_or(Error::Unauthorized)?;
    let expires_at = expires_at.parse::<i64>().map_err(|_| Error::Unauthorized)?;
    let expires_at = Utc
        .timestamp_opt(expires_at, 0)
        .single()
        .ok_or(Error::Unauthorized)?;

    if expires_at <= Utc::now() {
        return Err(Error::Unauthorized);
    }

    let payload = format!("admin:{}", expires_at.timestamp());
    let expected = sign(ctx.config.hmac_key.as_bytes(), &payload);

    if constant_time_eq(signature.as_bytes(), expected.as_bytes()) {
        Ok(())
    } else {
        Err(Error::Unauthorized)
    }
}

fn sign(key: &[u8], payload: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts keys of any size");
    mac.update(payload.as_bytes());
    mac.finalize()
        .into_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    left.iter()
        .zip(right.iter())
        .fold(0_u8, |acc, (left, right)| acc | (left ^ right))
        == 0
}
