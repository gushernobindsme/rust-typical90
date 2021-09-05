use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"21 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "15\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1330 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "555\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2311640221315 15
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "474547\n");
    assert!(output.stderr_str().is_empty());
}
