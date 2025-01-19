use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

const PROGRAM_NAME: &str = "concat";
const EMPTY_FILE: &str = "tests/inputs/empty.txt";
const ONE_LINE_FILE: &str = "tests/inputs/one-line.txt";
const MULTIPLE_LINES_FILE: &str = "tests/inputs/multiple-lines.txt";
const MULTIPLE_WITH_BLANKS_FILE: &str = "tests/inputs/multiple-with-blank-lines.txt";

#[test]
fn show_usage() -> Result<()> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PROGRAM_NAME)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PROGRAM_NAME)?
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn empty() -> Result<()> {
    run(&[EMPTY_FILE], "tests/expected/empty.txt.out")
}

#[test]
fn empty_b() -> Result<()> {
    run(&["-b", EMPTY_FILE], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n() -> Result<()> {
    run(&["-n", EMPTY_FILE], "tests/expected/empty.txt.out")
}

#[test]
fn one_line() -> Result<()> {
    run(&[ONE_LINE_FILE], "tests/expected/one-line.txt.out")
}

#[test]
fn one_line_b() -> Result<()> {
    run(&["-b", ONE_LINE_FILE], "tests/expected/one-line.txt.b.out")
}

#[test]
fn one_line_n() -> Result<()> {
    run(&["-n", ONE_LINE_FILE], "tests/expected/one-line.txt.n.out")
}

#[test]
fn multiple_lines() -> Result<()> {
    run(
        &[MULTIPLE_LINES_FILE],
        "tests/expected/multiple-lines.txt.out",
    )
}

#[test]
fn multiple_lines_b() -> Result<()> {
    run(
        &["-b", MULTIPLE_LINES_FILE],
        "tests/expected/multiple-lines.txt.b.out",
    )
}

#[test]
fn multiple_lines_n() -> Result<()> {
    run(
        &["-n", MULTIPLE_LINES_FILE],
        "tests/expected/multiple-lines.txt.n.out",
    )
}

#[test]
fn multiple_with_blanks() -> Result<()> {
    run(
        &[MULTIPLE_WITH_BLANKS_FILE],
        "tests/expected/multiple-with-blanks.txt.out",
    )
}

#[test]
fn multiple_with_blanks_b() -> Result<()> {
    run(
        &["-b", MULTIPLE_WITH_BLANKS_FILE],
        "tests/expected/multiple-with-blanks.txt.b.out",
    )
}

#[test]
fn multiple_with_blanks_n() -> Result<()> {
    run(
        &["-n", MULTIPLE_WITH_BLANKS_FILE],
        "tests/expected/multiple-with-blanks.txt.n.out",
    )
}
