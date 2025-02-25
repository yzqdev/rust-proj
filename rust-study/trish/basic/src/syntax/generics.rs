use std::collections::HashMap;

// 泛型函数，用于打印任意类型的值
fn print_value<T: std::fmt::Debug>(value: T) {
    println!("The value is: {:?}", value);
}

// 泛型结构体
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T: std::fmt::Debug, U:std::fmt::Debug> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    fn display(&self) {
        println!("First: {:?}, Second: {:?}", self.first, self.second);
    }
}

pub fn show_generic() {
    let pair = Pair::new(42, "Rust");
    pair.display();  // 输出: First: 42, Second: "Rust"
}
pub fn method_generic(){
        // 模拟 sync_matches 作为一个 HashMap，其中的值是一个 Vec<String>。
    let mut sync_matches = HashMap::new();
    
    // 插入键值对，值是 Vec<String> 类型的列表。
    sync_matches.insert("search", vec!["rust", "programming", "language"]);
    sync_matches.insert("other_key", vec!["example", "value"]);
    
    // 获取与 "search" 相关的多个值。
    if let Some(packages) = sync_matches.get("search") {
         
    } else {
        println!("No matches found for 'search'.");
    }
}