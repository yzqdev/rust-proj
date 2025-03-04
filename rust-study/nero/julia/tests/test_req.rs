use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use tokio;
#[tokio::test]
async fn test_local() -> Result<(), Error> {
    let res = reqwest::get("http://127.0.0.1:8550/cat/admin").await;
    match res {
        Ok(resp) => {
            println!("response =>{:?}", resp.status())
        }
        Err(e) => {
            eprint!("eeeeeee{:?}", e)
        }
    }

    Ok(())
}
#[derive(Deserialize, Debug)]
struct ApiResponse {
    #[serde(flatten)] // 捕获所有未知字段
    extra: serde_json::Map<String, Value>,
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
#[tokio::test]

async fn test_req_local() -> Result<(), Error> {
    // 调用 request_douyin 函数
    if let Err(e) = request_douyin().await {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

async fn request_douyin() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let body = client
        .get("http://127.0.0.1:8550/cat/admin")
        .send()
        .await?
        .text()
        .await;

    // 打印整个 body 对象
    println!("Body: {:?}", body);
    // 打印具体的字段

    Ok(())
}

#[tokio::test]
async fn test_main() -> Result<(), reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    let response = reqwest::get(url).await?.json::<Todo>().await?;

    println!("ID:  {:?}", response);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}
