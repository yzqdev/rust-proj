//! basic - Rust basics demo library (struct/array/string/generic demos).

pub mod datatype;
pub mod syntax;

/// Parse a string to a number, returning None on failure.
pub fn parse_number(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

/// Sum an array using iterator.
pub fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

/// String utility: capitalize first letter.
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Struct demo (teaching example).
pub fn struct_demo() {
    let runoob = datatype::struct_data::Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("rect1 is {:?}, area: {}", rect1, rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("struct data {runoob:?}");
    datatype::array_data::get_array();
}

/// String demo (teaching example).
pub fn string_demo() {
    let text = "hello world";
    println!("Original: {text}");
    println!("Capitalized: {}", capitalize(text));
    println!("Parse '42': {:?}", parse_number("42"));
    println!("Sum of [1,2,3,4,5]: {}", sum_array(&[1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numbers() {
        assert_eq!(parse_number("42"), Some(42));
        assert_eq!(parse_number("-7"), Some(-7));
        assert_eq!(parse_number("abc"), None);
    }

    #[test]
    fn sums_arrays() {
        assert_eq!(sum_array(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_array(&[]), 0);
    }

    #[test]
    fn capitalizes() {
        assert_eq!(capitalize("rust"), "Rust");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("already Capital"), "Already Capital");
    }

    #[test]
    fn rectangle_area_and_hold() {
        let big = Rectangle {
            width: 30,
            height: 50,
        };
        let small = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(big.area(), 1500);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }
}
