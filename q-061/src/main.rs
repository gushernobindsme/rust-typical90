// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    }

    let mut deck = VecDeque::new();
    let mut answers = Vec::new();
    for i in 0..q {
        let (t, q) = tx[i];
        if t == 1 {
            // 整数 x_i が書かれたカードを山札の一番上にいれる
            deck.push_front(q);
        } else if t == 2 {
            // 整数 x_i が書かれたカードを山札の一番下にいれる
            deck.push_back(q);
        } else if t == 3 {
            // 山札の上から x_i 番目のカードに書かれた数を紙に書き出す
            if let Some(number) = deck.get(q - 1) {
                answers.push(*number);
            }
        } else {
            panic!();
        }
    }

    for answer in answers {
        println!("{}", answer);
    }
}
