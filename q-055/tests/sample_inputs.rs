use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6 7 1
1 2 3 4 5 6
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10 1 0
0 0 0 0 0 0 0 0 0 0
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "252\n");
    assert!(output.stderr_str().is_empty());
}
