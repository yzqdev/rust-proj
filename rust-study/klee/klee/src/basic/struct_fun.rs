#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub sign: u32,
    pub active: bool,
}
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

 impl Rectangle {
    fn new(length: u32, width: u32) -> Rectangle {
        //若字段名和字段值对应变量名相同时，可以使用字段名初始化
        Rectangle { length, width }
    }
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

pub fn get_react() {
    let u1 = Rectangle::new(20, 10);

    println!("{}", u1.area());
}
pub fn struct_pear() {
    //实例化struct,可以不按照声明的顺序，但是每个属性都需要赋值
    //该struct拥有username和password变量所有权
    let mut u1 = User {
        sign: 30,
        active: true,
        username: String::from("test"),
        password: String::from("1234"),
    };

    //可变的实例每个属性都是可变的，因此可以修改实例的属性值
    u1.username = String::from("jack");

    println!("{:?}", u1);

    let u2 = User {
        username: String::from("max"),

        //更新语法，表示剩下的属性都采用u1的值
         ..u1
    };

    println!("{:?}", u2);
}


pub fn use_option(){
       let num1 = Some(5);
    let a_string = Some("A String");
    let some_string: Option<i32> = None;
    println!("num1 is {:?}, a_string is {:?}, some_string is {:?}", num1, a_string, some_string)
}