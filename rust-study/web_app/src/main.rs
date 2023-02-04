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
use std::net::SocketAddr;
use crate::controller::index_controller::{get_foo, root, post_foo};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(root())
        .merge(get_foo())
        .merge(post_foo());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
