// 教学示例：保留原始写法。
#![allow(clippy::all)]

pub fn get_str_type(string: String) {
    let arr: [i32; 4] = [1, 3, 4, 5];
    let _arr_vec = vec![1, 3, 4, 5];
    println!("{:?}", arr);
    let length = string.len();
    println!("{length}")
}
