use std::{env, net::SocketAddr, path::PathBuf};

use anyhow::{Context, Result};

/// 运行期配置，全部来自环境变量（可写在 `.env` 文件里，见 `.env.example`）。
#[derive(Debug, Clone)]
pub struct Config {
    /// HTTP 监听地址，如 `127.0.0.1:8080`。
    pub bind_addr: SocketAddr,
    /// SQLx 连接串，例如 `sqlite://data/app.db`。
    pub database_url: String,
    /// 连接池上限。
    pub db_max_connections: u32,
    /// 静态文件目录；`STATIC_DIR` 显式置空表示不提供页面。
    pub static_dir: Option<PathBuf>,
    /// 是否允许任意跨域来源。
    pub cors_permissive: bool,
}

impl Config {
    /// 从环境变量读取配置，缺省值面向本地开发。
    pub fn from_env() -> Result<Self> {
        let host = env_or("APP_HOST", "127.0.0.1");
        let port = env_or("APP_PORT", "8080");
        let bind_addr = format!("{host}:{port}")
            .parse()
            .with_context(|| format!("APP_HOST/APP_PORT 无法解析为监听地址: {host}:{port}"))?;

        let database_url = env_or("DATABASE_URL", "sqlite://data/app.db");

        let db_max_connections: u32 = env_or("DB_MAX_CONNECTIONS", "5")
            .parse()
            .context("DB_MAX_CONNECTIONS 必须是正整数")?;

        let static_dir = match env::var("STATIC_DIR") {
            Ok(value) if value.trim().is_empty() => None,
            Ok(value) => Some(PathBuf::from(value)),
            Err(_) => Some(PathBuf::from("static")),
        };

        let cors_permissive = env_or("CORS_PERMISSIVE", "true")
            .parse()
            .context("CORS_PERMISSIVE 应为 true 或 false")?;

        Ok(Self {
            bind_addr,
            database_url,
            db_max_connections,
            static_dir,
            cors_permissive,
        })
    }
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
