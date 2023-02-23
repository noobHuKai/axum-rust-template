use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use thiserror::Error;

#[allow(clippy::enum_variant_names, dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error("{0}")]
    OtherError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("app error : {:?}", self);

        let (status, error_message) = match self {
            AppError::SqlxError(sqlx::Error::RowNotFound) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Not Found Data".to_string(),
            ),
            Self::OtherError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
