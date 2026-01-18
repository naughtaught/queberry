use backtrace::Backtrace;
use rusqlite::Error as SqliteError;
use rusqlite::ErrorCode;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Plugin '{plugin_id}' exceeded memory limit ({limit}). This plugin may be malicious or poorly written. Consider uninstalling it.")]
    PluginOutOfMemory {
        plugin_id: String,
        limit: String,
        attempted_bytes: Option<usize>,
    },

    #[error("Plugin '{plugin_id}' method '{method}' exceeded timeout limit ({timeout_ms}ms). Consider optimizing the plugin or increasing the timeout.")]
    PluginTimeout {
        plugin_id: String,
        method: String,
        timeout_ms: u64,
    },

    #[error("Plugin '{plugin_id}' encountered a fatal error. This plugin is likely buggy or malicious: {details}")]
    PluginCrashed { plugin_id: String, details: String },

    #[error("Plugin '{plugin_id}' returned invalid data: {details}")]
    PluginInvalidOutput { plugin_id: String, details: String },

    #[error("Database error: {0}")]
    Database(String),
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Runtime(s)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Runtime(error.to_string())
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::Runtime(error.to_string())
    }
}

impl AppError {
    pub fn to_error_response(&self) -> ErrorResponse {
        let (code, message) = match self {
            AppError::Validation(msg) => (400, msg.clone()),
            AppError::Permission(msg) => (403, msg.clone()),
            AppError::NotFound(msg) => (404, msg.clone()),
            AppError::Timeout(msg) => (408, msg.clone()),
            AppError::PluginTimeout { .. } => (408, self.to_string()),
            AppError::RateLimit(msg) => (429, msg.clone()),
            AppError::Config(msg) => (500, msg.clone()),
            AppError::PluginOutOfMemory { .. } => (507, self.to_string()),
            AppError::PluginCrashed { .. } => (500, self.to_string()),
            AppError::PluginInvalidOutput { .. } => (502, self.to_string()),
            AppError::Io(e) => (500, e.to_string()),
            AppError::Json(e) => (500, e.to_string()),
            AppError::Url(e) => (400, e.to_string()),
            AppError::Runtime(msg) => (500, msg.clone()),
            AppError::Database(msg) => (500, msg.clone()),
        };

        ErrorResponse::error_with_stack(code, message, self.get_stack_trace())
    }

    fn get_stack_trace(&self) -> String {
        let backtrace = Backtrace::new();
        format!("{:?}", backtrace)
    }

    pub fn plugin_out_of_memory(
        plugin_id: String,
        limit_pages: Option<u32>,
        attempted_bytes: Option<usize>,
    ) -> Self {
        let limit = if let Some(pages) = limit_pages {
            let bytes = pages as usize * 64 * 1024;
            let mb = bytes as f64 / (1024.0 * 1024.0);
            format!("{:.1}MB ({} pages)", mb, pages)
        } else {
            "unlimited".to_string()
        };

        AppError::PluginOutOfMemory {
            plugin_id,
            limit,
            attempted_bytes,
        }
    }

    pub fn is_plugin_error(&self) -> bool {
        matches!(
            self,
            AppError::PluginOutOfMemory { .. }
                | AppError::PluginTimeout { .. }
                | AppError::PluginCrashed { .. }
                | AppError::PluginInvalidOutput { .. }
        )
    }

    pub fn plugin_id(&self) -> Option<&str> {
        match self {
            AppError::PluginOutOfMemory { plugin_id, .. }
            | AppError::PluginTimeout { plugin_id, .. }
            | AppError::PluginCrashed { plugin_id, .. }
            | AppError::PluginInvalidOutput { plugin_id, .. } => Some(plugin_id),
            _ => None,
        }
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub fn classify_plugin_error(
    plugin_id: &str,
    method: &str,
    error: &str,
    timeout_ms: u64,
    memory_limit: Option<u32>,
) -> AppError {
    let err_msg = error.to_lowercase();

    if err_msg.contains("unreachable")
        || err_msg.contains("trap")
        || err_msg.contains("panic")
        || err_msg.contains("abort")
        || err_msg.contains("wasm trap")
    {
        return AppError::PluginCrashed {
            plugin_id: plugin_id.to_string(),
            details: error.to_string(),
        };
    }

    if err_msg.contains("memory")
        || err_msg.contains("out of memory")
        || err_msg.contains("oom")
        || err_msg.contains("allocation failed")
        || err_msg.contains("memory limit")
    {
        return AppError::plugin_out_of_memory(plugin_id.to_string(), memory_limit, None);
    }

    if err_msg.contains("timeout") || err_msg.contains("deadline") || err_msg.contains("exceeded") {
        return AppError::PluginTimeout {
            plugin_id: plugin_id.to_string(),
            method: method.to_string(),
            timeout_ms,
        };
    }

    AppError::Runtime(format!(
        "Plugin '{}' method '{}' failed: {}",
        plugin_id, method, error
    ))
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;
pub type PlayerResult<T> = Result<T>;

#[derive(Serialize, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err<E: Into<AppError>>(error: E) -> Self {
        let app_error: AppError = error.into();
        app_error.into()
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

    pub fn from_result(result: Result<T, AppError>) -> Self {
        match result {
            Ok(data) => Self::ok(data),
            Err(err) => err.into(),
        }
    }
}

impl<T> From<AppError> for ApiResponse<T> {
    fn from(error: AppError) -> Self {
        error.to_error_response().into()
    }
}

impl<T> From<Result<T, AppError>> for ApiResponse<T> {
    fn from(result: Result<T, AppError>) -> Self {
        ApiResponse::from_result(result)
    }
}

impl<T> From<ErrorResponse> for ApiResponse<T> {
    fn from(error_response: ErrorResponse) -> Self {
        Self {
            success: error_response.success,
            data: None,
            error: error_response.error,
        }
    }
}

impl From<SqliteError> for AppError {
    fn from(error: SqliteError) -> Self {
        match error {
            SqliteError::QueryReturnedNoRows => AppError::NotFound("Record not found".to_string()),

            SqliteError::SqliteFailure(err_code, err_msg) => match err_code.code {
                // Access .code here
                ErrorCode::ConstraintViolation => {
                    let msg = err_msg.unwrap_or_default();
                    if msg.contains("UNIQUE") {
                        AppError::Validation(format!("Duplicate entry: {}", msg))
                    } else {
                        AppError::Validation(format!("Constraint violation: {}", msg))
                    }
                }
                ErrorCode::NotFound => {
                    AppError::NotFound("Database resource not found".to_string())
                }
                ErrorCode::PermissionDenied => {
                    AppError::Permission("Database permission denied".to_string())
                }
                ErrorCode::DatabaseCorrupt => {
                    AppError::Database("Database file is corrupt".to_string())
                }
                ErrorCode::DiskFull => AppError::Runtime("Disk full".to_string()),
                _ => {
                    AppError::Database(format!("SQLite error ({:?}): {:?}", err_code.code, err_msg))
                }
            },

            SqliteError::FromSqlConversionFailure(_, _, _)
            | SqliteError::ToSqlConversionFailure(_)
            | SqliteError::InvalidColumnType(_, _, _) => {
                AppError::Runtime("Data type conversion error".to_string())
            }

            SqliteError::InvalidParameterName(name) => {
                AppError::Validation(format!("Invalid parameter name: {}", name))
            }
            SqliteError::InvalidParameterCount(given, expected) => {
                AppError::Validation(format!("Expected {} parameters, got {}", expected, given))
            }

            SqliteError::InvalidColumnIndex(idx) => {
                AppError::Runtime(format!("Invalid column index: {}", idx))
            }
            SqliteError::InvalidColumnName(name) => {
                AppError::Runtime(format!("Invalid column name: {}", name))
            }

            SqliteError::MultipleStatement => {
                AppError::Database("Multiple statements not allowed".to_string())
            }
            SqliteError::ExecuteReturnedResults => {
                AppError::Runtime("Execute statement returned results".to_string())
            }

            _ => AppError::Runtime(format!("Database error: {}", error)),
        }
    }
}
