use proconio::{input, marker::Chars};

fn dijkstra(n: usize, inf: usize, e: &[Vec<usize>], s: usize) -> Vec<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut d = vec![inf; n];
    let mut pq = BinaryHeap::new();
    d[s] = 0;
    pq.push(Reverse((0, s)));
    while let Some(Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for v in e[u].iter().copied() {
            let w = w_u + 1;
            if w < d[v] {
                d[v] = w;
                pq.push(Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };
    let mut edges = vec![vec![]; n];
    let mut redges = vec![vec![]; n];
    for (i, s_i) in s.iter().enumerate() {
        for (j, s_ij) in s_i.iter().copied().enumerate() {
            if s_ij == '1' {
                edges[i].push(i + j + 1);
                redges[i + j + 1].push(i);
            }
        }
    }

    let inf = 1_usize << 60;
    let dist0 = dijkstra(n, inf, &edges, 0);
    let distn = dijkstra(n, inf, &redges, n - 1);

    for k in 1..n - 1 {
        let mut min = inf;
        for j in k.saturating_sub(m)..k {
            for l in edges[j].iter().copied() {
                if l <= k {
                    continue;
                }
                min = min.min(dist0[j] + 1 + distn[l]);
            }
        }
        if min == inf {
            println!("-1");
        } else {
            println!("{}", min);
        }
    }
}
