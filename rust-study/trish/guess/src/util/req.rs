use std::collections::HashMap;

#[tokio::main]
pub async fn main_req() -> Result<(), Box<dyn std::error::Error>> {
    let url = String::from("http://localhost:8550/cat/single");
    let ip_url = String::from("https://httpbin.org/ip");
    let resp = reqwest::get(url).await?.json().await?;
    println!("req main ");
    println!("{:#?}", resp);
    Ok(())
}
