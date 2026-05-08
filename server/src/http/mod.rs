// server/src/http/mod.rs
// this should stand up the HTTP server and route requests to the appropriate handlers
// admin should have auth, public should not have auth

use crate::config::Config;
use anyhow::Context;
use axum::{Extension, Json, Router, routing::get};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;
#[cfg(feature = "dev-cors")]
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod admin;
pub mod error;
mod public;

#[derive(Clone)]
pub(crate) struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let addr = format!("{}:{}", config.host, config.port);

    let layers = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(ApiContext {
            config: Arc::new(config),
            db,
        }));

    #[cfg(feature = "dev-cors")]
    let app = api_router().layer(layers.layer(dev_cors_layer()));

    #[cfg(not(feature = "dev-cors"))]
    let app = api_router().layer(layers);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("could not bind to {addr}"))?;
    tracing::info!("listening on {addr}");

    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    Router::new()
        .route("/api/v1/status", get(status_handler))
        .route("/api/development/versioning/info", get(version_info_handler))
        .merge(admin::router())
        .merge(public::router())
}

#[cfg(feature = "dev-cors")]
fn dev_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

#[derive(serde::Serialize, serde::Deserialize, Default, PartialEq, Eq)]
struct StatusResponse {
    status: String,
    datetime: String,
}

async fn status_handler() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "ok".to_string(),
        datetime: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    })
}

#[derive(serde::Serialize, serde::Deserialize, Default, PartialEq, Eq)]
struct VersionInfoResponse {
    name: &'static str,
    version: &'static str,
    edition: &'static str,
    build_date: &'static str,
}

async fn version_info_handler() -> Json<VersionInfoResponse> {
    Json(VersionInfoResponse {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        edition: "2024",
        build_date: env!("BUILD_DATE"),
    })
}
