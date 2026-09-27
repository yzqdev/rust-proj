//! web_app - Axum web application with controllers.

pub mod controller;

use axum::Router;

use crate::controller::health_controller::health_route;
use crate::controller::index_controller::{get_foo, post_foo, root, user_route};
use crate::controller::product_controller::product_route;

/// Build the full application router.
pub fn build_router() -> Router {
    Router::new()
        .merge(root())
        .merge(get_foo())
        .merge(post_foo())
        .merge(user_route())
        .merge(product_route())
        .merge(health_route())
}
