use mini_web::start_server;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match start_server().await {
        Ok(()) => {
            println!("All Redis operations completed successfully!");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Error: {err}");
            eprintln!(
                "Note: this demo requires a redis server listening on 127.0.0.1:6379 \
                 (see https://github.com/tokio-rs/mini-redis for a compatible server)."
            );
            ExitCode::FAILURE
        }
    }
}
