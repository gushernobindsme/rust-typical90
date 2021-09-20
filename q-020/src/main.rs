// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, u32, usize),
    }
    // 「\log_2 a < b * \log_2 c」満たすかどうかを調べる
    // \log_2 a は「2 を何乗したら a になるか」を意味する
    // \log_2 c は「2 を何乗したら c になるか」を意味する

    // たとえば a = 4, b = 3, c = 2 の場合、
    // \log_2 4 は「2 を何乗したら 4 になるか」を意味するので、答えは 2
    // \log_2 2 は「2 を何乗したら 2 になるか」を意味するので、答えは 1
    // 2 < 3 * 1 なので、\log_2 a < b * \log_2 c を満たす、といえる

    // 「\log_2 a < b * \log_2 c」は以下の通り変形できる（らしい）
    // => \log_2 a < \log_2 (c^b)
    // => a < c^b

    if a < c.pow(b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
