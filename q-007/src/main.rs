// -*- coding:utf-8-unix -*-

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
    }

    // 各クラスのレーティング情報を取得
    let mut classes = Vec::new();
    for _ in 0..n {
        input! {
            a: i32,
        }
        classes.push(a);
    }
    // 昇順でソートしておく
    classes.sort();

    input! {
        q: usize,
        mut b: [i32; q],
    }

    let mut answers = Vec::new();
    for student in b {
        // q と n の値はそれぞれ大きいため、愚直に for ループを書くと計算量が爆発してしまう
        // そこで、探索範囲を半分に絞る二分探索を使うことで計算量を節約する
        // lower_bound を使うと、配列の中から「指定した値よりも大きい最小の値」の index を取り出すことができる
        let lb = classes.lower_bound(&student);

        if lb == 0 {
            // クラスがひとつしかない or 一番下のクラスの場合は、そのまま不満度を取れば OK
            answers.push((classes[lb] - student).abs());
        } else if lb == n {
            answers.push((classes[lb - 1] - student).abs());
        } else {
            // 「自分のレートより一つ上のクラス」と「現在のレートに見合ったクラス」を比較して、より不満度の低い方に振り分ける
            let target1 = (classes[lb] - student).abs();
            let target2 = (classes[lb - 1] - student).abs();
            answers.push(target1.min(target2));
        }
    }

    for answer in answers {
        println!("{}", answer);
    }
}
