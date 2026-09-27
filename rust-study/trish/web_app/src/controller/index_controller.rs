use axum::Router;
use axum::routing::{get, post};

pub fn root() -> Router {
    async fn handler() -> &'static str {
        "Hello, World!"
    }

    route("/", get(handler))
}

pub fn get_foo() -> Router {
    async fn handler() -> &'static str {
        "foo router"
    }

    route("/foo", get(handler))
}

pub fn post_foo() -> Router {
    async fn handler() -> &'static str {
        "post foo"
    }

    route("/foo", post(handler))
}

pub fn user_route() -> Router {
    async fn handler() -> &'static str {
        "user router"
    }
    Router::new().route("/user", get(handler)).route(
        "/user",
        post(crate::controller::user_controller::create_user),
    )
}

pub fn route(path: &str, method_router: axum::routing::MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
