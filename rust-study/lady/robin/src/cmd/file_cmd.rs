use std::time::Instant;

use clap::{Args,  Subcommand};

use crate::util;
#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct FileArgs {
    #[command(subcommand)]
    pub command: Option<FileCmd>,
}
#[derive(Subcommand, Debug)]
pub enum FileCmd {
    #[command(name = "md5", about = "Show SCV, or convert CSV to other formats")]
    Md5 { file_name: String },
    #[command(name = "img", about = "Show SCV, or convert CSV to other formats")]
    Image,
}
#[derive(Debug, Args)]
pub struct FileSubCmd {
    file_name: String,
}
pub fn calc_md5(file_path: &str) {
    let now = Instant::now();

    util::gen_fsmd5(file_path);
    println!("用时: {}s", now.elapsed().as_secs());
}
