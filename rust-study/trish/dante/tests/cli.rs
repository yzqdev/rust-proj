use assert_cmd::Command;
use dante::{Cli, Commands, run};

fn bin() -> Command {
    Command::cargo_bin("dante").expect("binary builds")
}

#[test]
fn calc_end_to_end() {
    bin()
        .args(["calc", "6", "--op", "mul", "7"])
        .assert()
        .success()
        .stdout(predicates::str::contains("6 mul 7 = 42"));
}

#[test]
fn calc_division_by_zero_fails() {
    bin()
        .args(["calc", "1", "--op", "div", "0"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("division by zero"));
}

#[test]
fn echo_cli() {
    bin()
        .args(["echo", "--upper", "hello", "world"])
        .assert()
        .success()
        .stdout(predicates::str::contains("HELLO WORLD"));
}

#[test]
fn now_cli() {
    bin().args(["now", "--format", "date"]).assert().success();
}

#[test]
fn cli_help_and_version() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("A versatile CLI tool"));
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

#[test]
fn completions_via_lib() {
    let cli = Cli {
        command: Commands::Completions {
            shell: clap_complete::Shell::Fish,
        },
    };
    assert!(run(&cli).unwrap().contains("dante"));
}
