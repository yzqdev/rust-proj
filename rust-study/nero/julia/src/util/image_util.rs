use anyhow::{Error, Result};
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::WalkDir;
use webp::Encoder;

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn convert_jpg_to_webp(
    input_file_path: &Path,
    output_folder: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // 读取 JPG 图片
    let img = image::open(input_file_path)?;

    // 将图片转换为 WebP
    let encoder: Encoder = Encoder::from_image(&img)?;
    let webp_data = encoder.encode(80.0); // 设置 WebP 质量（0-100）
    // 获取文件名并修改扩展名
    let file_stem = input_file_path
        .file_stem()
        .ok_or("无法获取文件名")?
        .to_string_lossy();

    let output_file_path: PathBuf = env::current_dir()?
        .join(output_folder)
        .join(format!("{}.webp", file_stem));

    // 保存为 WebP
    fs::write(
        output_folder.join(Path::new(&output_file_path)),
        &*webp_data,
    )?;
    println!("Converted: {:?} -> {:?}", input_file_path, output_file_path);

    Ok(())
}

pub fn jpg_to_webp_folder(out_path: &str) -> Result<()> {
    let in_folder = env::current_dir()?;
    let out_folder = Path::new(out_path);

    if !out_folder.exists() {
        let _ = fs::create_dir_all(out_folder);
    }
    for entry in WalkDir::new(in_folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("png"))
        {
            convert_jpg_to_webp(path, out_folder).map_err(|e| eprintln!("error is=>{:?}", e));
        }
    }

    Ok(())
}
