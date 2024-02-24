use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ldkcab: [(i64, i64, i64, i64, Usize1, Usize1); m],
    };
    let mut rev_edges = vec![vec![]; n];
    for (l, d, k, c, a, b) in ldkcab {
        rev_edges[b].push((a, l, d, k, c));
    }

    let inf = 1_i64 << 60;
    let mut f = vec![-inf; n];
    for (_, l, d, k, c) in rev_edges[n - 1].iter().copied() {
        let w = l + (k - 1) * d + c;
        f[n - 1] = f[n - 1].max(w);
    }
    if f[n - 1] == -inf {
        for _ in 0..n - 1 {
            println!("Unreachable");
        }
        return;
    }
    let mut pq = BinaryHeap::new();
    pq.push((f[n - 1], n - 1));
    while let Some((w_u, u)) = pq.pop() {
        if w_u < f[u] {
            continue;
        }
        for (v, l, d, k, c) in rev_edges[u].iter().copied() {
            if w_u < l + c {
                continue;
            }
            let w = ((w_u - c - l) / d).min(k - 1) * d + l;
            if w > f[v] {
                f[v] = w;
                pq.push((w, v));
            }
        }
    }

    for a in 0..n - 1 {
        let ans = if f[a] == -inf {
            "Unreachable".to_string()
        } else {
            f[a].to_string()
        };
        println!("{}", ans);
    }
}
