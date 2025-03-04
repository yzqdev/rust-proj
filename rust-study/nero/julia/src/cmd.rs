pub mod douyin_cmd;

use crate::util::{file_util, image_util};
use anyhow::Error;
use clap::{Command, arg};
use colored::Colorize;
use douyin_cmd::douyin_command;
use reqwest::Result;
fn cli() -> Command {
    Command::new("julia")
        .about("spider")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("douyin")
                .subcommand(Command::new("db"))
                .subcommand(Command::new("req")),
        )
        .subcommand(
            Command::new("image").subcommand(Command::new("webp").arg(arg!(<out_dir> "输出目录"))),
        )
        .subcommand(
            Command::new("file").subcommand(Command::new("sort").arg(arg!(<folder> "目录"))),
        )
}
pub async fn run_main() -> Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("douyin", douyin_cmd)) => {
            let douyin_param = douyin_cmd.subcommand().unwrap();
            match douyin_param {
                ("db", db_param) => {
                    println!("run db command{:?}", db_param)
                }
                ("req", req_param) => {
                    // println!("req command{:?}", req_param);
                    douyin_command().await;
                }
                (name, _) => {
                    println!("no support {name}")
                }
            }
        }
        Some(("image", iamge_arg)) => {
            let image_arg_then = iamge_arg.subcommand().unwrap();
            match image_arg_then {
                ("webp", out_dir) => {
                    let folder = out_dir.get_one::<String>("out_dir").expect("webp");

                    println!("转换到{folder}文件夹");
                    if let Err(e) = image_util::jpg_to_webp_folder(folder) {
                        eprintln!("error=>{}", e);
                    };
                }
                (name, _) => {
                    println!("{}", "请输入指令".cyan())
                }
            }
        }
        Some(("file", file_arg)) => {
            let file_arg_then = file_arg.subcommand().unwrap_or(("help", file_arg));
            match file_arg_then {
                ("sort", folder) => {
                    let _ = file_util::move_file_folder(
                        folder.get_one::<String>("folder").expect("nothing"),
                    );
                }

                (_name, _) => {
                    println!("{}", "请输入指令".cyan())
                }
            }
        }
        _ => {
            println!("nothing")
        }
    }
    Ok(())
}
