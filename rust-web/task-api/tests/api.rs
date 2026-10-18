//! 集成测试。
//!
//! 不监听端口，直接把 `Request` 交给 [`Router`]（`tower::ServiceExt::oneshot`），
//! 数据库用 `sqlite::memory:`，连接池限制为 1 条连接以保证所有请求落在同一个内存库上。

use std::time::Instant;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use rust_web::{AppOptions, AppState, build_router, db};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn test_app() -> Router {
    let pool = db::connect_with_url("sqlite::memory:", 1)
        .await
        .expect("无法创建内存数据库");
    db::migrate(&pool).await.expect("数据库迁移失败");

    build_router(
        AppState {
            pool,
            started_at: Instant::now(),
        },
        AppOptions::default(),
    )
}

/// 发送请求，返回状态码与解析后的 JSON（空响应体为 `Value::Null`）。
///
/// 只用于断言我们自己产出的 JSON；axum 提取器直接拒绝的请求请用 [`send_text`]。
async fn send(app: &Router, request: Request<Body>) -> (StatusCode, Value) {
    let (status, bytes) = send_raw(app, request).await;

    let body = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("响应不是合法 JSON")
    };

    (status, body)
}

/// 发送请求并按 UTF-8 文本返回响应体，用于 axum 内置拒绝（text/plain）。
async fn send_text(app: &Router, request: Request<Body>) -> (StatusCode, String) {
    let (status, bytes) = send_raw(app, request).await;
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

async fn send_raw(app: &Router, request: Request<Body>) -> (StatusCode, axum::body::Bytes) {
    let response = app.clone().oneshot(request).await.expect("请求执行失败");
    let status = response.status();

    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("读取响应体失败")
        .to_bytes();

    (status, bytes)
}

fn request(method: &str, uri: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .expect("构造请求失败")
}

fn json_request(method: &str, uri: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .expect("构造请求失败")
}

/// 便捷地新建一个任务，返回其 `id`。
async fn create_task(app: &Router, title: &str) -> String {
    let (status, body) = send(
        app,
        json_request("POST", "/api/v1/tasks", &json!({ "title": title })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED, "创建任务失败: {body}");
    body["id"].as_str().expect("响应缺少 id").to_string()
}

#[tokio::test]
async fn health_endpoint_reports_ok() {
    let app = test_app().await;

    let (status, body) = send(&app, request("GET", "/health")).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
    assert_eq!(body["database"], "up");
}

#[tokio::test]
async fn about_lists_endpoints() {
    let app = test_app().await;

    let (status, body) = send(&app, request("GET", "/about")).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "rust-web");
    assert!(
        body["endpoints"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
    );
}

#[tokio::test]
async fn unknown_route_returns_json_not_found() {
    let app = test_app().await;

    let (status, body) = send(&app, request("GET", "/nope")).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");
}

#[tokio::test]
async fn task_lifecycle_roundtrip() {
    let app = test_app().await;

    // 1) 新建：默认 status=todo、priority=2
    let (status, created) = send(
        &app,
        json_request(
            "POST",
            "/api/v1/tasks",
            &json!({
                "title": "Write docs",
                "description": "Cover every endpoint",
                "priority": 4,
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["status"], "todo");
    assert_eq!(created["priority"], 4);
    assert_eq!(created["description"], "Cover every endpoint");
    assert!(created["created_at"].is_string());

    let id = created["id"].as_str().expect("缺少 id").to_string();
    assert_eq!(id.len(), 36, "id 应该是 UUID 而不是自增主键");
    assert_eq!(id.matches('-').count(), 4);

    // 2) 按 id 读取
    let (status, fetched) = send(&app, request("GET", &format!("/api/v1/tasks/{id}"))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched["title"], "Write docs");

    // 3) 局部更新；显式传 null 表示清空 description
    let (status, patched) = send(
        &app,
        json_request(
            "PATCH",
            &format!("/api/v1/tasks/{id}"),
            &json!({ "status": "in_progress", "description": null }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(patched["status"], "in_progress");
    assert_eq!(patched["description"], Value::Null);
    assert_eq!(patched["title"], "Write docs", "未提供的字段不应被改动");

    // 4) 过滤查询
    let (status, page) = send(
        &app,
        request("GET", "/api/v1/tasks?status=in_progress&q=docs"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(page["total"], 1);
    assert_eq!(page["items"][0]["id"], id);

    // 过滤条件不匹配时为空
    let (_, empty) = send(&app, request("GET", "/api/v1/tasks?status=done")).await;
    assert_eq!(empty["total"], 0);

    // 5) 统计
    let (status, stats) = send(&app, request("GET", "/api/v1/stats")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(stats["total"], 1);
    assert_eq!(stats["by_status"][0]["status"], "in_progress");
    assert_eq!(stats["by_status"][0]["count"], 1);

    // 6) 删除
    let (status, body) = send(&app, request("DELETE", &format!("/api/v1/tasks/{id}"))).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(body, Value::Null);

    // 7) 删除后不可见
    let (status, body) = send(&app, request("GET", &format!("/api/v1/tasks/{id}"))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");
}

#[tokio::test]
async fn list_paginates_and_clamps_limit() {
    let app = test_app().await;
    for index in 0..3 {
        create_task(&app, &format!("task {index}")).await;
    }

    let (status, page) = send(&app, request("GET", "/api/v1/tasks?limit=999")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(page["limit"], 100, "limit 应被夹到上限");
    assert_eq!(page["total"], 3);
    assert_eq!(page["items"].as_array().unwrap().len(), 3);

    let (_, page) = send(&app, request("GET", "/api/v1/tasks?limit=0&offset=-5")).await;
    assert_eq!(page["limit"], 1, "limit 应被夹到下限");
    assert_eq!(page["offset"], 0, "offset 不应为负数");
    assert_eq!(page["items"].as_array().unwrap().len(), 1, "只取一条");
}

#[tokio::test]
async fn rejects_invalid_payloads() {
    let app = test_app().await;

    // 空标题
    let (status, body) = send(
        &app,
        json_request("POST", "/api/v1/tasks", &json!({ "title": "   " })),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_request");

    // priority 越界
    let (status, body) = send(
        &app,
        json_request(
            "POST",
            "/api/v1/tasks",
            &json!({ "title": "x", "priority": 9 }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("priority")
    );

    // 未知状态值
    let (status, body) = send(&app, request("GET", "/api/v1/tasks?status=bogus")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_request");

    // 非法 JSON 由 axum 的 Json 提取器拒绝（语法错误 → 400，响应体是纯文本）
    let (status, text) = send_text(
        &app,
        Request::builder()
            .method("POST")
            .uri("/api/v1/tasks")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{ not json"))
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(!text.is_empty());
}

#[tokio::test]
async fn update_and_delete_missing_task_return_404() {
    let app = test_app().await;

    let (status, body) = send(
        &app,
        json_request(
            "PATCH",
            "/api/v1/tasks/does-not-exist",
            &json!({ "title": "y" }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");

    let (status, _) = send(&app, request("DELETE", "/api/v1/tasks/does-not-exist")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn empty_patch_is_a_noop() {
    let app = test_app().await;
    let id = create_task(&app, "unchanged").await;

    let (status, body) = send(
        &app,
        json_request("PATCH", &format!("/api/v1/tasks/{id}"), &json!({})),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["title"], "unchanged");
}
