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
