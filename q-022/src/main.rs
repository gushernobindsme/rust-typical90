// -*- coding:utf-8-unix -*-

use proconio::input;
use num::integer::gcd;

fn main() {
    input! {
        mut a: u64, // 幅
        mut b: u64, // 奥行き
        mut c: u64, // 高さ
    }

    // r * r * r の正方形を作るには、
    // a, b, c のすべてが r で割り切れる必要がある。
    // 言い換えると、r は a, b, c の最大公約数である必要がある。
    let r = gcd(a, gcd(b, c));

    // あとは r のサイズになるまでナイフを入れ続ければ OK
    let a_count = if a == r { 0 } else { a / r - 1 };
    let b_count = if b == r { 0 } else { b / r - 1 };
    let c_count = if c == r { 0 } else { c / r - 1 };

    println!("{}", a_count + b_count + c_count);
}
