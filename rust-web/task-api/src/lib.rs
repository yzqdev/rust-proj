//! `rust-web`：一个结构完整的 Rust Web API 示例。
//!
//! 技术栈：**Axum 0.8** + **SQLx 0.9(SQLite)** + **Tokio** + **tracing**。
//!
//! 分层：
//! - [`config`]：环境变量 → [`config::Config`]
//! - [`db`]：连接池 + 迁移
//! - [`models`]：领域模型与请求/响应 DTO
//! - [`repo`]：数据访问层（唯一直接写 SQL 的地方）
//! - [`routes`]：HTTP handler
//! - [`error`]：统一错误类型 → JSON 错误响应
//!
//! 为了方便集成测试，应用组装逻辑放在库里（[`build_router`]），二进制入口只负责启动。

pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod repo;
pub mod routes;

use std::{path::PathBuf, time::Instant};

use axum::{Router, routing::get};
use sqlx::SqlitePool;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

/// 所有 handler 共享的应用状态（通过 `State` 提取器获取）。
///
/// 内部只持有 `SqlitePool`（本身就是 `Arc` 包裹的连接池），克隆开销可忽略。
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    /// 进程启动时间，用于 `/health` 报告 uptime。
    pub started_at: Instant,
}

/// 组装应用时的可选配置。
#[derive(Debug, Clone, Default)]
pub struct AppOptions {
    /// 静态文件目录；`None` 表示不提供前端页面（测试里用默认值）。
    pub static_dir: Option<PathBuf>,
    /// 是否允许任意跨域来源。仅建议在开发环境打开。
    pub permissive_cors: bool,
}

/// 构建完整路由：API + 静态资源 + 中间件。
pub fn build_router(state: AppState, options: AppOptions) -> Router {
    let api = Router::new()
        .route(
            "/tasks",
            get(routes::tasks::list).post(routes::tasks::create),
        )
        .route(
            "/tasks/{public_id}",
            get(routes::tasks::get_one)
                .patch(routes::tasks::update)
                .delete(routes::tasks::remove),
        )
        .route("/stats", get(routes::tasks::stats));

    let router = Router::new()
        .route("/health", get(routes::health::health))
        .route("/about", get(routes::health::about))
        .nest("/api/v1", api)
        .with_state(state);

    // 有静态目录时用它托管前端页面（未命中的路径回落到 index.html），
    // 否则返回统一的 JSON 404。
    let router = match options.static_dir {
        Some(dir) => {
            let index = dir.join("index.html");
            router.fallback_service(ServeDir::new(dir).fallback(ServeFile::new(index)))
        }
        None => router.fallback(routes::health::not_found),
    };

    // CorsLayer 必须在最外层处理预检请求；TraceLayer 再包一层，保证被 CORS 拦下的请求也有日志。
    let cors = if options.permissive_cors {
        CorsLayer::permissive()
    } else {
        CorsLayer::new()
    };

    router.layer(cors).layer(TraceLayer::new_for_http())
}
