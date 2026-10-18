use axum::{Json, extract::State};
use serde::Serialize;

use crate::{AppState, error::AppError};

/// `GET /health` 的响应体。
#[derive(Debug, Serialize)]
pub struct Health {
    pub status: &'static str,
    pub version: &'static str,
    pub uptime_seconds: u64,
    pub database: &'static str,
}

/// `GET /health` —— 存活探针，顺带确认数据库可用。
///
/// 数据库不可用时返回 500，方便负载均衡直接摘掉实例。
pub async fn health(State(state): State<AppState>) -> Result<Json<Health>, AppError> {
    sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(Health {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: state.started_at.elapsed().as_secs(),
        database: "up",
    }))
}

/// `GET /about` —— 输出服务信息与端点清单，方便快速自测。
pub async fn about() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": [
            { "method": "GET",    "path": "/health" },
            { "method": "GET",    "path": "/about" },
            { "method": "GET",    "path": "/api/v1/tasks", "query": "status, q, limit, offset" },
            { "method": "POST",   "path": "/api/v1/tasks" },
            { "method": "GET",    "path": "/api/v1/tasks/{id}" },
            { "method": "PATCH",  "path": "/api/v1/tasks/{id}" },
            { "method": "DELETE", "path": "/api/v1/tasks/{id}" },
            { "method": "GET",    "path": "/api/v1/stats" }
        ],
    }))
}

/// 未匹配任何路由时的 404，保持与其他错误一致的 JSON 形状。
pub async fn not_found() -> AppError {
    AppError::NotFound
}
