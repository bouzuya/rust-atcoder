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
        ab: [(Usize1, Usize1); m],
        q: usize,
        xk: [(Usize1, usize); q],
    };

    let edges = adjacency_list(n, &ab);
    for (x_i, k_i) in xk {
        let mut ans = 0_usize;
        let mut used = vec![false; n];

        let mut deque = VecDeque::new();
        deque.push_back((x_i, 0));
        used[x_i] = true;
        ans += x_i + 1;
        while let Some((u, d)) = deque.pop_front() {
            for v in edges[u].iter().copied() {
                if used[v] || d + 1 > k_i {
                    continue;
                }
                used[v] = true;
                deque.push_back((v, d + 1));
                ans += v + 1;
            }
        }
        println!("{}", ans);
    }
}
