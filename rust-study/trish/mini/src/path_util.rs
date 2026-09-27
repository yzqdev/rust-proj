use std::path::Path;

/// Get the file extension from a path string
pub fn get_extension(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    path.extension()
        .map(|ext| ext.to_string_lossy().to_string())
}

/// Get the parent directory from a path string
pub fn get_parent(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    path.parent().map(|p| p.to_string_lossy().to_string())
}

/// Check if a path is absolute
pub fn is_absolute(path_str: &str) -> bool {
    Path::new(path_str).is_absolute()
}

/// Join two path components
pub fn join(base: &str, sub: &str) -> String {
    let base_path = Path::new(base);
    base_path.join(sub).to_string_lossy().to_string()
}

/// Get the file name from a path
pub fn file_name(path_str: &str) -> Option<String> {
    Path::new(path_str)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
}
