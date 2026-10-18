//! 数据访问层。
//!
//! 这里是整个项目里唯一直接编写 SQL 的地方；上层只看到领域模型，
//! 因此将来换库/换 ORM 只需要改动本模块。

use chrono::Utc;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{
        CreateTask, Page, Stats, StatusCount, Task, TaskFilter, TaskRow, TaskStatus, UpdateTask,
    },
};

/// 查询任务时统一选出的列（不含自增主键 `id`）。
const SELECT_COLUMNS: &str =
    "public_id, title, description, status, priority, due_at, created_at, updated_at";

const SQL_GET: &str = "SELECT public_id, title, description, status, priority, due_at, created_at, updated_at \
                       FROM tasks WHERE public_id = ?";

const SQL_INSERT: &str = "INSERT INTO tasks \
                           (public_id, title, description, status, priority, due_at, created_at, updated_at) \
                           VALUES (?, ?, ?, ?, ?, ?, ?, ?) \
                           RETURNING public_id, title, description, status, priority, due_at, created_at, updated_at";

const SQL_DELETE: &str = "DELETE FROM tasks WHERE public_id = ?";

const SQL_STATS: &str = "SELECT status, COUNT(*) FROM tasks GROUP BY status";

/// 分页查询任务，同时返回满足条件的总数。
pub async fn list(pool: &SqlitePool, filter: &TaskFilter) -> Result<Page<Task>, AppError> {
    let status = filter.status()?;
    let keyword = filter.keyword();
    let limit = filter.limit();
    let offset = filter.offset();

    let mut count_query = QueryBuilder::<Sqlite>::new("SELECT COUNT(*) FROM tasks WHERE 1 = 1");
    apply_filters(&mut count_query, status.as_ref(), keyword);
    let total: i64 = count_query
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await?;

    let mut list_query =
        QueryBuilder::<Sqlite>::new(format!("SELECT {SELECT_COLUMNS} FROM tasks WHERE 1 = 1"));
    apply_filters(&mut list_query, status.as_ref(), keyword);
    list_query
        .push(" ORDER BY created_at DESC, id DESC LIMIT ")
        .push_bind(limit)
        .push(" OFFSET ")
        .push_bind(offset);

    let rows = list_query
        .build_query_as::<TaskRow>()
        .fetch_all(pool)
        .await?;
    let items = rows
        .into_iter()
        .map(TaskRow::into_task)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Page {
        items,
        total,
        limit,
        offset,
    })
}

/// 按 `public_id` 取单个任务，不存在时返回 [`AppError::NotFound`]。
pub async fn get(pool: &SqlitePool, public_id: &str) -> Result<Task, AppError> {
    sqlx::query_as::<_, TaskRow>(SQL_GET)
        .bind(public_id)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)?
        .into_task()
}

/// 新建任务，返回持久化后的完整记录。
pub async fn create(pool: &SqlitePool, input: &CreateTask) -> Result<Task, AppError> {
    let now = Utc::now();
    let public_id = Uuid::new_v4().to_string();

    sqlx::query_as::<_, TaskRow>(SQL_INSERT)
        .bind(&public_id)
        .bind(input.title.trim())
        .bind(input.description.as_deref().map(str::trim))
        .bind(input.status.unwrap_or(TaskStatus::Todo).as_str())
        .bind(input.priority.unwrap_or(2))
        .bind(input.due_at)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?
        .into_task()
}

/// 局部更新任务。`UpdateTask` 里用 `Option<Option<T>>` 区分「不动」与「清空」。
pub async fn update(
    pool: &SqlitePool,
    public_id: &str,
    patch: &UpdateTask,
) -> Result<Task, AppError> {
    // 空补丁视为一次读取，避免生成 `UPDATE ... SET updated_at = ?` 这种无意义写入。
    if patch.is_empty() {
        return get(pool, public_id).await;
    }

    let now = Utc::now();

    let mut query = QueryBuilder::<Sqlite>::new("UPDATE tasks SET ");
    {
        // `Separated::push`/`push_bind` 各自都会补一次分隔符，
        // 所以「列名 + 占位符」要拆成 push + push_bind_unseparated 两步。
        let mut set = query.separated(", ");

        if let Some(title) = &patch.title {
            set.push("title =")
                .push_bind_unseparated(title.trim().to_owned());
        }
        if let Some(description) = &patch.description {
            set.push("description =")
                .push_bind_unseparated(description.as_deref().map(str::trim).map(str::to_owned));
        }
        if let Some(status) = &patch.status {
            set.push("status =").push_bind_unseparated(status.as_str());
        }
        if let Some(priority) = &patch.priority {
            set.push("priority =").push_bind_unseparated(*priority);
        }
        if let Some(due_at) = &patch.due_at {
            set.push("due_at =").push_bind_unseparated(*due_at);
        }

        set.push("updated_at =").push_bind_unseparated(now);
    }
    query
        .push(" WHERE public_id = ")
        .push_bind(public_id.to_owned());
    query.push(" RETURNING ");
    query.push(SELECT_COLUMNS);

    query
        .build_query_as::<TaskRow>()
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)?
        .into_task()
}

/// 删除任务；影响行数为 0 说明目标不存在。
pub async fn remove(pool: &SqlitePool, public_id: &str) -> Result<(), AppError> {
    let result = sqlx::query(SQL_DELETE)
        .bind(public_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

/// 按状态汇总任务数量。
pub async fn stats(pool: &SqlitePool) -> Result<Stats, AppError> {
    let rows: Vec<(String, i64)> = sqlx::query_as(SQL_STATS).fetch_all(pool).await?;

    let mut total = 0;
    let mut by_status = Vec::with_capacity(rows.len());
    for (status, count) in rows {
        total += count;
        by_status.push(StatusCount {
            status: TaskStatus::from_db(&status)?,
            count,
        });
    }
    // 固定按枚举顺序输出，客户端不必再排序。
    by_status.sort_by_key(|entry| {
        TaskStatus::ALL
            .iter()
            .position(|s| *s == entry.status)
            .unwrap_or(usize::MAX)
    });

    Ok(Stats { total, by_status })
}

/// 把过滤条件追加到 `WHERE 1 = 1` 之后，供列表与计数两条语句复用。
fn apply_filters(
    qb: &mut QueryBuilder<Sqlite>,
    status: Option<&TaskStatus>,
    keyword: Option<&str>,
) {
    if let Some(status) = status {
        qb.push(" AND status = ").push_bind(status.as_str());
    }

    if let Some(keyword) = keyword {
        // LIKE 里的 %/_ 需要转义，否则用户输入的 % 会变成通配符。
        let pattern = format!("%{}%", escape_like(keyword));
        qb.push(" AND (title LIKE ")
            .push_bind(pattern.clone())
            .push(" ESCAPE '\\' OR description LIKE ")
            .push_bind(pattern)
            .push(" ESCAPE '\\')");
    }
}

/// 转义 `LIKE` 通配符（配合 SQL 中的 `ESCAPE '\'`）。
fn escape_like(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        if matches!(ch, '\\' | '%' | '_') {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}
