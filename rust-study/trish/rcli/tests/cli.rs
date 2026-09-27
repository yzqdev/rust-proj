use assert_cmd::Command;

fn bin() -> Command {
    Command::cargo_bin("rcli").expect("binary builds")
}

#[test]
fn encode_decode_roundtrip() {
    let output = bin()
        .args(["encode", "hello world"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let encoded = String::from_utf8_lossy(&output);
    assert!(encoded.contains("aGVsbG8gd29ybGQ="), "{encoded}");

    bin()
        .args(["decode", "aGVsbG8gd29ybGQ="])
        .assert()
        .success()
        .stdout(predicates::str::contains("Decoded: hello world"));
}

#[test]
fn decode_invalid_input_fails() {
    bin()
        .args(["decode", "not*valid"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error"));
}

#[test]
fn count_text_and_file() {
    bin()
        .args(["count", "--", "one two three"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Words: 3"));
}

#[test]
fn test_subcommand_smoke() {
    bin()
        .args(["test", "--list"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Printing testing lists"));
}

#[test]
fn cli_help_and_version() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage"));
    bin().arg("--version").assert().success();
}

#[test]
fn completions_generation() {
    bin()
        .args(["completions", "--shell", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("rcli"));
}
