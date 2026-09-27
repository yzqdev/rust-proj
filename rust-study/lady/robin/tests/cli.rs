use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("robin").expect("binary builds")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("robin-test-{}-{tag}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn md5_of_file_with_known_hash() {
    let dir = temp_dir("md5");
    let file = dir.join("data.txt");
    fs::write(&file, b"hello").unwrap();

    bin()
        .args(["md5", file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "5d41402abc4b2a76b9719d911017c592",
        ));

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn md5_missing_file_fails() {
    bin()
        .args(["md5", "no-such-file.bin"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("file not found"));
}

#[test]
fn info_reports_file_details() {
    let dir = temp_dir("info");
    let file = dir.join("notes.txt");
    fs::write(&file, b"content").unwrap();

    bin()
        .args(["info", file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(
            predicates::str::contains("Type:     file")
                .and(predicates::str::contains("Size:     7 bytes")),
        );

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn tree_lists_entries() {
    let dir = temp_dir("tree");
    fs::create_dir_all(dir.join("sub")).unwrap();
    fs::write(dir.join("a.txt"), b"a").unwrap();
    fs::write(dir.join("sub").join("b.txt"), b"bb").unwrap();

    let out = bin()
        .args(["tree", dir.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Directory tree"))
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8_lossy(&out);
    assert!(text.contains("a.txt"), "{text}");
    assert!(text.contains("sub/"), "{text}");

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn tree_rejects_regular_file() {
    let dir = temp_dir("treefile");
    let file = dir.join("plain.txt");
    fs::write(&file, b"x").unwrap();

    bin()
        .args(["tree", file.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicates::str::contains("not a valid directory"));

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn cli_help_and_version() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("File utility CLI"));
    bin().arg("--version").assert().success();
}

#[test]
fn cli_invalid_subcommand_fails() {
    bin()
        .arg("frobnicate")
        .assert()
        .failure()
        .stderr(predicates::str::contains("unrecognized"));
}
