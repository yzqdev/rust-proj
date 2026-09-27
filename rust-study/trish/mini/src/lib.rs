pub mod advance;
pub mod files;
pub mod json_util;
pub mod path_util;
pub mod string_util;

#[cfg(test)]
mod tests {
    use super::path_util;
    use super::string_util;

    #[test]
    fn string_utilities() {
        assert_eq!(string_util::reverse("hello"), "olleh");
        assert_eq!(string_util::word_count("a b  c"), 3);
        assert!(string_util::is_palindrome("A man, a plan, a canal, Panama"));
        assert!(!string_util::is_palindrome("hello"));
        assert_eq!(
            string_util::to_snake_case("camelCaseText"),
            "camel_case_text"
        );
    }

    #[test]
    fn path_utilities() {
        assert_eq!(path_util::get_extension("a/b.txt").as_deref(), Some("txt"));
        assert_eq!(path_util::get_extension("noext").as_deref(), None);
        assert!(path_util::get_parent("a/b.txt").is_some());
        assert_eq!(path_util::file_name("a/b.txt").as_deref(), Some("b.txt"));
        assert!(!path_util::is_absolute("a/b"));
        assert_eq!(
            path_util::join("a", "b").replace(std::path::MAIN_SEPARATOR, "/"),
            "a/b"
        );
    }
}
