use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("phoebe").expect("binary builds")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("phoebe-test-{}-{tag}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn csv_conversion_end_to_end() {
    let dir = temp_dir("csv");
    let input = dir.join("in.csv");
    let output = dir.join("out.json");
    fs::write(&input, "name,age\nalice,30\nbob,25\n").unwrap();

    bin()
        .args([
            "csv",
            "--input",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Converted 2 records"));

    let json = fs::read_to_string(&output).unwrap();
    assert!(json.contains(r#""name":"alice""#));
    assert!(json.contains(r#""age":"25""#));

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn csv_without_headers_uses_generated_columns() {
    let dir = temp_dir("noheader");
    let input = dir.join("in.csv");
    let output = dir.join("out.json");
    fs::write(&input, "alice,30\n").unwrap();

    bin()
        .args([
            "csv",
            "--input",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--header=false",
        ])
        .assert()
        .success();

    let json = fs::read_to_string(&output).unwrap();
    assert!(json.contains(r#""column_0":"alice""#));

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn csv_missing_input_fails_with_clear_error() {
    bin()
        .args(["csv", "--input", "no-such-file.csv"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot open"));
}

#[test]
fn install_simulates_package() {
    let dir = temp_dir("install"); // install writes to ./node_modules relative to cwd

    bin()
        .args(["install", "left-pad"])
        .current_dir(&dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Installed 'left-pad'"));

    assert!(dir.join("node_modules").exists());
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn cli_help_smoke() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("CSV processor"));
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
