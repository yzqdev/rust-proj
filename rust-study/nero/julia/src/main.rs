use reqwest::Result;
mod cmd;
mod util;
use log::{debug, error, info, trace, warn};
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let _ = cmd::run_main().await.map_err(|e| eprintln!("error=>{}", e));
    Ok(())
}
