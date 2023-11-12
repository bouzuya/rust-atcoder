use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvw: [(Usize1, Usize1, usize); m],
    };

    let mut edges = vec![vec![]; n];
    for (u, v, w) in uvw {
        edges[u].push((v, w));
        edges[v].push((u, w));
    }

    let mut costs = vec![BTreeSet::new(); 1 << n];
    costs[0].insert(0_usize);
    for u in 0..n {
        for (v, w) in edges[u].iter().copied() {
            costs[1 << u | 1 << v].insert(w % k);
        }
    }
    for bits in 1..1 << n {
        for u in 0..n {
            if ((bits >> u) & 1) != 1 {
                continue;
            }
            for (v, w) in edges[u].iter().copied() {
                if ((bits >> v) & 1) != 1 {
                    continue;
                }
                let cs = costs[bits ^ (1 << u)]
                    .iter()
                    .copied()
                    .collect::<Vec<usize>>();
                for c in cs {
                    costs[bits].insert((c + w) % k);
                }
            }
        }
    }
    let ans = costs[(1 << n) - 1].first().unwrap();
    println!("{:?}", ans);
}
