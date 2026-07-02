use std::fs;
use std::io::Read;
use std::path::Path;

use colored::Colorize;
use digest::Digest;

pub fn calc_md5(file_path: &str) {
    use std::time::Instant;
    let now = Instant::now();

    let path = Path::new(file_path);
    if !path.exists() {
        eprintln!("Error: file '{}' not found", file_path);
        return;
    }

    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: cannot open '{}': {}", file_path, e);
            return;
        }
    };

    let mut hasher = md5::Md5::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).unwrap_or(0);
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();

    let file_size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    println!("File:     {}", file_path.cyan());
    println!("Size:     {} bytes", file_size.to_string().yellow());
    println!("MD5:      {:x}", result);
    println!("Elapsed:  {}s", now.elapsed().as_secs_f64());
}

pub fn image_info(file_path: &str) {
    let path = Path::new(file_path);
    if !path.exists() {
        eprintln!("Error: file '{}' not found", file_path);
        return;
    }

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown")
        .to_lowercase();

    let is_image = ["png", "jpg", "jpeg", "gif", "bmp", "webp", "svg", "ico"]
        .contains(&ext.as_str());

    if !is_image {
        eprintln!("Warning: '{}' may not be an image file", file_path);
    }

    println!("{}", format!("=== Image Info: {} ===", file_path).green());
    println!("Type:     {}", ext.to_uppercase());
    println!("Size:     {} bytes", metadata.len());
    println!("Read-only: {}", metadata.permissions().readonly());
}

pub fn file_info(file_path: &str) {
    let path = Path::new(file_path);
    if !path.exists() {
        eprintln!("Error: file '{}' not found", file_path);
        return;
    }

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let file_type = if metadata.is_dir() {
        "directory"
    } else if metadata.is_file() {
        "file"
    } else {
        "other"
    };

    println!("{}", format!("=== File Info: {} ===", file_path).green());
    println!("Type:     {}", file_type);
    println!("Size:     {} bytes", metadata.len());
    println!(
        "Read-only: {}",
        if metadata.permissions().readonly() {
            "yes".red()
        } else {
            "no".green()
        }
    );
}

pub fn dir_tree(dir_path: &str) {
    let path = Path::new(dir_path);
    if !path.exists() || !path.is_dir() {
        eprintln!("Error: '{}' is not a valid directory", dir_path);
        return;
    }

    println!("{}", format!("Directory tree: {}", dir_path).green());
    print_tree(path, 0, "");
}

fn print_tree(dir: &Path, depth: usize, prefix: &str) {
    if depth > 3 {
        println!("{}   ... (max depth reached)", prefix);
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut items: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .collect();

    items.sort_by_key(|e| e.file_name());

    let count = items.len();
    for (i, entry) in items.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let name = entry.file_name();
        let name_str = name.to_string_lossy().to_string();

        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            println!("{}{}{}/", prefix, connector, name_str.cyan());
            print_tree(&entry.path(), depth + 1, &new_prefix);
        } else {
            let size = entry.metadata().ok().map(|m| m.len()).unwrap_or(0);
            println!("{}{}{} ({})", prefix, connector, name_str, format_bytes(size));
        }
    }
}

fn format_bytes(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut s = size as f64;
    for unit in UNITS {
        if s < 1024.0 {
            return format!("{:.1} {}", s, unit);
        }
        s /= 1024.0;
    }
    format!("{:.2} TB", s)
}
