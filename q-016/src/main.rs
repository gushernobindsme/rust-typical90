// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i64,
        (a, b, c): (i64, i64, i64),
    }

    let mut answer = std::i64::MAX;
    for i in 0..9999 {
        for j in 0..(9999 - i) {
            let remains = n - ((i * a) + (j * b));
            // a と b で支払った金額の残りから c の枚数を求める
            let coin_counts = i + j + remains / c;

            // ちょうど n 円を支払い切れていない場合、NG
            if remains % c != 0 {
                continue;
            }
            // n 円を超過している場合、NG
            if remains < 0 {
                continue;
            }
            // 金貨の枚数の合計が 9,999 枚を超えている場合、NG
            if coin_counts > 9999 {
                continue;
            }
            answer = answer.min(coin_counts);
        }
    }

    println!("{}", answer);
}
