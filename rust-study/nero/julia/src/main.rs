mod cmd;
use clap::Command;
use cmd::douyin_cmd::douyin_command;
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
}
#[tokio::main]
async fn main() -> Result<()> {
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
        _ => {
            println!("nothing")
        }
    }
    Ok(())
}
