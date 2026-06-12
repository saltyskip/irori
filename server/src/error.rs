use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Database(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            Error::Storage(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            Error::Auth(ref msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            Error::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Error::InvalidInput(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Error::Conflict(ref msg) => (StatusCode::CONFLICT, msg.clone()),
            Error::QuotaExceeded(ref msg) => (StatusCode::PAYMENT_REQUIRED, msg.clone()),
            Error::Internal(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = serde_json::to_string(&ErrorResponse {
            error: error_message,
            details: None,
        })
        .unwrap_or_else(|_| r#"{"error":"Internal server error"}"#.to_string());

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        Error::Database(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        tracing::error!("IO error: {:?}", err);
        Error::Storage(err.to_string())
    }
}
