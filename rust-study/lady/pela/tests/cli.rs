use std::fs;

use assert_cmd::Command;
use pela::{Cli, Commands, run};
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("pela").expect("binary builds")
}

#[test]
fn diff_default_report() {
    let cli = Cli {
        command: Commands::Diff {
            base: None,
            head: None,
            path: None,
            color: pela::ColorWhen::Auto,
        },
    };
    let out = run(&cli).unwrap();
    assert_eq!(out, "Diffing stage..worktree  (color=auto)");
}

#[test]
fn stash_list() {
    let cli = Cli {
        command: Commands::Stash(pela::StashArgs {
            command: Some(pela::StashCommands::List),
            push: pela::StashPushArgs { message: None },
        }),
    };
    let out = run(&cli).unwrap();
    assert_eq!(out, "No stashes found");
}

#[test]
fn log_respects_max_count() {
    let cli = Cli {
        command: Commands::Log { max_count: 3 },
    };
    let out = run(&cli).unwrap();
    assert!(out.contains("Showing last 3 commits"));
}

#[test]
fn external_subcommand_is_reported() {
    let cli = Cli {
        command: Commands::External(vec!["remote".into(), "add".into(), "origin".into()]),
    };
    let out = run(&cli).unwrap();
    assert!(out.contains("Calling out to \"remote\""));
}

#[test]
fn cli_init_creates_repository() {
    let dir = std::env::temp_dir().join(format!("pela-e2e-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    bin()
        .args(["init", "proj"])
        .current_dir(&dir)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Initialized empty repository: .proj",
        ));

    assert!(dir.join(".proj").join("README.md").exists());
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn cli_status_smoke() {
    bin()
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("On branch main"));
}

#[test]
fn cli_add_missing_path_fails() {
    bin()
        .args(["add", "no-such-path.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("path not found"));
}

#[test]
fn cli_help_and_version() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A fictional versioning CLI"));
    bin().arg("--version").assert().success();
}

#[test]
fn cli_invalid_subcommand_is_external_or_fails() {
    // pela allows external subcommands, so an unknown command is reported,
    // not rejected.
    bin()
        .args(["frobnicate", "--flag"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Calling out to \"frobnicate\""));
}

#[test]
fn completions_generation() {
    let cli = Cli {
        command: Commands::Completions {
            shell: clap_complete::Shell::Zsh,
        },
    };
    let out = run(&cli).unwrap();
    assert!(out.contains("pela"));
}
