use std::collections::HashMap;

use reqwest::{Error, Proxy};
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct ApiResponse {
    #[serde(rename = "statusCode")]
    status_code: i32,
    msg: String,
    code: i32,
    #[serde(rename = "data")]
    data: CatResponseData,
}

#[derive(Deserialize, Debug)]
struct CatResponseData {
    result: String,
    meta: String,
}
async fn request_douyin() -> Result<(), Error> {
    let client = reqwest::Client::builder().no_proxy().build()?;
    let body = client
        .get("http://localhost:8550/cat/admin")
        .send()
        .await?
        .json::<ApiResponse>()
        .await;
    match body {
        Ok(res) => {
            println!("{:?}", res)
        }
        Err(e) => {
            eprint!("Error:{e:?}")
        }
    }
    Ok(())
}

pub async fn douyin_command() {
    let res = request_douyin();
    let _ = res.await;
}
