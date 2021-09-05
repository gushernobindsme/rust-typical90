// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    // 現在地点と目標地点の差分を出す
    let mut diff = 0;
    for i in 0..n {
        diff += (a[i] - b[i]).abs();
    }

    println!("{}", if solve(diff, k) { "Yes" } else { "No" });
}

fn solve(diff: i64, k: i64) -> bool {
    // 差分が操作回数よりも多い場合、NG
    if diff > k {
        return false;
    }

    // 差分と操作回数の偶奇が一致していない場合、NG
    if diff % 2 != k % 2 {
        return false;
    }

    return true;
}
