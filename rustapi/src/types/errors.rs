use http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::env::VarError;
use serde::Serialize;

/// Errors, there are too many of them
/// TODO: consolidate error type into one solid type
#[derive(Debug)]
pub struct OauthError {
    _code: StatusCode,
    message: String,
    _user_message: String,
}
#[derive(Debug)]
pub(crate) struct AppError(anyhow::Error);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

impl From<VarError> for OauthError {
    fn from(err: VarError) -> Self {
        OauthError::new(format!("Dotenv error: {:#}", err))
    }
}
impl IntoResponse for OauthError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.message);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}
impl From<sqlx::Error> for OauthError {
    fn from(err: sqlx::Error) -> Self {
        OauthError::new(format!("Database query error: {:#}", err))
    }
}
impl From<String> for OauthError {
    fn from(err: String) -> Self {
        OauthError::new(err)
    }
}
impl From<&str> for OauthError {
    fn from(err: &str) -> Self {
        OauthError::new(err)
    }
}
// Tell axum how to convert `AppError` into a response.
impl OauthError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            _user_message: "".to_owned(),
            _code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
pub(crate) async fn error_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}


