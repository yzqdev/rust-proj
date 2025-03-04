use reqwest::Result;
mod cmd;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    cmd::run_main().await.expect("TODO: panic message");
    Ok(())
}
