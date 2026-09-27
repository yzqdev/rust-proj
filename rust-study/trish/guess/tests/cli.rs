use std::fs;

use assert_cmd::Command;
use guess::core_ops;

fn bin() -> Command {
    Command::cargo_bin("guess").expect("binary builds")
}

#[test]
fn hash_matches_known_vector() {
    bin()
        .args(["hash", "hello"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "5d41402abc4b2a76b9719d911017c592",
        ));
}

#[test]
fn random_respects_bounds() {
    for _ in 0..20 {
        let out = bin()
            .args(["random", "--min", "5", "--max", "10"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        let text = String::from_utf8_lossy(&out);
        let value: i32 = text
            .rsplit(':')
            .next()
            .unwrap_or("0")
            .trim()
            .parse()
            .unwrap_or(0);
        assert!((5..=10).contains(&value), "{text}");
    }
}

#[test]
fn read_displays_file() {
    let dir = std::env::temp_dir().join(format!("guess-test-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("note.txt");
    fs::write(&file, "guess-content").unwrap();

    bin()
        .args(["read", file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("guess-content"));

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn read_missing_file_fails() {
    bin()
        .args(["read", "no-such-file.txt"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("cannot read"));
}

#[test]
fn info_reports_system() {
    bin()
        .arg("info")
        .assert()
        .success()
        .stdout(predicates::str::contains("OS:"));
}

#[test]
fn fetch_invalid_url_fails() {
    bin()
        .args(["fetch", "not a url"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error"));
}

#[test]
fn cli_help_and_version() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("A multi-purpose CLI tool"));
    bin().arg("--version").assert().success();
}

#[test]
fn completions_generation() {
    bin()
        .args(["completions", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("guess"));
}

#[test]
fn md5_helper_matches() {
    assert_eq!(
        core_ops::hash_md5("hello"),
        "5d41402abc4b2a76b9719d911017c592"
    );
}
