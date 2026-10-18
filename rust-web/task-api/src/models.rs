use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::error::AppError;

// ---------------------------------------------------------------------------
// 领域模型
// ---------------------------------------------------------------------------

/// 任务状态。序列化为 `snake_case` 字符串，与数据库中的取值一致。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

impl TaskStatus {
    /// 全部取值，便于遍历校验。
    pub const ALL: [TaskStatus; 4] = [Self::Todo, Self::InProgress, Self::Done, Self::Cancelled];

    /// 数据库 / JSON 中的字符串表示。
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Todo => "todo",
            Self::InProgress => "in_progress",
            Self::Done => "done",
            Self::Cancelled => "cancelled",
        }
    }

    /// 宽松解析：无法识别时返回 `None`，交由调用方决定报什么错。
    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == value)
    }

    /// 从数据库读出的值解析；取不到说明数据被外部改坏了。
    pub fn from_db(value: &str) -> Result<Self, AppError> {
        Self::parse(value)
            .ok_or_else(|| AppError::internal(format!("数据库中出现了非法的任务状态: {value}")))
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 对外的任务表示。
///
/// `id` 是 UUID（对应库里的 `public_id`），不向外部暴露自增主键。
#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: i64,
    pub due_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 数据库行 → 领域模型的中间结构。
///
/// 有了它，`status` 在库里是 `TEXT`、在 API 里是强类型枚举，两边都不会互相妥协。
#[derive(Debug, FromRow)]
pub(crate) struct TaskRow {
    public_id: String,
    title: String,
    description: Option<String>,
    status: String,
    priority: i64,
    due_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TaskRow {
    pub(crate) fn into_task(self) -> Result<Task, AppError> {
        Ok(Task {
            id: self.public_id,
            title: self.title,
            description: self.description,
            status: TaskStatus::from_db(&self.status)?,
            priority: self.priority,
            due_at: self.due_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

// ---------------------------------------------------------------------------
// 请求体
// ---------------------------------------------------------------------------

/// `POST /api/v1/tasks` 的请求体。
#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<TaskStatus>,
    #[serde(default)]
    pub priority: Option<i64>,
    #[serde(default)]
    pub due_at: Option<DateTime<Utc>>,
}

impl CreateTask {
    /// 字段级校验。业务规则集中在模型里，handler 只负责调用。
    pub fn validate(&self) -> Result<(), AppError> {
        ensure_title(&self.title)?;
        ensure_description(self.description.as_deref())?;
        ensure_priority(self.priority)?;
        Ok(())
    }
}

/// `PATCH /api/v1/tasks/{id}` 的请求体。
///
/// 可空字段用 `Option<Option<T>>` 区分两种语义：
/// - 字段缺席 → `None`（不改动）
/// - 显式传 `null` → `Some(None)`（清空）
#[derive(Debug, Deserialize)]
pub struct UpdateTask {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, deserialize_with = "double_option")]
    pub description: Option<Option<String>>,
    #[serde(default)]
    pub status: Option<TaskStatus>,
    #[serde(default)]
    pub priority: Option<i64>,
    #[serde(default, deserialize_with = "double_option")]
    pub due_at: Option<Option<DateTime<Utc>>>,
}

impl UpdateTask {
    pub fn validate(&self) -> Result<(), AppError> {
        if let Some(title) = &self.title {
            ensure_title(title)?;
        }
        if let Some(Some(description)) = &self.description {
            ensure_description(Some(description))?;
        }
        ensure_priority(self.priority)?;
        Ok(())
    }

    /// 是否至少带了一个要修改的字段。
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.description.is_none()
            && self.status.is_none()
            && self.priority.is_none()
            && self.due_at.is_none()
    }
}

fn double_option<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

fn ensure_title(title: &str) -> Result<(), AppError> {
    let title = title.trim();
    if title.is_empty() {
        return Err(AppError::Validation("title 不能为空".to_string()));
    }
    if title.chars().count() > 200 {
        return Err(AppError::Validation("title 最长 200 个字符".to_string()));
    }
    Ok(())
}

fn ensure_description(description: Option<&str>) -> Result<(), AppError> {
    if let Some(description) = description {
        if description.chars().count() > 2000 {
            return Err(AppError::Validation(
                "description 最长 2000 个字符".to_string(),
            ));
        }
    }
    Ok(())
}

fn ensure_priority(priority: Option<i64>) -> Result<(), AppError> {
    if let Some(priority) = priority {
        if !(0..=4).contains(&priority) {
            return Err(AppError::Validation(
                "priority 取值范围为 0..=4".to_string(),
            ));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// 列表查询参数与响应
// ---------------------------------------------------------------------------

/// `GET /api/v1/tasks` 的查询参数。
///
/// `status` 故意声明为 `String`：非法取值要给出清晰的 400，而不是让
/// serde 抛出一个笼统的解析错误。
#[derive(Debug, Deserialize)]
pub struct TaskFilter {
    #[serde(default)]
    pub status: Option<String>,
    /// 标题/描述的关键字模糊匹配。
    #[serde(default)]
    pub q: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

impl TaskFilter {
    pub const DEFAULT_LIMIT: i64 = 20;
    pub const MAX_LIMIT: i64 = 100;

    /// 归一化后的分页大小，永远落在 `1..=MAX_LIMIT`。
    pub fn limit(&self) -> i64 {
        self.limit
            .unwrap_or(Self::DEFAULT_LIMIT)
            .clamp(1, Self::MAX_LIMIT)
    }

    /// 归一化后的偏移量，负数视为 0。
    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }

    /// 解析并校验状态过滤条件。
    pub fn status(&self) -> Result<Option<TaskStatus>, AppError> {
        match &self.status {
            None => Ok(None),
            Some(value) => TaskStatus::parse(value).map(Some).ok_or_else(|| {
                AppError::Validation(format!(
                    "status 取值无效: {value}（可选: todo, in_progress, done, cancelled）"
                ))
            }),
        }
    }

    /// 去掉首尾空白后的关键字，空串视为未提供。
    pub fn keyword(&self) -> Option<&str> {
        self.q
            .as_deref()
            .map(str::trim)
            .filter(|keyword| !keyword.is_empty())
    }
}

/// 通用分页响应。
#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    /// 满足过滤条件的总条数（不受分页影响）。
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// `GET /api/v1/stats` 的响应。
#[derive(Debug, Serialize)]
pub struct Stats {
    pub total: i64,
    pub by_status: Vec<StatusCount>,
}

#[derive(Debug, Serialize)]
pub struct StatusCount {
    pub status: TaskStatus,
    pub count: i64,
}
