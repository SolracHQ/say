use assert_cmd::Command;
use predicates::prelude::predicate;
use std::process::Command as CMD;

static MESSAGE: &str = "\\nHello \\v\\t World!\\a\\c please stop";
static HEX_ESCAPED: &str = "Hello, \\x1b[31mWorld!\\x1b[0m";

/// Test that `say` correctly outputs a message with newlines and other
/// escape characters when given no flags.
#[test]
fn say_new_line_no_interpretation() {
    let echo = CMD::new("echo")
        .arg(MESSAGE)
        .output()
        .expect("failed running echo");

    let mut say = Command::cargo_bin("say").unwrap();
    say.arg(MESSAGE)
        .assert()
        .success()
        .stdout(predicate::eq(echo.stdout.as_slice()));
}

/// Test that `say` correctly outputs a message without newlines or other
/// escape characters when given the `-n` flag.
#[test]
fn say_no_new_line_no_interpretation() {
    let echo = CMD::new("echo")
        .arg("-n")
        .arg(MESSAGE)
        .output()
        .expect("failed running echo");

    let mut say = Command::cargo_bin("say").unwrap();
    say.arg("-n")
        .arg(MESSAGE)
        .assert()
        .success()
        .stdout(predicate::eq(echo.stdout.as_slice()));
}

/// Test that `say` correctly interprets escape characters and outputs a
/// message with newlines and other escape characters when given the `-e` flag.
#[test]
fn say_new_line_interpretation() {
    let echo = CMD::new("echo")
        .arg("-e")
        .arg(MESSAGE)
        .output()
        .expect("failed running echo");

    let mut say = Command::cargo_bin("say").unwrap();
    say.arg("-e")
        .arg(MESSAGE)
        .assert()
        .success()
        .stdout(predicate::eq(echo.stdout.as_slice()));
}

/// Test that `say` correctly interprets escape characters and outputs a
/// message without newlines or other escape characters when given both the
/// `-e` and `-n` flags.
#[test]
fn say_no_new_line_interpretation() {
    let echo = CMD::new("echo")
        .arg("-e")
        .arg("-n")
        .arg(MESSAGE)
        .output()
        .expect("failed running echo");

    let mut say = Command::cargo_bin("say").unwrap();
    say.arg("-e")
        .arg("-n")
        .arg(MESSAGE)
        .assert()
        .success()
        .stdout(predicate::eq(echo.stdout.as_slice()));
}

/// Test that `say` correctly interprets hexadecimal escape sequences and
/// outputs the corresponding characters.
#[test]
fn say_hex_interpretation() {
    let echo = CMD::new("echo")
        .arg("-e")
        .arg(HEX_ESCAPED)
        .output()
        .expect("failed running echo");

    let mut say = Command::cargo_bin("say").unwrap();
    say.arg("-e")
        .arg(HEX_ESCAPED)
        .assert()
        .success()
        .stdout(predicate::eq(echo.stdout.as_slice()));
}
