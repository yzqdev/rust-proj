use anyhow::{Error, Result};
use std::{fs, io, path::Path};

fn read_file(path: &str) -> Result<()> {
    let _file = std::fs::File::open(path)?;
    fs::rename("from", "to")?;
    let file_name = Path::new(path).file_name().ok_or("err")?;

    Ok(())
}
fn move_file_to_folder(file_path: &Path, target_folder: &Path) -> Result<()> {
    // 确保目标文件夹存在
    if !target_folder.exists() {
        fs::create_dir_all(target_folder)?;
    }

    // 获取文件名并拼接新的目标路径
    let file_name = file_path.file_name().ok_or("err")?;
    let target_path = target_folder.join(file_name);

    // 移动文件
    fs::rename(file_path, target_path)?;

    println!("文件 {} 移动到 {:?}", file_path.display(), target_folder);
    Ok(())
}
#[test]
fn test_anyhow_app() {
    match read_file("nonexistent.txt") {
        Ok(_) => println!("Success!"),
        Err(e) => {
            if let Some(io_error) = e.downcast_ref::<io::Error>() {
                println!("IO error: {}", io_error);
            } else {
                println!("Other error: {}", e);
            }
        }
    }
}
