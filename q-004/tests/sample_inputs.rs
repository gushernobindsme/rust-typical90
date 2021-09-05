use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 3
1 1 1
1 1 1
1 1 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "5 5 5
5 5 5
5 5 5\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4 4
3 1 4 1
5 9 2 6
5 3 5 8
9 7 9 3
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "28 28 25 26
39 33 40 34
38 38 36 31
41 41 39 43\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 10
31 41 59 26 53 58 97 93 23 84
62 64 33 83 27 95 2 88 41 97
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "627 629 598 648 592 660 567 653 606 662
623 633 651 618 645 650 689 685 615 676\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10 10
83 86 77 65 93 85 86 92 99 71
62 77 90 59 63 76 90 76 72 86
61 68 67 79 82 80 62 73 67 85
79 52 72 58 69 67 93 56 61 92
79 73 71 69 84 87 98 74 65 70
63 76 91 80 56 73 62 70 96 81
55 75 84 77 86 55 96 79 63 57
74 95 82 95 64 67 84 64 93 50
87 58 76 78 88 84 53 51 54 99
82 60 76 68 89 62 76 86 94 89
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "1479 1471 1546 1500 1518 1488 1551 1466 1502 1546
1414 1394 1447 1420 1462 1411 1461 1396 1443 1445
1388 1376 1443 1373 1416 1380 1462 1372 1421 1419
1345 1367 1413 1369 1404 1368 1406 1364 1402 1387
1416 1417 1485 1429 1460 1419 1472 1417 1469 1480
1410 1392 1443 1396 1466 1411 1486 1399 1416 1447
1397 1372 1429 1378 1415 1408 1431 1369 1428 1450
1419 1393 1472 1401 1478 1437 1484 1425 1439 1498
1366 1390 1438 1378 1414 1380 1475 1398 1438 1409
1425 1442 1492 1442 1467 1456 1506 1417 1452 1473\n"
    );
    assert!(output.stderr_str().is_empty());
}
