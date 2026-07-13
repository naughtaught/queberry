use backtrace::Backtrace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<ErrorDetail>,
}

impl ErrorResponse {
    pub fn success<T: serde::Serialize>(data: T) -> Self {
        Self {
            success: true,
            data: Some(serde_json::to_value(data).unwrap_or_default()),
            error: None,
        }
    }

    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorDetail {
                code,
                message: message.into(),
                stack: None,
                context: None,
            }),
        }
    }

    pub fn error_with_stack(code: u16, message: impl Into<String>, stack: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorDetail {
                code,
                message: message.into(),
                stack: Some(stack),
                context: None,
            }),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Plugin '{plugin_id}' exceeded memory limit ({limit})")]
    PluginOutOfMemory {
        plugin_id: String,
        limit: String,
        attempted_bytes: Option<usize>,
    },

    #[error("Plugin '{plugin_id}' method '{method}' timed out after {timeout_ms}ms")]
    PluginTimeout {
        plugin_id: String,
        method: String,
        timeout_ms: u64,
    },

    #[error("Plugin '{plugin_id}' crashed: {details}")]
    PluginCrashed { plugin_id: String, details: String },

    #[error("Plugin '{plugin_id}' returned invalid data: {details}")]
    PluginInvalidOutput { plugin_id: String, details: String },
}

impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::Validation(_) | Self::Url(_) => 400,
            Self::Permission(_) => 403,
            Self::NotFound(_) => 404,
            Self::Timeout(_) | Self::PluginTimeout { .. } => 408,
            Self::RateLimit(_) => 429,

            Self::PluginInvalidOutput { .. } => 502,
            Self::PluginOutOfMemory { .. } => 507,

            Self::Io(_)
            | Self::Json(_)
            | Self::Config(_)
            | Self::Runtime(_)
            | Self::Database(_)
            | Self::PluginCrashed { .. } => 500,
        }
    }

    pub fn is_client_error(&self) -> bool {
        self.status_code() >= 400 && self.status_code() < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.status_code() >= 500
    }

    fn capture_backtrace(&self) -> Option<String> {
        if self.is_server_error() {
            Some(format!("{:?}", Backtrace::new()))
        } else {
            None
        }
    }

    pub fn to_error_detail(&self, context: Option<String>) -> ErrorDetail {
        ErrorDetail {
            code: self.status_code(),
            message: self.to_string(),
            stack: self.capture_backtrace(),
            context,
        }
    }

    pub fn to_error_response(&self, context: Option<String>) -> ErrorResponse {
        ErrorResponse {
            success: false,
            data: None,
            error: Some(self.to_error_detail(context)),
        }
    }

    pub fn log(&self, context: &str) {
        let code = self.status_code();
        let msg = self.to_string();

        if self.is_server_error() {
            log::error!("[{}] {} - {}", context, code, msg);
            if let Some(backtrace) = self.capture_backtrace() {
                log::debug!("Backtrace: {}", backtrace);
            }
        } else {
            log::warn!("[{}] {} - {}", context, code, msg);
        }
    }

    pub fn plugin_out_of_memory(
        plugin_id: String,
        limit_pages: Option<u32>,
        attempted_bytes: Option<usize>,
    ) -> Self {
        let limit = match limit_pages {
            Some(pages) => {
                let bytes = pages as usize * 64 * 1024;
                let mb = bytes as f64 / (1024.0 * 1024.0);
                format!("{:.1}MB ({} pages)", mb, pages)
            }
            None => "unlimited".to_string(),
        };
        Self::PluginOutOfMemory {
            plugin_id,
            limit,
            attempted_bytes,
        }
    }

    pub fn plugin_timeout(plugin_id: String, method: String, timeout_ms: u64) -> Self {
        Self::PluginTimeout {
            plugin_id,
            method,
            timeout_ms,
        }
    }

    pub fn plugin_crashed(plugin_id: String, details: String) -> Self {
        Self::PluginCrashed { plugin_id, details }
    }

    pub fn plugin_invalid_output(plugin_id: String, details: String) -> Self {
        Self::PluginInvalidOutput { plugin_id, details }
    }

    pub fn is_plugin_error(&self) -> bool {
        matches!(
            self,
            Self::PluginOutOfMemory { .. }
                | Self::PluginTimeout { .. }
                | Self::PluginCrashed { .. }
                | Self::PluginInvalidOutput { .. }
        )
    }

    pub fn plugin_id(&self) -> Option<&str> {
        match self {
            Self::PluginOutOfMemory { plugin_id, .. }
            | Self::PluginTimeout { plugin_id, .. }
            | Self::PluginCrashed { plugin_id, .. }
            | Self::PluginInvalidOutput { plugin_id, .. } => Some(plugin_id),
            _ => None,
        }
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        Self::Runtime(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        Self::Runtime(s.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Runtime(e.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound("Record not found".to_string()),
            sqlx::Error::Database(ref e) => {
                if e.is_unique_violation() {
                    Self::Validation(format!("Duplicate entry: {}", e.message()))
                } else if e.is_foreign_key_violation() {
                    Self::Validation(format!("Foreign key violation: {}", e.message()))
                } else {
                    Self::Database(format!("Database error: {}", e.message()))
                }
            }
            sqlx::Error::ColumnNotFound(name) => {
                Self::Runtime(format!("Column not found: {}", name))
            }
            sqlx::Error::Decode(e) => Self::Runtime(format!("Data decode error: {}", e)),
            _ => Self::Database(format!("Database error: {}", error)),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        success: bool,
        data: T,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<ErrorDetail>,
    },
    Error {
        success: bool,
        data: Option<T>,
        error: ErrorDetail,
    },
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self::Success {
            success: true,
            data,
            error: None,
        }
    }

    pub fn err(code: u16, message: impl Into<String>) -> Self {
        Self::Error {
            success: false,
            data: None,
            error: ErrorDetail {
                code,
                message: message.into(),
                stack: None,
                context: None,
            },
        }
    }

    pub fn err_with_context(
        code: u16,
        message: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self::Error {
            success: false,
            data: None,
            error: ErrorDetail {
                code,
                message: message.into(),
                stack: None,
                context: Some(context.into()),
            },
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    pub fn data(self) -> Option<T> {
        match self {
            Self::Success { data, .. } => Some(data),
            Self::Error { data, .. } => data,
        }
    }

    pub fn error_detail(&self) -> Option<&ErrorDetail> {
        match self {
            Self::Success { error, .. } => error.as_ref(),
            Self::Error { error, .. } => Some(error),
        }
    }
}

impl<T> From<AppError> for ApiResponse<T> {
    fn from(error: AppError) -> Self {
        Self::Error {
            success: false,
            data: None,
            error: error.to_error_detail(None),
        }
    }
}

impl<T> From<Result<T, AppError>> for ApiResponse<T> {
    fn from(result: Result<T, AppError>) -> Self {
        match result {
            Ok(data) => Self::ok(data),
            Err(err) => err.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum PluginErrorKind {
    Crash,
    OutOfMemory,
    Timeout,
    Unknown,
}

impl PluginErrorKind {
    fn from_message(err_msg: &str) -> Self {
        let err_msg = err_msg.to_lowercase();

        const CRASH_SIGNALS: &[&str] = &["unreachable", "wasm trap", "trap", "panic", "abort"];
        if CRASH_SIGNALS.iter().any(|&s| err_msg.contains(s)) {
            return Self::Crash;
        }

        const OOM_SIGNALS: &[&str] = &["out of memory", "oom", "allocation failed", "memory limit"];
        if OOM_SIGNALS.iter().any(|&s| err_msg.contains(s)) {
            return Self::OutOfMemory;
        }

        const TIMEOUT_SIGNALS: &[&str] = &["timeout", "deadline", "exceeded"];
        if TIMEOUT_SIGNALS.iter().any(|&s| err_msg.contains(s)) {
            return Self::Timeout;
        }

        Self::Unknown
    }
}

pub fn classify_plugin_error(
    plugin_id: &str,
    method: &str,
    error: &str,
    timeout_ms: u64,
    memory_limit: Option<u32>,
) -> AppError {
    match PluginErrorKind::from_message(error) {
        PluginErrorKind::Crash => {
            AppError::plugin_crashed(plugin_id.to_string(), error.to_string())
        }
        PluginErrorKind::OutOfMemory => {
            AppError::plugin_out_of_memory(plugin_id.to_string(), memory_limit, None)
        }
        PluginErrorKind::Timeout => {
            AppError::plugin_timeout(plugin_id.to_string(), method.to_string(), timeout_ms)
        }
        PluginErrorKind::Unknown => AppError::Runtime(format!(
            "Plugin '{}' method '{}' failed: {}",
            plugin_id, method, error
        )),
    }
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;
pub type PlayerResult<T> = Result<T>;

pub fn handle_command<T, F>(context: &str, f: F) -> ApiResponse<T>
where
    F: FnOnce() -> Result<T>,
    T: Serialize,
{
    match f() {
        Ok(data) => ApiResponse::ok(data),
        Err(e) => {
            e.log(context);
            e.into()
        }
    }
}

pub async fn handle_command_async<T, F, Fut>(
    context: &str,
    f: F,
) -> Result<ApiResponse<T>, AppError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
    T: Serialize,
{
    match f().await {
        Ok(data) => Ok(ApiResponse::ok(data)),
        Err(e) => {
            e.log(context);
            Ok(e.into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Download cancelled")]
    Cancelled,

    #[error("Network error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("HTTP {status}: {url}")]
    HttpStatus { status: u16, url: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Insufficient disk space: need {need} bytes, have {available} bytes")]
    InsufficientSpace { need: u64, available: u64 },

    #[error("Download incomplete: received {0} bytes, expected more")]
    Incomplete(u64),

    #[error("Download failed after {attempts} attempts: {last_error}")]
    RetriesExhausted { attempts: u32, last_error: String },

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

impl DownloadError {
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http(e) => e.is_timeout() || e.is_connect(),
            Self::HttpStatus { status, .. } => *status >= 500,
            Self::Incomplete(_) => true,
            Self::InvalidState(_) => false,
            _ => false,
        }
    }
}

impl From<DownloadError> for AppError {
    fn from(e: DownloadError) -> Self {
        match e {
            DownloadError::Cancelled => Self::Runtime("Download cancelled".to_string()),
            DownloadError::Io(e) => Self::Io(e),
            DownloadError::Http(e) => Self::Runtime(format!("Network error: {}", e)),
            DownloadError::HttpStatus { status, url } => {
                Self::Runtime(format!("HTTP {}: {}", status, url))
            }
            DownloadError::InvalidUrl(u) => Self::Runtime(format!("Invalid URL: {}", u)),
            DownloadError::InsufficientSpace { need, available } => Self::Runtime(format!(
                "Insufficient disk space: need {} bytes, have {} bytes",
                need, available
            )),
            DownloadError::Incomplete(bytes) => {
                Self::Runtime(format!("Download incomplete after {} bytes", bytes))
            }
            DownloadError::RetriesExhausted {
                attempts,
                last_error,
            } => Self::Runtime(format!(
                "Download failed after {} attempts: {}",
                attempts, last_error
            )),
            DownloadError::InvalidState(msg) => {
                Self::Runtime(format!("Invalid download state: {}", msg))
            }
        }
    }
}

impl From<AppError> for DownloadError {
    fn from(e: AppError) -> Self {
        DownloadError::InvalidState(e.to_string())
    }
}

pub type DownloadResult<T> = Result<T, DownloadError>;
