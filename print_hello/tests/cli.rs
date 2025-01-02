use assert_cmd::Command;

#[test]
fn hello_ok(){
    let mut cmd = Command::cargo_bin("print_hello").unwrap();
    cmd.assert().success().stdout("Hello\n");
}
