use std::collections::HashMap;

#[tokio::main]
pub async fn main_req()  {
    let url = String::from("http://localhost:8550/cat/single");
    let ip_url = String::from("https://httpbin.org/ip");
    let resp = reqwest::get(url).await.expect("errr").json::<String>().await;
    println!("req main ");
    println!("{:#?}", resp.expect("hehh") );

}
