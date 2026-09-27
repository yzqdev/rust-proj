use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Command, arg};
use clap_complete::shells::Shell;
use sampo::{Error, Result};

fn cli() -> Command {
    Command::new("sampo")
        .about("A fictional versioning CLI (builder-style)")
        .version(clap::crate_version!())
        .author("yzqdev")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new repository")
                .arg(arg!(<NAME> "Project name")),
        )
        .subcommand(
            Command::new("clone")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("diff")
                .about("Compare two commits")
                .arg(arg!(base: [COMMIT]))
                .arg(arg!(head: [COMMIT]))
                .arg(arg!(path: [PATH]).last(true))
                .arg(
                    arg!(--color <WHEN>)
                        .value_parser(["always", "auto", "never"])
                        .num_args(0..=1)
                        .require_equals(true)
                        .default_value("auto")
                        .default_missing_value("always"),
                ),
        )
        .subcommand(
            Command::new("push")
                .about("Pushes things")
                .arg(arg!(<REMOTE> "The remote to target"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("Stage files")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> ... "Files to stage").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("commit")
                .about("Commit staged changes")
                .arg(arg!(-m --message <MESSAGE> "Commit message").required(true)),
        )
        .subcommand(Command::new("status").about("Show working tree status"))
        .subcommand(
            Command::new("stash")
                .args_conflicts_with_subcommands(true)
                .args(push_args())
                .subcommand(Command::new("push").args(push_args()))
                .subcommand(Command::new("pop").arg(arg!([STASH])))
                .subcommand(Command::new("apply").arg(arg!([STASH])))
                .subcommand(Command::new("list")),
        )
        .subcommand(
            Command::new("log")
                .about("Show commit history")
                .arg(arg!(-n --"max-count" <NUM> "Max entries").default_value("10")),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    arg!(--shell <SHELL> "Shell to generate completions for")
                        .value_parser(clap::value_parser!(Shell))
                        .required(true),
                ),
        )
}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE> "Stash message")]
}

fn required<'a>(matches: &'a clap::ArgMatches, id: &'static str) -> Result<&'a str> {
    matches
        .get_one::<String>(id)
        .map(String::as_str)
        .ok_or_else(|| Error::MissingArgument(id.to_string()))
}

fn dispatch(matches: clap::ArgMatches) -> Result<String> {
    match matches.subcommand() {
        Some(("init", sub)) => sampo::init_repo(required(sub, "NAME")?),
        Some(("clone", sub)) => sampo::clone_repo(required(sub, "REMOTE")?),
        Some(("diff", sub)) => {
            let color = sub
                .get_one::<String>("color")
                .map(String::as_str)
                .unwrap_or("auto");
            let mut base = sub.get_one::<String>("base").map(String::as_str);
            let mut head = sub.get_one::<String>("head").map(String::as_str);
            let mut path = sub.get_one::<String>("path").map(String::as_str);
            if path.is_none() {
                path = head.take();
                if path.is_none() {
                    path = base.take();
                }
            }
            Ok(sampo::diff(
                base.unwrap_or("stage"),
                head.unwrap_or("worktree"),
                path.unwrap_or(""),
                color,
            ))
        }
        Some(("push", sub)) => Ok(sampo::push(required(sub, "REMOTE")?)),
        Some(("add", sub)) => {
            let paths: Vec<PathBuf> = sub
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .cloned()
                .collect();
            sampo::add_paths(&paths)
        }
        Some(("commit", sub)) => Ok(sampo::commit(required(sub, "message")?)),
        Some(("status", _)) => Ok(sampo::status()),
        Some(("stash", sub)) => {
            let stash_command = sub.subcommand().unwrap_or(("push", sub));
            match stash_command {
                ("apply", s) => {
                    let stash = s.get_one::<String>("STASH").map(String::as_str);
                    Ok(match stash {
                        Some(s) => format!("Applied stash: {s}"),
                        None => "Applied latest stash".into(),
                    })
                }
                ("pop", s) => {
                    let stash = s.get_one::<String>("STASH").map(String::as_str);
                    Ok(match stash {
                        Some(s) => format!("Popped stash: {s}"),
                        None => "Popped latest stash".into(),
                    })
                }
                ("push", s) => Ok(match s.get_one::<String>("message").map(String::as_str) {
                    Some(msg) => format!("Stashed with message: \"{msg}\""),
                    None => "Stashed working directory changes".into(),
                }),
                ("list", _) => Ok("No stashes found".into()),
                (name, _) => Err(Error::MissingArgument(name.to_string())),
            }
        }
        Some(("log", sub)) => Ok(sampo::log(required(sub, "max-count")?)),
        Some(("completions", sub)) => {
            let shell = sub
                .get_one::<Shell>("shell")
                .ok_or_else(|| Error::MissingArgument("shell".to_string()))?;
            let mut cmd = cli();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            Ok(String::from_utf8_lossy(&out).into_owned())
        }
        Some((ext, sub)) => {
            let args: Vec<OsString> = sub
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .cloned()
                .collect();
            Ok(sampo::external(ext, &args))
        }
        // `subcommand_required(true)` makes this unreachable.
        None => Err(Error::MissingArgument("subcommand".to_string())),
    }
}

fn main() -> ExitCode {
    let matches = cli().get_matches();
    match dispatch(matches) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
