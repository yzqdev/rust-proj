// use crate::files::f
// json_util
use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_derive::Serialize;
use serde_json::to_string as to_json;
//这个注解用来让serde_derive自行编写需要的代码以便实现struct在内存和磁盘中的转换
#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}
pub fn json_decode() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };
    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();
    println!("json:\n{}\n", as_json);
    println!("cbor:\n{:?}\n", as_cbor);
    println!("bincode:\n{:?}\n", as_bincode);
    //输出
    // json:
    // {"name":"Calabar","population":470000,"latitude":4.95,"longitude":8.33}
    // cbor:
    // [164, 100, 110, 97, 109, 101, 103, 67, 97, 108, 97, 98, 97, 114, 106, 112, 111, 112, 117, 108, 97, 116, 105, 111, 110, 26, 0, 7, 43, 240, 104, 108, 97, 116, 105, 116, 117, 100, 101, 251, 64, 19, 204, 204, 204, 204, 204, 205, 105, 108, 111, 110, 103, 105, 116, 117, 100, 101, 251, 64, 32, 168, 245, 194, 143, 92, 41]
    // bincode:
    // [7, 0, 0, 0, 0, 0, 0, 0, 67, 97, 108, 97, 98, 97, 114, 240, 43, 7, 0, 0, 0, 0, 0, 205, 204, 204, 204, 204, 204, 19, 64, 41, 92, 143, 194, 245, 168, 32, 64]
}
pub fn message() -> String {
    String::from("This is the 2nd module.")
}