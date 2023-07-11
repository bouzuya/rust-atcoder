use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn dfs(edges: &[Vec<usize>], x: &[usize], ans: &mut [Vec<usize>], u: usize, p: usize) {
    let mut pq = BinaryHeap::new();
    pq.push(x[u]);
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(edges, x, ans, v, u);
        for a in ans[v].iter().copied() {
            pq.push(a);
        }
    }
    let mut count = 0;
    while let Some(a) = pq.pop() {
        if count == 20 {
            break;
        }
        count += 1;
        ans[u].push(a);
    }
}

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
        q: usize,
        x: [usize; n],
        ab: [(Usize1, Usize1); n - 1],
        vk: [(Usize1, Usize1); q],
    };

    let edges = adjacency_list(n, &ab);
    let mut ans = vec![vec![]; n];
    dfs(&edges, &x, &mut ans, 0, 0);

    for (v, k) in vk {
        println!("{}", ans[v][k]);
    }
}
