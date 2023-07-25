use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m]
    };

    let edges = {
        let mut edges = vec![vec![]; n];
        for (u, v, w) in abc.iter().copied() {
            edges[u].push((v, w));
        }
        edges
    };
    let inf = 1_usize << 60;

    for s in 0..n {
        let mut min_dist: Option<usize> = None;
        let mut dist = vec![inf; n];
        let mut pq = BinaryHeap::new();
        dist[s] = 0;
        pq.push(Reverse((0, s)));
        while let Some(Reverse((w_u, u))) = pq.pop() {
            if w_u > dist[u] {
                continue;
            }
            for (v, w_v) in edges[u].iter().copied() {
                if v == s {
                    min_dist = Some(match min_dist {
                        Some(m) => m.min(w_u + w_v),
                        None => w_u + w_v,
                    });
                    continue;
                }
                let w = w_u + w_v;
                if w < dist[v] {
                    dist[v] = w;
                    pq.push(Reverse((w, v)));
                }
            }
        }

        println!("{}", min_dist.map(|ans| ans as i64).unwrap_or(-1));
    }
}
