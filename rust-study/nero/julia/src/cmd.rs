pub mod douyin_cmd;

use crate::util::image_util;
use clap::{Command, arg};

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
                    let _ = image_util::jpg_to_webp_folder(folder)
                        .map(|_| println!("完成"))
                        .map_err(|e| eprint!("error=>{}", e));
                }
                (name, _) => {}
            }
        }
        _ => {
            println!("nothing")
        }
    }
    Ok(())
}
