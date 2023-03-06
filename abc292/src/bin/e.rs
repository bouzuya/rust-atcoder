use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };
    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        edges[u].push(v);
    }

    let mut all = 0_usize;
    for start in 0..n {
        let mut used = vec![false; n];
        used[start] = true;
        let mut count = 0_usize;
        let mut deque = edges[start].iter().copied().collect::<VecDeque<usize>>();
        while let Some(u) = deque.pop_front() {
            if used[u] {
                continue;
            }
            used[u] = true;
            count += 1;
            for v in edges[u].iter().copied() {
                deque.push_back(v);
            }
        }
        all += count;
    }

    let ans = all - m;
    println!("{}", ans);
}
