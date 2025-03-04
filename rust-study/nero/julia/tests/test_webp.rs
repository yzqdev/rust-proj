use std::{env, path::Path};

use anyhow::Error;
use julia::util::image_util::jpg_to_webp_folder;
use walkdir::WalkDir;

#[test]
fn test_jpg_to_webp() {
    let folder_path = "E:/Desktop/末影龙"; // 替换成你的文件夹路径
    if let Err(e) = jpg_to_webp_folder(folder_path) {
        eprintln!("Error: {}", e);
    }
}
#[test]
fn test_path() -> Result<(), Error> {
    let out_path = "dou";
    let folder = Path::new(out_path);
    let in_folder = env::current_dir()?;
    for entry in WalkDir::new(in_folder).into_iter().filter_map(|e| e.ok()) {
        println!("{:?}", entry.file_name())
    }
    assert_eq!(folder.to_string_lossy(), "dou");
    Ok(())
}
