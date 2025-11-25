//! Error handling for Personal Controller API

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Standard API error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Application errors
#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
    LlmError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::LlmError(msg) => write!(f, "LLM error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type) = match &self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "validation_error"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
            AppError::LlmError(_) => (StatusCode::SERVICE_UNAVAILABLE, "llm_error"),
        };

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
            status: status.as_u16(),
            details: None,
        };

        tracing::error!("API Error: {}", self);

        (status, Json(error_response)).into_response()
    }
}

// Conversions from common error types
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(format!("JSON error: {}", err))
    }
}

/// Result type for API handlers
pub type ApiResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::NotFound("Company not found".to_string());
        assert_eq!(err.to_string(), "Not found: Company not found");
    }

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse {
            error: "validation_error".to_string(),
            message: "Invalid input".to_string(),
            status: 400,
            details: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("validation_error"));
    }
}
