use crate::controller::user_controller::create_user;
use crate::print_hello;
use axum::Router;
use axum::routing::{MethodRouter, get, post};

pub fn root() -> Router {
    async fn handler() -> &'static str {
        "Hello, World!"
    }

    route("/", get(handler))
}

pub fn get_foo() -> Router {
    print_hello();
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
    route("/user", post(create_user));
    return route("/user", get(handler));
}
pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
