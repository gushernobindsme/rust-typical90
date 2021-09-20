use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"227
21 47 56
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"9999
1 5 10
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1004\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"998244353
314159 265358 97932
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3333\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"100000000
10001 10002 10003
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9998\n");
    assert!(output.stderr_str().is_empty());
}
