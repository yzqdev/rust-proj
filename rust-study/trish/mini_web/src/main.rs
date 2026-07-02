use mini_web::start_server;

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    start_server().await
}
