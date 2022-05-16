use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        s: Usize1,
        t: Usize1,
    };
    let edges = adjacency_list(n, &uv);

    let inf = 1_usize << 60;
    let mut dist = vec![vec![inf; 3]; n];
    let mut pq = BinaryHeap::new();
    dist[s][0] = 0;
    pq.push(Reverse((0, 0, s)));
    while let Some(Reverse((w, x, u))) = pq.pop() {
        if w > dist[u][x] {
            continue;
        }
        for v in edges[u].iter().copied() {
            let w_v = w + 1;
            let x_v = (x + 1) % 3;
            if w_v < dist[v][x_v] {
                dist[v][x_v] = w_v;
                pq.push(Reverse((w_v, x_v, v)));
            }
        }
    }

    let ans = dist[t][0];
    if ans == inf || ans % 3 != 0 {
        println!("-1");
    } else {
        println!("{}", ans / 3);
    }
}
