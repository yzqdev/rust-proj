//! 二进制入口：加载配置 → 建连接池 → 跑迁移 → 启动 HTTP 服务。
//!
//! 应用本身的组装逻辑在 `rust_web::build_router`，因此集成测试可以脱离网络直接压路由。

use std::time::Instant;

use anyhow::Context;
use rust_web::{AppOptions, AppState, build_router, config::Config, db};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env 不存在也无所谓，生产环境直接用真实环境变量。
    let _ = dotenvy::dotenv();
    init_tracing();

    let config = Config::from_env().context("加载配置失败")?;
    tracing::info!(
        addr = %config.bind_addr,
        database = %config.database_url,
        "启动 rust-web"
    );

    let pool = db::connect(&config).await.context("初始化数据库失败")?;
    db::migrate(&pool).await?;

    let state = AppState {
        pool,
        started_at: Instant::now(),
    };
    let app = build_router(
        state,
        AppOptions {
            static_dir: config.static_dir.clone(),
            permissive_cors: config.cors_permissive,
        },
    );

    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .with_context(|| format!("无法监听 {}", config.bind_addr))?;
    tracing::info!("服务已就绪: http://{}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("服务已退出");
    Ok(())
}

/// 日志级别来自 `RUST_LOG`，默认开发友好的配置。
fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,rust_web=debug,tower_http=debug"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .init();
}

/// 等待 Ctrl-C / SIGTERM，收到后让 axum 停止接收新连接并处理完在途请求。
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("无法监听 Ctrl-C 信号");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("无法监听 SIGTERM 信号")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("收到退出信号，正在优雅关闭…");
}
