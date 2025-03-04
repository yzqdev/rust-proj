use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
#[derive(Debug)]
pub struct FileUtil {
    pub name: String,
}

trait FileFunc {
    fn new(&self) -> FileUtil;
}

impl FileFunc for FileUtil {
    fn new(&self) -> FileUtil {
        FileUtil {
            name: "fileutil".to_string(),
        }
    }
}

fn move_file_to_folder(
    file_path: &Path,
    target_folder: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // 确保目标文件夹存在
    if !target_folder.exists() {
        fs::create_dir_all(target_folder)?;
    }

    // 获取文件名并拼接新的目标路径
    let file_name = file_path.file_name().ok_or("无法获取文件名")?;
    let target_path = target_folder.join(file_name);

    // 移动文件
    fs::rename(file_path, target_path)?;

    println!("文件 {} 移动到 {:?}", file_path.display(), target_folder);
    Ok(())
}

pub fn move_file_folder(input_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new(input_folder);

    // 遍历所有文件
    for entry in WalkDir::new(input_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            // 获取文件后缀名
            if let Some(extension) = path.extension() {
                let extension_str = extension.to_string_lossy().to_lowercase();

                // 根据文件扩展名选择目标文件夹
                let target_folder = match extension_str.as_str() {
                    "jpg" | "png" | "webp" => "images",
                    "mp3" | "wav" => "audio",
                    "txt" => "text_files",

                    "pdf" => "pdf_files",
                    _ => "others", // 默认文件夹
                };

                let target_path = Path::new(target_folder);

                // 移动文件到相应文件夹
                if let Err(e) = move_file_to_folder(path, target_path) {
                    eprintln!("移动文件失败 {:?}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(())
}
mod tests {
    use super::move_file_folder;

    #[test]
    fn test_main() {
        let input_folder = "input_folder"; // 输入文件夹
        if let Err(e) = move_file_folder(input_folder) {
            eprintln!("处理文件夹失败: {}", e);
        }
    }
}
