pub mod io_use;
pub mod trait_use;
pub mod macro_use;

/// String utilities module
pub mod string_util {
    /// Count occurrences of a substring in a string
    pub fn count_occurrences(text: &str, pattern: &str) -> usize {
        text.matches(pattern).count()
    }

    /// Truncate a string to a maximum length, adding "..." if truncated
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }

    /// Check if a string starts with a digit
    pub fn starts_with_digit(s: &str) -> bool {
        s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
    }
}

/// Path utilities module  
pub mod path_util {
    use std::path::Path;

    /// Normalize a path, converting backslashes to forward slashes
    pub fn normalize(path_str: &str) -> String {
        path_str.replace('\\', "/")
    }

    /// Check if a file has a given extension
    pub fn has_extension(path_str: &str, ext: &str) -> bool {
        Path::new(path_str)
            .extension()
            .map(|e| e == ext)
            .unwrap_or(false)
    }
}
