// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    }

    let mut answer = 0;

    // a から 5 個を選ぶ組み合わせを列挙
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        // そのまま積を出すとオーバーフローしてしまうので、 p で割ったあまりどうしの計算に置き換える
                        // p で割ったあまりが q となるパターンを探す
                        if a[i] * a[j] % p * a[k] % p * a[l] % p * a[m] % p == q {
                            answer = answer + 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", answer);
}
