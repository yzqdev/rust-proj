use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    AppState,
    error::AppError,
    models::{CreateTask, Page, Stats, Task, TaskFilter, UpdateTask},
    repo,
};

/// `GET /api/v1/tasks` —— 分页 + 过滤查询。
pub async fn list(
    State(state): State<AppState>,
    Query(filter): Query<TaskFilter>,
) -> Result<Json<Page<Task>>, AppError> {
    Ok(Json(repo::list(&state.pool, &filter).await?))
}

/// `POST /api/v1/tasks` —— 新建任务，成功返回 201。
pub async fn create(
    State(state): State<AppState>,
    Json(input): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    input.validate()?;

    let task = repo::create(&state.pool, &input).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

/// `GET /api/v1/tasks/{public_id}` —— 按 id 查单个任务。
pub async fn get_one(
    State(state): State<AppState>,
    Path(public_id): Path<String>,
) -> Result<Json<Task>, AppError> {
    Ok(Json(repo::get(&state.pool, &public_id).await?))
}

/// `PATCH /api/v1/tasks/{public_id}` —— 局部更新。
pub async fn update(
    State(state): State<AppState>,
    Path(public_id): Path<String>,
    Json(patch): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    patch.validate()?;

    Ok(Json(repo::update(&state.pool, &public_id, &patch).await?))
}

/// `DELETE /api/v1/tasks/{public_id}` —— 删除，成功返回 204。
pub async fn remove(
    State(state): State<AppState>,
    Path(public_id): Path<String>,
) -> Result<StatusCode, AppError> {
    repo::remove(&state.pool, &public_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// `GET /api/v1/stats` —— 按状态汇总。
pub async fn stats(State(state): State<AppState>) -> Result<Json<Stats>, AppError> {
    Ok(Json(repo::stats(&state.pool).await?))
}
