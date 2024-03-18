use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut map = HashMap::new();
    for e in edges {
        *map.entry(e.len()).or_insert(0) += 1;
    }

    let mut m = HashMap::new();
    m.insert(n - 1, 1);
    m.insert(1, n - 1);
    let ans = m == map;
    println!("{}", if ans { "Yes" } else { "No" });
}
