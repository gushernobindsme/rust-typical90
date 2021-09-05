// -*- coding:utf-8-unix -*-

use petgraph::{Graph, Undirected};
use proconio::input;

fn main() {
    input! {
        mut _n: usize, // 頂点
        mut m: usize, // 辺
        paths: [(u32, u32); m],
    }

    // グラフを生成する
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    graph.extend_with_edges(&paths);

    // グラフの頂点を走査する
    let mut answer = 0_usize;
    for node_index in graph.node_indices() {
        // 自分自身より頂点番号が小さい隣接頂点の件数を数える
        let neighbors = graph
            .neighbors_undirected(node_index)
            .filter(|v| node_index.index() > v.index())
            .count();
        // 条件を満たす頂点の個数を数える
        if neighbors == 1 {
            answer = answer + 1
        }
    }

    println!("{}", answer);
}
