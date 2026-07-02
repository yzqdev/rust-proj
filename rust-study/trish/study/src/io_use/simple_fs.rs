use std::{env, fs, path::Path};
use md5::Digest;

pub fn get_file_md5() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", &args[1]);
        let f = fs::read(&args[1]);
        println!("{:x}", md5::Md5::digest(f.unwrap()));
    }
}

/// Compute MD5 hash of a file, returning hex string
pub fn compute_md5(path: &str) -> Result<String, std::io::Error> {
    let data = fs::read(path)?;
    Ok(format!("{:x}", md5::Md5::digest(data)))
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Get file size and display it
pub fn file_size(path: &str) {
    match fs::metadata(path) {
        Ok(meta) => {
            let size = meta.len();
            let size_str = if size < 1024 {
                format!("{} B", size)
            } else if size < 1024 * 1024 {
                format!("{:.2} KB", size as f64 / 1024.0)
            } else {
                format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
            };
            println!("Size of '{}': {} ({})", path, size_str, size);
        }
        Err(e) => eprintln!("Error reading '{}': {}", path, e),
    }
}

/// List directory contents
pub fn list_dir(path: &str) {
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut dirs = Vec::new();
            let mut files = Vec::new();
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    dirs.push(format!("[DIR]  {}", name));
                } else {
                    let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                    files.push(format!("[FILE] {} ({} bytes)", name, size));
                }
            }
            dirs.sort();
            files.sort();
            println!("Contents of '{}':", path);
            for d in &dirs {
                println!("  {}", d);
            }
            for f in &files {
                println!("  {}", f);
            }
            println!("Total: {} entries", dirs.len() + files.len());
        }
        Err(e) => eprintln!("Error reading directory '{}': {}", path, e),
    }
}

/// Check if a file exists
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path, process};
    use crate::io_use::conf_constant::UNBUILD_CONF;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        println!("{}", result);
        fs::write(Path::new("./target/build.config.ts"), UNBUILD_CONF)
            .expect("cant find target foldr");
        assert_eq!(result, 4);
    }
}
