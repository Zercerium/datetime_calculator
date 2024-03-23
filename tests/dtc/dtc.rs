use assert_cmd::Command;

#[test]
fn test_add_duration_to_dmy_dot() {
    let assert = Command::cargo_bin("dtc")
        .unwrap()
        .arg("1.01.2024")
        .arg("1d")
        .assert();
    assert.stdout("02.01.2024\n");
}
