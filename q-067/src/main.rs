// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;
use std::char;

fn main() {
    input! {
        mut n: Chars,
        k: usize,
    }

    let mut answer = n;
    for _i in 0..k {
        let n_ten = to_ten(answer);
        let n_nine = to_nine(n_ten);
        answer = eight_to_five(n_nine);
    }
    println!("{}", answer.iter().collect::<String>());
}

/// 8 進法を 10 進法に直す
fn to_ten(n: Vec<char>) -> u64 {
    let mut result = 0_u64;
    // 下の位から順に変換していく
    for (index, char) in n.into_iter().rev().enumerate() {
        let number = char.to_digit(10).unwrap() as u64;
        result += number * 8_u64.pow(index as u32);
    }
    return result;
}

/// 10 進法を 9 進法に直す
fn to_nine(mut n: u64) -> String {
    let mut result = Vec::new();
    // 9 で割った数が 0 になるまでつづける
    loop {
        // 9 で割ったあまりを先頭に載せる
        let char = char::from_digit((n % 9) as u32, 10).unwrap();
        result.insert(0, char);
        n /= 9;
        if !(n > 0) {
            break;
        }
    }
    return result.into_iter().collect::<String>();
}

/// 8 を 5 に変換する
fn eight_to_five(n: String) -> Vec<char> {
    return n.replace("8", "5").chars().collect();
}
