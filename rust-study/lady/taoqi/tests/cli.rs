use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;

fn bin() -> Command {
    Command::cargo_bin("taoqi").expect("binary builds")
}

fn write_json(tag: &str, content: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("taoqi-test-{}-{tag}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("data.json");
    fs::write(&file, content).unwrap();
    file
}

#[test]
fn validate_accepts_valid_json() {
    let file = write_json("valid", r#"{"name": "tq"}"#);
    bin()
        .args(["validate", file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "is valid JSON (root type: object)",
        ));

    fs::remove_dir_all(file.parent().unwrap()).ok();
}

#[test]
fn validate_rejects_invalid_json_with_nonzero_exit() {
    let file = write_json("invalid", "{ not json }");
    bin()
        .args(["validate", file.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicates::str::contains("is NOT valid JSON"));

    fs::remove_dir_all(file.parent().unwrap()).ok();
}

#[test]
fn format_prints_pretty_json() {
    let file = write_json("format", r#"{"a":1,"b":[1,2]}"#);
    bin()
        .args(["format", file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("\"b\": [\n"));

    fs::remove_dir_all(file.parent().unwrap()).ok();
}

#[test]
fn format_in_place_rewrites_file() {
    let file = write_json("inplace", r#"{"a":1}"#);
    bin()
        .args(["format", file.to_str().unwrap(), "--in-place"])
        .assert()
        .success()
        .stdout(predicates::str::contains("in-place"));

    let rewritten = fs::read_to_string(&file).unwrap();
    assert!(rewritten.contains("{\n  \"a\": 1\n}"));

    fs::remove_dir_all(file.parent().unwrap()).ok();
}

#[test]
fn minify_removes_whitespace() {
    let file = write_json("minify", "{\n  \"a\": 1\n}\n");
    bin()
        .args(["minify", file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("{\"a\":1}"));

    fs::remove_dir_all(file.parent().unwrap()).ok();
}

#[test]
fn query_finds_nested_value() {
    let file = write_json(
        "query",
        r#"{"name":"tq","address":{"city":"Bj"},"items":[1,2]}"#,
    );
    bin()
        .args(["query", file.to_str().unwrap(), "address.city"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Bj"));

    bin()
        .args(["query", file.to_str().unwrap(), "items.0"])
        .assert()
        .success()
        .stdout(predicates::str::contains("1"));

    fs::remove_dir_all(file.parent().unwrap()).ok();
}

#[test]
fn query_missing_key_fails() {
    let file = write_json("qmiss", r#"{"a":1}"#);
    bin()
        .args(["query", file.to_str().unwrap(), "b"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("key `b` not found"));

    fs::remove_dir_all(file.parent().unwrap()).ok();
}

#[test]
fn cli_help_and_version() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("JSON processor"));
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
