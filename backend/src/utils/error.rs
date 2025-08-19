use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("CSV parsing error: {0}")]
    CsvError(String),

    #[error("File upload error: {0}")]
    FileUploadError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::ValidationError(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::CsvError(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::FileUploadError(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::ConfigError(ref msg) => {
                tracing::error!("Configuration error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

// Additional error conversions
impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::BadRequest(format!("JSON error: {}", error))
    }
}

impl From<csv::Error> for AppError {
    fn from(error: csv::Error) -> Self {
        AppError::CsvError(format!("CSV parsing error: {}", error))
    }
}

impl From<csv::IntoInnerError<csv::Writer<Vec<u8>>>> for AppError {
    fn from(_: csv::IntoInnerError<csv::Writer<Vec<u8>>>) -> Self {
        AppError::InternalServerError
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        AppError::BadRequest(format!("Invalid UTF-8: {}", error))
    }
}

impl From<std::fmt::Error> for AppError {
    fn from(_: std::fmt::Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(error: axum::extract::multipart::MultipartError) -> Self {
        AppError::BadRequest(format!("Multipart error: {}", error))
    }
}
