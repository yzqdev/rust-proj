pub fn get_array() {
    let mut a = vec![1, 2, 4];
    a.push(11);
    println!("{:?}", a);
}
pub fn destruct() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

struct Struct {
    e: i32,
}

fn check_destruct() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 使用 filter 筛选出所有偶数
    let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();

    println!("原始数组中的偶数有: {:?}", even_numbers);

    let guess = "42".parse::<String>().expect("Not a number!");
    for i in 1..4 {}
}

fn simple_borrow() {
    let s = String::from("hello"); // s 进入作用域

    let borrowed = takes_ownership(s); // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    println!("在move进函数后继续使用s: {}", borrowed);
    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(some_string: String) -> String {
    // some_string 进入作用域
    println!("{}", some_string);
    some_string
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
