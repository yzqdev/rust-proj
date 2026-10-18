use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// 统一错误类型。
///
/// handler 与数据访问层都可以用 `?` 把错误向上抛，最终由 [`IntoResponse`]
/// 转换成固定形状的 JSON：
///
/// ```json
/// { "error": { "code": "not_found", "message": "资源不存在" } }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// 资源不存在 → 404。
    #[error("资源不存在")]
    NotFound,

    /// 请求参数不合法 → 400。
    #[error("{0}")]
    Validation(String),

    /// 与现有数据冲突（如唯一键重复）→ 409。
    #[error("{0}")]
    Conflict(String),

    /// 未预期的内部错误 → 500，细节只写日志不外泄。
    #[error("内部错误")]
    Internal(#[source] anyhow::Error),

    /// 数据库错误 → 500，细节只写日志不外泄。
    #[error("数据库错误")]
    Database(#[source] sqlx::Error),
}

impl AppError {
    /// 构造一个内部错误，便于用 `anyhow!` 风格的简短写法。
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(anyhow::anyhow!(message.into()))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self::Internal(error)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        // 把 SQLite 约束冲突翻译成更贴切的 HTTP 语义，其余原样上报。
        if let sqlx::Error::Database(db_error) = &error {
            let message = db_error.message();
            if message.contains("UNIQUE constraint failed") {
                return Self::Conflict("资源已存在".to_string());
            }
            if message.contains("CHECK constraint failed") {
                return Self::Validation(format!("字段取值不符合约束: {message}"));
            }
        }

        if matches!(error, sqlx::Error::RowNotFound) {
            return Self::NotFound;
        }

        Self::Database(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            Self::NotFound => (StatusCode::NOT_FOUND, "not_found", self.to_string()),
            Self::Validation(_) => (StatusCode::BAD_REQUEST, "invalid_request", self.to_string()),
            Self::Conflict(_) => (StatusCode::CONFLICT, "conflict", self.to_string()),
            Self::Internal(error) => {
                tracing::error!(error = %error, "未处理的内部错误");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    self.to_string(),
                )
            }
            Self::Database(error) => {
                tracing::error!(error = %error, "数据库错误");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    self.to_string(),
                )
            }
        };

        let body = json!({ "error": { "code": code, "message": message } });
        (status, Json(body)).into_response()
    }
}
