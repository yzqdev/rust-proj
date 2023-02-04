use axum::Router;
use axum::routing::{get,post, MethodRouter};
use crate::print_hello;

pub fn root() -> Router {
    async fn handler() -> &'static str {
        "Hello, World!"
    }

    route("/", get(handler))
}

pub fn get_foo() -> Router {
    print_hello();
    async fn handler() -> &'static str {
        "Hi from `GET /foo`"
    }

    route("/foo", get(handler))
}

pub fn post_foo() -> Router {
    async fn handler() -> &'static str {
        "Hi from `POST /foo`"
    }

    route("/foo", post(handler))
}

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}