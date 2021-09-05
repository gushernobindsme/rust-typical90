// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[usize; w]; h],
    }

    // 列と行の合計をあらかじめ計算しておく
    let mut rows = HashMap::new();
    let mut columns = HashMap::new();
    for i in 0..h {
        rows.insert(i, a[i].iter().sum::<usize>());
        for j in 0..w {
            *columns.entry(j).or_insert(0) += a[i][j];
        }
    }

    let mut answers = Vec::new();
    for i in 0..h {
        let mut answer = Vec::new();
        for j in 0..w {
            // 横と縦の計算結果をそれぞれ取得し、自分自身を除く
            let result = *rows.get(&i).unwrap() + *columns.get(&j).unwrap() - a[i][j];
            answer.push(result.to_string());
        }
        answers.push(answer);
    }

    for i in 0..h {
        println!("{}", answers[i].join(" "));
    }
}
