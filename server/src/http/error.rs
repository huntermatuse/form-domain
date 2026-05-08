use axum::Json;
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized,

    #[error("user may not perform that action")]
    Forbidden,

    #[error("request path not found")]
    NotFound,

    #[error("this form link is not valid")]
    InvalidShareToken,

    #[error("this form link has expired")]
    ExpiredShareToken,

    #[error("this form link has already been used")]
    UsedShareToken,

    #[error("this form link is no longer active")]
    InactiveShareToken,

    #[error("this form is no longer active")]
    InactiveForm,

    #[error("this viewer link is not valid")]
    InvalidViewerToken,

    #[error("this viewer link has expired")]
    ExpiredViewerToken,

    #[error("this viewer link is no longer active")]
    InactiveViewerToken,

    #[error("this link has expired or been revoked")]
    Gone,

    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>> = HashMap::new();
        for (key, val) in errors {
            error_map.entry(key.into()).or_default().push(val.into());
        }
        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InvalidShareToken | Self::InvalidViewerToken => StatusCode::NOT_FOUND,
            Self::ExpiredShareToken
            | Self::UsedShareToken
            | Self::InactiveShareToken
            | Self::InactiveForm
            | Self::ExpiredViewerToken
            | Self::InactiveViewerToken
            | Self::Gone => StatusCode::GONE,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            }
            Self::Unauthorized => {
                return (
                    StatusCode::UNAUTHORIZED,
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
                        .into_iter()
                        .collect::<HeaderMap>(),
                    self.to_string(),
                )
                    .into_response();
            }
            Self::Sqlx(sqlx::Error::Database(ref dbe)) if dbe.code().as_deref() == Some("42P01") => {
                tracing::error!("database schema is not initialized: {:?}", dbe);
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Database schema is not initialized. Run server migrations and retry.",
                )
                    .into_response();
            }
            Self::Sqlx(ref e) => {
                tracing::error!("SQLx error: {:?}", e);
            }
            Self::Anyhow(ref e) => {
                tracing::error!("Generic error: {:?}", e);
            }
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

pub trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}
