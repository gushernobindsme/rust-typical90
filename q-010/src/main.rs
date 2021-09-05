// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize, // 生徒の人数
        a: [(Usize1, u32); n], // (学籍番号, 期末試験の点数)
        q: usize,
        ask: [(Usize1, usize); q], // (from, to)
    }

    // クラスと学籍番号の二次元配列に、期末試験の点数を詰め直す
    let mut sum = vec![vec![0; n + 1]; 2];
    for (i, (c, p)) in a.into_iter().enumerate() {
        sum[c][i] += p;
    }

    // 累積和を作る
    for s in sum.iter_mut() {
        for i in (0..n).rev() {
            s[i] += s[i + 1];
        }
    }

    // 質問に回答する
    for (left, right) in ask {
        let a = sum[0][left] - sum[0][right];
        let b = sum[1][left] - sum[1][right];
        println!("{} {}", a, b);
    }
}
