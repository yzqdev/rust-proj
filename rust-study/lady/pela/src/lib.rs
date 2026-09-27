//! pela - A fictional versioning CLI (clap **derive API** variant).
//!
//! Sister project of `sampo`, which implements the same CLI with the clap
//! **builder API**. Both are kept as teaching examples of the two styles.
//! Operations are simulated; `init` and `clone` do touch the filesystem.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::CommandFactory;
use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

/// pela - A fictional versioning CLI
///
/// Simulates Git-like version control operations.
#[derive(Debug, Parser)]
#[command(
    name = "pela",
    version,
    author,
    about = "A fictional versioning CLI",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
        base: Option<String>,
        #[arg(value_name = "COMMIT")]
        head: Option<String>,
        #[arg(last = true)]
        path: Option<String>,
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
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(short, long, value_enum)]
        shell: Shell,
    },
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorWhen {
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

#[derive(Debug, Clone, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct StashArgs {
    #[command(subcommand)]
    pub command: Option<StashCommands>,
    #[command(flatten)]
    pub push: StashPushArgs,
}

#[derive(Debug, Clone, Subcommand)]
pub enum StashCommands {
    Push(StashPushArgs),
    Pop { stash: Option<String> },
    Apply { stash: Option<String> },
    List,
}

#[derive(Debug, Clone, Args)]
pub struct StashPushArgs {
    #[arg(short, long)]
    pub message: Option<String>,
}

/// Errors reported by the CLI layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A staged path does not exist.
    #[error("path not found: {}", .0.display())]
    PathNotFound(PathBuf),

    /// Filesystem operation failed.
    #[error("{action}: {source}")]
    Io {
        action: &'static str,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

/// Execute a parsed command and return the text to print on stdout.
pub fn run(cli: &Cli) -> Result<String> {
    match &cli.command {
        Commands::Init { name } => init_repo(name),
        Commands::Clone { remote } => clone_repo(remote),
        Commands::Diff {
            base,
            head,
            path,
            color,
        } => {
            let mut base = base.clone();
            let mut head = head.clone();
            let mut path = path.clone();
            if path.is_none() {
                path = head.take();
                if path.is_none() {
                    path = base.take();
                }
            }
            let base = base.as_deref().unwrap_or("stage");
            let head = head.as_deref().unwrap_or("worktree");
            let path = path.as_deref().unwrap_or("");
            Ok(format!(
                "Diffing {base}..{head} {path} (color={color})",
                base = base,
                head = head,
                path = path,
                color = color
            ))
        }
        Commands::Push { remote } => Ok(format!("Pushing to '{remote}' (simulated)")),
        Commands::Add { path } => {
            let mut lines = Vec::new();
            for p in path {
                if !p.exists() {
                    return Err(Error::PathNotFound(p.clone()));
                }
                lines.push(format!("Staged: {}", p.display()));
            }
            Ok(lines.join("\n"))
        }
        Commands::Commit { message } => Ok(format!("Committed with message: \"{message}\"")),
        Commands::Status => Ok("On branch main\nNothing to commit, working tree clean".into()),
        Commands::Log { max_count } => Ok(format!(
            "Showing last {max_count} commits (simulated)\ncommit a1b2c3d4e5f6... (HEAD -> main)\n    Initial commit"
        )),
        Commands::Stash(stash) => {
            let stash_cmd = stash
                .command
                .clone()
                .unwrap_or(StashCommands::Push(clone_push(&stash.push)));
            match stash_cmd {
                StashCommands::Push(push) => match &push.message {
                    Some(msg) => Ok(format!("Stashed with message: \"{msg}\"")),
                    None => Ok("Stashed working directory changes".into()),
                },
                StashCommands::Pop { stash } => match stash {
                    Some(s) => Ok(format!("Popped stash: {s}")),
                    None => Ok("Popped latest stash".into()),
                },
                StashCommands::Apply { stash } => match stash {
                    Some(s) => Ok(format!("Applied stash: {s}")),
                    None => Ok("Applied latest stash".into()),
                },
                StashCommands::List => Ok("No stashes found".into()),
            }
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            Ok(String::from_utf8_lossy(&out).into_owned())
        }
        Commands::External(args) => match args.split_first() {
            Some((name, rest)) => Ok(format!("Calling out to {name:?} with {rest:?}")),
            None => Ok("No external command given".into()),
        },
    }
}

fn clone_push(push: &StashPushArgs) -> StashPushArgs {
    StashPushArgs {
        message: push.message.clone(),
    }
}

fn init_repo(name: &str) -> Result<String> {
    let dir = PathBuf::from(format!(".{name}"));
    fs::create_dir_all(&dir).map_err(|source| Error::Io {
        action: "failed to create repository directory",
        source,
    })?;
    let mut readme = fs::File::create(dir.join("README.md")).map_err(|source| Error::Io {
        action: "failed to create repository README",
        source,
    })?;
    writeln!(readme, "# {name}").map_err(|source| Error::Io {
        action: "failed to write repository README",
        source,
    })?;
    Ok(format!("Initialized empty repository: {}", dir.display()))
}

fn clone_repo(remote: &str) -> Result<String> {
    let dir_name = remote
        .split('/')
        .next_back()
        .unwrap_or(remote)
        .trim_end_matches(".git");
    let dir = Path::new(dir_name);
    fs::create_dir_all(dir).map_err(|source| Error::Io {
        action: "failed to create clone directory",
        source,
    })?;
    Ok(format!("Cloned '{remote}' into '{dir_name}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_creates_directory_and_readme() {
        let dir = std::env::temp_dir().join(format!("pela-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let out = init_repo("demo").unwrap();
        assert!(out.contains(".demo"));
        assert!(dir.join(".demo").join("README.md").exists());
        let readme = std::fs::read_to_string(dir.join(".demo").join("README.md")).unwrap();
        assert_eq!(readme.trim(), "# demo");

        std::env::set_current_dir(prev).unwrap();
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn clone_derives_directory_name() {
        assert_eq!(
            clone_repo("https://example.com/group/repo.git").unwrap(),
            "Cloned 'https://example.com/group/repo.git' into 'repo'"
        );
    }

    #[test]
    fn status_reports_clean_tree() {
        let cli = Cli {
            command: Commands::Status,
        };
        let out = run(&cli).unwrap();
        assert!(out.contains("working tree clean"));
    }

    #[test]
    fn add_missing_path_is_an_error() {
        let cli = Cli {
            command: Commands::Add {
                path: vec![PathBuf::from("definitely-missing.txt")],
            },
        };
        let err = run(&cli).unwrap_err();
        assert!(err.to_string().contains("path not found"));
    }
}
