use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m]
    };
    let mut count = vec![0; n];
    let mut edges = vec![vec![]; n];
    for (x, y) in xy.iter().copied() {
        edges[y].push(x);
        count[x] += 1;
    }

    let mut ans = vec![0; n];
    let mut deque = count
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, c)| c == 0)
        .map(|(i, _)| i)
        .collect::<VecDeque<_>>();
    while let Some(v) = deque.pop_front() {
        for u in edges[v].iter().copied() {
            count[u] -= 1;
            if count[u] == 0 {
                ans[u] = ans[v] + 1;
                deque.push_back(u);
            }
        }
    }

    let ans = *ans.iter().max().unwrap();
    println!("{}", ans);
}
