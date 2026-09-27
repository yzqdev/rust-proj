use guess::util::req::main_req;
use std::thread;
use std::time::Duration;

// 依赖本地服务（127.0.0.1:8550），默认跳过；本地起服务后用
// `cargo test -p guess -- --ignored` 手动运行。
#[test]
#[ignore = "requires a local service on 127.0.0.1:8550"]
fn req() {
    main_req();
    thread::sleep(Duration::from_secs(3));
    println!("hello");
}
