use reqwest::Result;
mod cmd;
mod util;
use tracing::info;
use tracing_subscriber;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");
    let _ = cmd::run_main().await.map_err(|e| eprintln!("error=>{}", e));
    Ok(())
}
