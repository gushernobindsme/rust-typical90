// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    // (ユーザ名, 登録日) のマップを作る
    // 降順にして一回だけ走査し、同じ値だった場合は上書きする
    let mut map: HashMap<String, usize> = HashMap::new();
    for i in (0..n).rev() {
        map.insert(s[i].to_string(), i + 1);
    }

    // 登録日の昇順でソートする
    let mut vec = map.into_iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| a.1.cmp(&b.1));

    for (_, date) in vec.iter() {
        println!("{}", date);
    }
}
