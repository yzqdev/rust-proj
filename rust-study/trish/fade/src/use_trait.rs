// 通过 trait 关键即可定义一个 trait，它类似 Go 里面的接口，里面只需要定义一系列方法即可
// 如果要实现这个 trait，那么必须实现里面所有的方法，少一个都不行
trait Animal {
    // 只需要定义方法的参数和返回值签名即可，具体细节交给相应的类型实现
    fn eat(&self);
    fn drink(&self);
}

struct Dog {
    name: String,
    category: &'static str
}

struct Cat {
    name: String,
    category: &'static str
}

// 在 Go 里面只需要给 Dog 实现方法即可
// 只要实现了某个接口里的所有方法，那么就自动实现了该接口
// 但 Rust 则不同，它还要求你必须显式地指定要实现的 trait
impl Animal for Dog {
    fn eat(&self) {
        println!("{} 在吃东西，它是一只 {}", self.name, self.category);
    }
    fn drink(&self) {
        println!("{} 在喝饮料，它是一只 {}", self.name, self.category);
    }
}

impl Animal for Cat {
    fn eat(&self) {
        println!("{} 在吃东西，它是一只 {}", self.name, self.category);
    }
    fn drink(&self) {
        println!("{} 在喝饮料，它是一只 {}", self.name, self.category);
    }
}

// 一个 eat 函数，接收一个泛型 T，但 T 必须是实现了 Animal trait 的类型
// 如果实现了 Animal trait，那么它一定能够调用 eat 方法
// 要是没有 eat 方法，那么它就不叫实现 Animal trait，那么当前的 eat 函数也不可能被调用
fn eat<T: Animal>(animal: &T) {
    animal.eat();
}
fn drink<T: Animal>(animal: &T) {
    animal.drink();
}

pub fn use_trait() {
    let dog = Dog{name: "旺财".to_string(), category: "小狗"};
    let cat = Cat{name: "翠花".to_string(), category: "小猫"};
    eat(&dog);  // 旺财 在吃东西，它是一只 小狗
    eat(&cat);  // 翠花 在吃东西，它是一只 小猫
    drink(&dog);  // 旺财 在喝饮料，它是一只 小狗
    drink(&cat);  // 翠花 在喝饮料，它是一只 小猫
}
