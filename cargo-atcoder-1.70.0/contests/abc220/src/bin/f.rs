use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn dfs(edges: &[Vec<usize>], depth: &mut [usize], size: &mut [usize], u: usize, p: usize) {
    depth[u] = depth[p] + 1;
    let mut s = 1_usize;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        dfs(edges, depth, size, v, u);
        s += size[v];
    }
    size[u] = s;
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };

    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }

    let inf = 1_usize << 60;

    let mut depth = vec![0; n];
    let mut size = vec![inf; n];
    dfs(&edges, &mut depth, &mut size, 0, 0);

    let mut ans = vec![inf; n];
    ans[0] = depth.iter().sum::<usize>() - n;

    let mut deque = VecDeque::new();
    deque.push_back(0);
    while let Some(u) = deque.pop_front() {
        for v in edges[u].iter().copied() {
            if ans[v] != inf {
                continue;
            }
            deque.push_back(v);
            ans[v] = ans[u] + (n - size[v]) - size[v];
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
