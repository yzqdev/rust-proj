// 教学示例测试：演示所有权、借用与字符串。
#![allow(unused_variables, dead_code)]

#[test]
fn array_sort() {
    let mut arr = vec![
        "study",
        "fade",
        "mini",
        "rcli",
        "web_app",
        "basic",
        "mini_web",
        "guess",
        "find_file",
        "minigrep",
        "counter",
        "dante",
        "seele",
    ];
    arr.sort();
    println!("{:?}", arr);
}
