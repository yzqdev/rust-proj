use std::{path::Path, str::FromStr, time::Duration};

use anyhow::Context;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};

use crate::config::Config;

/// 按配置创建连接池。
pub async fn connect(config: &Config) -> anyhow::Result<SqlitePool> {
    connect_with_url(&config.database_url, config.db_max_connections).await
}

/// 用任意连接串创建连接池（测试会传入 `sqlite::memory:`）。
///
/// SQLite 的推荐配置：
/// - `WAL`：读写并发，写不阻塞读
/// - `busy_timeout`：遇到锁时等待而不是立刻报错
/// - `foreign_keys`：SQLite 默认关闭外键约束
pub async fn connect_with_url(url: &str, max_connections: u32) -> anyhow::Result<SqlitePool> {
    ensure_parent_dir(url);

    let connect_options = SqliteConnectOptions::from_str(url)
        .with_context(|| format!("DATABASE_URL 无法解析: {url}"))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections.max(1))
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(connect_options)
        .await
        .with_context(|| format!("无法连接数据库: {url}"))?;

    Ok(pool)
}

/// 执行 `migrations/` 下的迁移。
///
/// `sqlx::migrate!` 在编译期把 SQL 内嵌进二进制，因此部署时不需要额外带上迁移文件；
/// 已执行过的迁移靠 `_sqlx_migrations` 表和内容哈希去重。
pub async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("执行数据库迁移失败")?;
    Ok(())
}

/// SQLite 不会自动创建数据库文件的父目录，这里补上。
fn ensure_parent_dir(url: &str) {
    let Some(rest) = url.strip_prefix("sqlite:") else {
        return;
    };
    let rest = rest.trim_start_matches("//");
    let file = rest.split('?').next().unwrap_or_default();

    // 内存库（`:memory:` 或 `?mode=memory`）没有文件路径。
    if file.is_empty() || file == ":memory:" {
        return;
    }

    if let Some(parent) = Path::new(file).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                tracing::warn!(%error, dir = %parent.display(), "无法创建数据库所在目录");
            }
        }
    }
}
