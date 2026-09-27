use assert_cmd::Command;

fn bin() -> Command {
    Command::cargo_bin("minigrep").expect("binary builds")
}

#[test]
fn cli_help_and_version() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("grep"));
    bin().arg("--version").assert().success();
}

#[test]
fn search_finds_matches() {
    bin()
        .args(["is", "./test-data/test.txt"])
        .assert()
        .success()
        .stdout(predicates::str::contains("is"));
}

#[test]
fn count_flag_reports_matches() {
    bin()
        .args(["--count", "is", "./test-data/test.txt"])
        .assert()
        .success()
        .stdout(predicates::str::contains("match(es)"));
}

#[test]
fn missing_file_fails() {
    bin()
        .args(["query", "./test-data/does-not-exist.txt"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("minigrep:"));
}
