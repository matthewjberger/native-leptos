use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(thiserror::Error)]
pub enum ApiError {
    #[error("Resource not found")]
    NotFound,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal server error")]
    Internal,
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, formatter)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND", self.to_string()),
            ApiError::Database(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                message.clone(),
            ),
            ApiError::Validation(message) => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message.clone())
            }
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", self.to_string()),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                self.to_string(),
            ),
        };

        let body = Json(json!({
            "error": message,
            "code": code
        }));

        (status, body).into_response()
    }
}
