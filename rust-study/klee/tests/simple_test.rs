pub fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        assert_eq!(reverse("racecar"), "racecar");
    }
    #[test]
    fn test_sentence() {
        println!("Hello, world!");
        assert_eq!(reverse("step on no pets"), "step on no pets");
    }
}