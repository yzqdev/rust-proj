use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize)]
struct HealthStatus {
    status: String,
    version: String,
    uptime_seconds: u64,
    memory_info: String,
}

static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

pub fn health_route() -> Router {
    START_TIME.get_or_init(Instant::now);
    Router::new()
        .route("/health", get(health_check))
        .route("/api/version", get(get_version))
}

async fn health_check() -> Json<HealthStatus> {
    let uptime = START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0);
    Json(HealthStatus {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        memory_info: format!("{} KB", "N/A"),
    })
}

#[derive(Serialize)]
struct VersionInfo {
    name: String,
    version: String,
    description: String,
}

async fn get_version() -> Json<VersionInfo> {
    Json(VersionInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
    })
}
