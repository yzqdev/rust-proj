use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::{arg, Command};

fn cli() -> Command {
    Command::new("sampo")
        .about("A fictional versioning CLI (builder-style)")
        .version("1.0")
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
}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE> "Stash message")]
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            let dir = format!(".{}", name);
            match fs::create_dir_all(&dir) {
                Ok(_) => {
                    let mut readme = fs::File::create(format!("{}/README.md", dir)).unwrap();
                    writeln!(readme, "# {}", name).unwrap();
                    println!("Initialized empty repository: {}", dir);
                }
                Err(e) => eprintln!("Error creating repository: {}", e),
            }
        }
        Some(("clone", sub_matches)) => {
            let remote = sub_matches.get_one::<String>("REMOTE").expect("required");
            let dir_name = remote
                .split('/')
                .last()
                .unwrap_or(remote)
                .trim_end_matches(".git");
            match fs::create_dir_all(dir_name) {
                Ok(_) => println!("Cloned '{}' into '{}'", remote, dir_name),
                Err(e) => eprintln!("Error cloning: {}", e),
            }
        }
        Some(("diff", sub_matches)) => {
            let color = sub_matches
                .get_one::<String>("color")
                .map(|s| s.as_str())
                .expect("defaulted in clap");

            let mut base = sub_matches.get_one::<String>("base").map(|s| s.as_str());
            let mut head = sub_matches.get_one::<String>("head").map(|s| s.as_str());
            let mut path = sub_matches.get_one::<String>("path").map(|s| s.as_str());
            if path.is_none() {
                path = head;
                head = None;
                if path.is_none() {
                    path = base;
                    base = None;
                }
            }
            let base = base.unwrap_or("stage");
            let head = head.unwrap_or("worktree");
            let path = path.unwrap_or("");
            println!("Diffing {}..{} {} (color={})", base, head, path, color);
        }
        Some(("push", sub_matches)) => {
            let remote = sub_matches.get_one::<String>("REMOTE").expect("required");
            println!("Pushing to '{}' (simulated)", remote);
        }
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            for p in &paths {
                if p.exists() {
                    println!("Staged: {}", p.display());
                } else {
                    eprintln!("Path not found: {}", p.display());
                }
            }
        }
        Some(("commit", sub_matches)) => {
            let msg = sub_matches.get_one::<String>("message").expect("required");
            println!("Committed with message: \"{}\"", msg);
        }
        Some(("status", _)) => {
            println!("On branch main");
            println!("Nothing to commit, working tree clean");
        }
        Some(("stash", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
            match stash_command {
                ("apply", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Applying {:?}", stash);
                }
                ("pop", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Popping {:?}", stash);
                }
                ("push", sub_matches) => {
                    let message = sub_matches.get_one::<String>("message");
                    if let Some(msg) = message {
                        println!("Stashed with message: \"{}\"", msg);
                    } else {
                        println!("Stashed working directory changes");
                    }
                }
                ("list", _) => {
                    println!("No stashes found");
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        Some(("log", sub_matches)) => {
            let max_count = sub_matches
                .get_one::<String>("max-count")
                .map(|s| s.as_str())
                .unwrap_or("10");
            println!("Showing last {} commits (simulated)", max_count);
            println!("commit a1b2c3d4e5f6... (HEAD -> main)");
            println!("    Initial commit");
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(),
    }
}
