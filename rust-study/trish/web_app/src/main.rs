//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-routes-and-handlers-close-together
//! ```

mod controller;

pub use crate::controller::{print_hello};


use axum::{

    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use crate::controller::index_controller::{get_foo, root, post_foo, user_route};

#[tokio::main]
async fn main() {
      tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_versioning=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .merge(root())
        .merge(get_foo())
        .merge(post_foo()).merge(user_route());
 // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
