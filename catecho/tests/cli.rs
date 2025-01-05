use assert_cmd::Command;
use predicates::prelude::*;

// 引数なしで実行してUSAGEが標準エラーに出力されることを確認
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("catecho").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
}