use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<ErrorDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

impl ErrorResponse {
    pub fn success<T: serde::Serialize>(data: T) -> Self {
        Self {
            success: true,
            data: Some(serde_json::to_value(data).unwrap_or_default()),
            error: None,
        }
    }

    pub fn error(code: u16, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorDetail {
                code,
                message,
                stack: None,
            }),
        }
    }

    pub fn error_with_stack(code: u16, message: String, stack: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorDetail {
                code,
                message,
                stack: Some(stack),
            }),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Database error: {0}")]
    Database(String),
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        AppError::Runtime(error.to_string())
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Runtime(error)
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::Runtime(error.to_string())
    }
}

impl AppError {
    pub fn to_error_response(&self) -> ErrorResponse {
        match self {
            AppError::NotFound(msg) => ErrorResponse::error(404, msg.clone()),
            AppError::Validation(msg) => ErrorResponse::error(400, msg.clone()),
            AppError::RateLimit(msg) => ErrorResponse::error(429, msg.clone()),
            AppError::Permission(msg) => ErrorResponse::error(403, msg.clone()),
            AppError::Config(msg) => ErrorResponse::error(500, msg.clone()),
            _ => ErrorResponse::error(500, self.to_string()),
        }
    }
}

// For Tauri commands to return proper error responses
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
