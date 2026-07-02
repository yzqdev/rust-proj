/// String utilities for the mini project
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lower = cleaned.to_lowercase();
    lower == reverse(&lower)
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}
