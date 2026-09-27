use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("httpman").expect("binary builds")
}

#[test]
fn cli_help_smoke() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("HTTP client CLI"));
}

#[test]
fn cli_version_smoke() {
    bin()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("httpman"));
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
fn get_rejects_invalid_url() {
    bin().args(["get", "not a url"]).assert().failure().stderr(
        predicate::str::contains("relative URL without a base")
            .or(predicate::str::contains("Error")),
    );
}

#[test]
fn completions_generation() {
    bin()
        .args(["completions", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("httpman"));
}
