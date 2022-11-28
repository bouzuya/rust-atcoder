use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    for (k, edges_k) in edges.iter().enumerate() {
        println!(
            "{}: {{{}}}",
            k + 1,
            edges_k
                .iter()
                .map(|v| v + 1)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
