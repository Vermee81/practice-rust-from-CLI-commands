use assert_cmd::Command;
use predicates::prelude::*;

// 引数に文字列を指定して実行して、その文字列が標準出力に出力されることを確認
#[test]
fn runs() {
    let mut cmd: Command = Command::cargo_bin("catecho").unwrap();
    cmd.arg("Hello").assert().success().stdout("Hello MEOW\n");
}

// 引数なしで実行してUSAGEが標準エラーに出力されることを確認
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("catecho").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
}