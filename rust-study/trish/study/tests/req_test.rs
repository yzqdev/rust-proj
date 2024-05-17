#![deny(warnings)]

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[cfg(not(target_arch = "wasm32"))]
 
#[tokio::test]
async fn req_env() -> Result<(), reqwest::Error> {
    // Some simple CLI args requirements...
    let url = "https://httpbin.org/get";

    eprintln!("Fetching {url:?}...");

    // reqwest::get() is a convenience function.
    //
    // In most cases, you should create/build a reqwest::Client and reuse
    // it for all requests.
    let res = reqwest::get(url).await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    Ok(())
}
#[test]
fn use_req(){
    println!("hello")
}