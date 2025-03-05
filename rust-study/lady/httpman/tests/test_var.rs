use colored::Colorize;
#[test]

fn test_var() {
    let mut nice = 100;
    nice = 1;
    println!("{}", nice)
}
#[test]
fn test_arr() {
    //tupe
    let tup = (1, "h", 3, "hello");

    println!("tup element{}", tup.1.cyan());
    let arr = [1, 2, 3];
    for ele in arr {
        println!("{}", ele)
    }
    let firs_str = String::from("hello stre");
    let sec_str = firs_str.clone();
    println!("{firs_str}")
}
#[derive(Debug)]
struct Person {
    name: String,
    color: String,
}
#[test]
fn test_str() {
    let name = String::from("hello cdd");
    let co = "rustd".to_owned();
    let rs = "rust";
    let person = Person {
        name: name,
        color: co,
    };
    println!("{}", person.name);
    println!("{:?}", person);
}
#[test]
fn test_pointer_str() {
    let a: &str = "fisrt";
    let b: &str = a;
    println!("{}", a);
}
enum Color {
    Red,
    Yellow,
    Blue,
}
enum BuildLocation {
    Number(i32),
    Name(String),
    Unknown,
}
impl BuildLocation {
    fn print_location(&self) {
        match self {
            BuildLocation::Number(nm) => println!("{}", nm),
            BuildLocation::Name(s) => println!("{}", s),
            BuildLocation::Unknown => println!("unknown"),
        }
    }
}
#[test]
fn test_em() {
    fn print_color(my_color: Color) {
        match my_color {
            Color::Red => println!("red"),
            Color::Yellow => println!("yellow"),
            Color::Blue => println!("blue"),
            _ => (),
        }
    }
    print_color(Color::Blue);
    let house = BuildLocation::Name("thiiii".to_string());
}
#[derive(Debug)]
enum Flavor {
    Spicy,
    Sweet,
    Fruity,
}
#[derive(Debug)]
struct Drink {
    flavor: Flavor,
    price: f64,
}
impl Drink {
    fn new(price: f64) -> Self {
        Drink {
            flavor: Flavor::Fruity,
            price,
        }
    }
    fn buy(&self) {
        if self.price > 10.0 {
            println!("im poor")
        } else {
            println!("buy it ")
        }
    }
}

#[test]
fn test_method() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 6.0,
    };
    println!("{}", sweet.price);
    println!("{:?}", sweet);
    sweet.buy();
    let fruit=Drink::new(0.6);
fruit.buy();
}
