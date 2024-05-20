use mini_redis::{client, Result};
use mini_web::start_server;

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    start_server().await
}