use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use asta::{Cli, Commands, FileAlgorithm, run};
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("asta").expect("binary builds")
}

#[test]
fn md5_of_hello() {
    let cli = Cli {
        command: Commands::Md5 {
            text: "hello".into(),
        },
    };
    let out = run(&cli).unwrap();
    assert_eq!(out, "MD5(\"hello\") = 5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn sha256_of_hello() {
    let cli = Cli {
        command: Commands::Sha256 {
            text: "hello".into(),
        },
    };
    let out = run(&cli).unwrap();
    assert_eq!(
        out,
        "SHA256(\"hello\") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn file_md5_defaults_to_md5() {
    let dir = std::env::temp_dir().join(format!("asta-test-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("sample.txt");
    fs::write(&path, b"hello").unwrap();

    let cli = Cli {
        command: Commands::File {
            path: path.clone(),
            algorithm: FileAlgorithm::Md5,
        },
    };
    let out = run(&cli).unwrap();
    assert!(out.ends_with("5d41402abc4b2a76b9719d911017c592"), "{out}");

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn missing_file_is_an_error() {
    let cli = Cli {
        command: Commands::File {
            path: PathBuf::from("no-such-file-xyz.bin"),
            algorithm: FileAlgorithm::Md5,
        },
    };
    let err = run(&cli).unwrap_err().to_string();
    assert!(err.contains("no-such-file-xyz.bin"), "{err}");
}

#[test]
fn cli_help_smoke() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hash calculator"));
}

#[test]
fn cli_version_smoke() {
    bin().arg("--version").assert().success();
}

#[test]
fn cli_invalid_subcommand_fails() {
    bin()
        .arg("frobnicate")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized"));
}

#[test]
fn cli_md5_matches_expected_hash() {
    bin()
        .args(["md5", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn completions_generation() {
    let cli = Cli {
        command: Commands::Completions {
            shell: clap_complete::Shell::Bash,
        },
    };
    let out = run(&cli).unwrap();
    assert!(
        out.contains("asta"),
        "completions should mention the binary"
    );
}
