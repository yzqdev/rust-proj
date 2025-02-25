// 泛型函数示例：打印任意实现了 Display trait 的类型
pub fn print_value<T: std::fmt::Display>(value: T) {
    println!("The value is: {}", value);
}

// 泛型结构体示例：定义一个可以存储任何类型的 Box
struct Box<T> {
    value: T,
}

impl<T> Box<T> {
    // 泛型方法：获取存储的值
    fn get(&self) -> &T {
        &self.value
    }
}

// 泛型 trait 示例：定义一个可以计算长度的 trait
trait Length {
    fn len(&self) -> usize;
}

// 为 String 类型实现 Length trait
impl Length for String {
    fn len(&self) -> usize {
        self.len()
    }
}

// 为 Vec<T> 泛型类型实现 Length trait（当 T 也实现 Length 时）
impl<T: Length> Length for Vec<T> {
    fn len(&self) -> usize {
        self.iter().sum::<usize>(|item| item.len())
    }
}

pub fn generic_app() {
    // 使用泛型函数
    print_value(42);        // i32 类型
    print_value("Hello");   // String 类型

    // 使用泛型结构体
    let int_box = Box::new(5);
    let string_box = Box::new(String::from("Rust"));

    println!("Integer value: {}", int_box.get());
    println!("String value: {}", string_box.get());

    // 使用泛型 trait
    let s = String::from("Hello, world!");
    let len_s = s.len();       // 直接调用 String 的 len 方法

    let vec = vec![
        Box::new(s),
        Box::new(String::from("Rust")),
    ];

    // 需要 Vec<Box<dyn Length>> 才能调用 len 方法
    // 或者让 Box<T> 自动实现 Length 当 T 实现时：
    // （这里为了简化，直接使用动态 dispatch）
    let dynamic_vec: Vec<Box<dyn Length>> = vec;
    println!("Dynamic vector length: {}", dynamic_vec.len());
}