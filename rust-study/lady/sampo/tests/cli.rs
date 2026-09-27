use assert_cmd::Command;

fn bin() -> Command {
    Command::cargo_bin("sampo").expect("binary builds")
}

#[test]
fn cli_init_creates_repository() {
    let dir = std::env::temp_dir().join(format!("sampo-e2e-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();

    bin()
        .args(["init", "proj"])
        .current_dir(&dir)
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "Initialized empty repository: .proj",
        ));

    assert!(dir.join(".proj").join("README.md").exists());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn cli_status_smoke() {
    bin()
        .arg("status")
        .assert()
        .success()
        .stdout(predicates::str::contains("On branch main"));
}

#[test]
fn cli_commit_requires_message() {
    bin()
        .arg("commit")
        .assert()
        .failure()
        .stderr(predicates::str::contains("--message"));
}

#[test]
fn cli_add_missing_path_fails() {
    bin()
        .args(["add", "no-such-path.txt"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("path not found"));
}

#[test]
fn cli_external_subcommand_is_reported() {
    bin()
        .args(["remote", "add", "origin"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Calling out to \"remote\""));
}

#[test]
fn cli_help_and_version() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("A fictional versioning CLI"));
    bin().arg("--version").assert().success();
}

#[test]
fn completions_generation() {
    bin()
        .args(["completions", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("sampo"));
}
