use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// pela - A fictional versioning CLI
///
/// Simulates Git-like version control operations.
#[derive(Debug, Parser)]
#[command(name = "pela")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init {
        /// Name of the project
        name: String,
    },
    /// Clone a repository
    #[command(arg_required_else_help = true)]
    Clone {
        /// The remote to clone
        remote: String,
    },
    /// Compare two commits or files
    Diff {
        #[arg(value_name = "COMMIT")]
        base: Option<OsString>,
        #[arg(value_name = "COMMIT")]
        head: Option<OsString>,
        #[arg(last = true)]
        path: Option<OsString>,
        #[arg(
            long,
            require_equals = true,
            value_name = "WHEN",
            num_args = 0..=1,
            default_value_t = ColorWhen::Auto,
            default_missing_value = "always",
            value_enum
        )]
        color: ColorWhen,
    },
    /// Push changes to a remote
    #[command(arg_required_else_help = true)]
    Push {
        /// The remote to target
        remote: String,
    },
    /// Stage files
    #[command(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    /// Commit staged changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
    },
    /// List the status of working tree
    Status,
    /// Stash changes
    Stash(StashArgs),
    /// Log commit history
    Log {
        /// Max log entries
        #[arg(short, long, default_value_t = 10)]
        max_count: u32,
    },
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl std::fmt::Display for ColorWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct StashArgs {
    #[command(subcommand)]
    command: Option<StashCommands>,
    #[command(flatten)]
    push: StashPushArgs,
}

#[derive(Debug, Subcommand)]
enum StashCommands {
    Push(StashPushArgs),
    Pop { stash: Option<String> },
    Apply { stash: Option<String> },
    List,
}

#[derive(Debug, Args)]
struct StashPushArgs {
    #[arg(short, long)]
    message: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init { name } => {
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
        Commands::Clone { remote } => {
            let dir_name = remote
                .split('/')
                .last()
                .unwrap_or(&remote)
                .trim_end_matches(".git");
            match fs::create_dir_all(dir_name) {
                Ok(_) => println!("Cloned '{}' into '{}'", remote, dir_name),
                Err(e) => eprintln!("Error cloning: {}", e),
            }
        }
        Commands::Diff {
            mut base,
            mut head,
            mut path,
            color,
        } => {
            if path.is_none() {
                path = head;
                head = None;
                if path.is_none() {
                    path = base;
                    base = None;
                }
            }
            let base = base
                .as_deref()
                .map(|s| s.to_str().unwrap())
                .unwrap_or("stage");
            let head = head
                .as_deref()
                .map(|s| s.to_str().unwrap())
                .unwrap_or("worktree");
            let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
            println!(
                "Diffing {}..{} {} (color={})",
                base,
                head,
                path.to_string_lossy(),
                color
            );
        }
        Commands::Push { remote } => {
            println!("Pushing to '{}' (simulated)", remote);
        }
        Commands::Add { path } => {
            for p in &path {
                if p.exists() {
                    println!("Staged: {}", p.display());
                } else {
                    eprintln!("Path not found: {}", p.display());
                }
            }
        }
        Commands::Commit { message } => {
            println!("Committed with message: \"{}\"", message);
        }
        Commands::Status => {
            println!("On branch main");
            println!("Nothing to commit, working tree clean");
        }
        Commands::Log { max_count } => {
            println!("Showing last {} commits (simulated)", max_count);
            println!("commit a1b2c3d4e5f6... (HEAD -> main)");
            println!("    Initial commit");
        }
        Commands::Stash(stash) => {
            let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
            match stash_cmd {
                StashCommands::Push(push) => {
                    if let Some(msg) = &push.message {
                        println!("Stashed with message: \"{}\"", msg);
                    } else {
                        println!("Stashed working directory changes");
                    }
                }
                StashCommands::Pop { stash } => {
                    if let Some(s) = stash {
                        println!("Popped stash: {}", s);
                    } else {
                        println!("Popped latest stash");
                    }
                }
                StashCommands::Apply { stash } => {
                    if let Some(s) = stash {
                        println!("Applied stash: {}", s);
                    } else {
                        println!("Applied latest stash");
                    }
                }
                StashCommands::List => {
                    println!("No stashes found");
                }
            }
        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }
}
