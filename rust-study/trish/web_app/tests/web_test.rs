use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use web_app::build_router;

async fn response_body(response: axum::response::Response) -> String {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8_lossy(&bytes).into_owned()
}

#[tokio::test]
async fn root_says_hello() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response_body(response).await, "Hello, World!");
}

#[tokio::test]
async fn foo_get_and_post() {
    let app = build_router();
    let response = app
        .oneshot(Request::builder().uri("/foo").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response_body(response).await, "foo router");

    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/foo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response_body(response).await, "post foo");
}

#[tokio::test]
async fn user_post_creates_user() {
    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"yzq"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response_body(response).await;
    assert!(body.contains(r#""username":"yzq""#), "{body}");
    assert!(body.contains(r#""id":1337"#), "{body}");
}

#[tokio::test]
async fn health_reports_ok() {
    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_body(response).await;
    assert!(body.contains(r#""status":"ok""#), "{body}");
}

#[tokio::test]
async fn products_list_is_json_array() {
    let app = build_router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/products")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_body(response).await;
    assert!(body.starts_with('['), "{body}");
}
