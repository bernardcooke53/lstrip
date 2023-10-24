use assert_cmd::Command;

#[test]
fn test_cli_output_1arg_no_ltrim_ws() {
    Command::cargo_bin("lstrip")
        .unwrap()
        .args(["# abc\n  123\n"])
        .assert()
        .success()
        .stdout(" abc\n  123\n");
}

#[test]
fn test_cli_output_1arg_no_ltrim_ws_extra_leading_ws() {
    Command::cargo_bin("lstrip")
        .unwrap()
        .args(["    # abc\n  123"])
        .assert()
        .success()
        .stdout(" abc\n  123\n");
}

#[test]
fn test_cli_output_1arg_ltrim_ws() {
    Command::cargo_bin("lstrip")
        .unwrap()
        .args(["# abc\n123\n", "-w"])
        .assert()
        .success()
        .stdout("abc\n123\n");
}

#[test]
fn test_cli_output_1arg_ltrim_ws_extra_leading_ws() {
    Command::cargo_bin("lstrip")
        .unwrap()
        .args(["    # abc\n  123", "-w"])
        .assert()
        .success()
        .stdout("abc\n123\n");
}
