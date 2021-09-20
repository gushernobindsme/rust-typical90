// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut students = Vec::new();
    for _ in 0..n {
        input! {
            a: i64,
        }
        students.push(a);
    }
    students.sort();

    let mut schools = Vec::new();
    for _ in 0..n {
        input! {
            b: i64,
        }
        schools.push(b);
    }
    schools.sort();

    // この場合、学校の位置と生徒の位置をそれぞれを昇順ソートして、順番に取り出せば最適になる。
    // 1 ステップ先のことのみを考えて最適化する判断を繰り返して、解を作り上げていく方法を貪欲法と呼ぶ。
    // 貪欲法は、全ステップを通したときに必ずしも最適解を導くとは限らないが、ある種の問題に対しては有効に機能する。
    let mut answer = 0;
    for i in 0..n {
        let student = students[i];
        let school = schools[i];
        answer += (school - student).abs();
    }
    println!("{}", answer);
}
