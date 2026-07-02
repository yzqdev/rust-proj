pub mod use_trait;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    use_trait::use_trait();

    // Demonstrate new utilities
    let s = "A man, a plan, a canal, Panama";
    println!("Is palindrome: {}", fade::str_util::is_palindrome(s));
    println!("Factorial of 10: {}", fade::math_util::factorial(10));
    println!("Random string: {}", fade::rand_util::random_string(12));
}
