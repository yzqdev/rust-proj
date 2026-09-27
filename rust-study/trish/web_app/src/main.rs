use std::process::ExitCode;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web_app::build_router;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web_app=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = build_router();

    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => {
            tracing::debug!("listening on {}", listener.local_addr().unwrap());
            match axum::serve(listener, app).await {
                Ok(()) => ExitCode::SUCCESS,
                Err(err) => {
                    eprintln!("Error serving: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Err(err) => {
            eprintln!("Error: cannot bind to 0.0.0.0:3000: {err}");
            ExitCode::FAILURE
        }
    }
}
