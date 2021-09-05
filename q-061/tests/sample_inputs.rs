use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6
1 2
1 1
2 3
3 1
3 2
3 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1
2
3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6
2 1
3 1
2 2
3 1
2 3
3 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1
1
1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6
1 1000000000
2 200000000
1 30000000
2 4000000
1 500000
3 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000000000\n");
    assert!(output.stderr_str().is_empty());
}
