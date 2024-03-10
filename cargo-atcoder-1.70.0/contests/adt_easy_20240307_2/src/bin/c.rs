use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m]
    };
    let mut edges = vec![HashSet::new(); n];
    for (u, v) in uv {
        edges[u].insert(v);
        edges[v].insert(u);
    }

    let mut count = 0_usize;
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if edges[a].contains(&b) && edges[b].contains(&c) && edges[c].contains(&a) {
                    count += 1;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
