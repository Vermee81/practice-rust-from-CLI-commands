use assert_cmd::Command;
use predicates::prelude::*;

const BIN_NAME: &str = "catecho";

// 引数に文字列を指定して実行して、その文字列が標準出力に出力されることを確認
#[test]
fn runs() {
    let mut cmd: Command = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("Hello").assert().success().stdout("Hello MEOW\n");
}

// 文字が2つ以上ある場合に、それらが連結されて標準出力に出力されることを確認
#[test]
fn runs_multiple() {
    let mut cmd: Command = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("Hello").arg("World").assert().success().stdout("Hello World MEOW\n");
}

// -nオプションを指定して実行して、改行がないことを確認
#[test]
fn runs_no_newline() {
    let mut cmd: Command = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("-n").arg("Hello").arg("World").assert().success().stdout("Hello World MEOW");
}

// 引数なしで実行してUSAGEが標準エラーに出力されることを確認
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
}