use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };
    let edges = adjacency_list(n, &uv);
    let inf = 1_usize << 60;
    let mut dist = vec![vec![inf; n]; 1 << n];
    let mut deque = VecDeque::new();
    for i in 0..n {
        dist[0][i] = 0;
        dist[1 << i][i] = 1;
        deque.push_back((1 << i, i));
    }
    while let Some((s, u)) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            let s_next = s ^ (1 << v);
            if dist[s_next][v] == inf {
                dist[s_next][v] = dist[s][u] + 1;
                deque.push_back((s_next, v));
            }
        }
    }
    let ans = (0..1 << n)
        .map(|s| dist[s].iter().min().unwrap())
        .sum::<usize>();
    println!("{}", ans);
}
