use std::collections::BinaryHeap;

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
        k: usize,
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, usize); k],
    };

    let edges = adjacency_list(n, &ab);

    let mut ans = vec![None; n];
    let mut pq = BinaryHeap::new();
    for (p, h) in ph {
        ans[p] = Some(h);
        pq.push((h, p));
    }

    while let Some((h, p)) = pq.pop() {
        if h < ans[p].unwrap() {
            continue;
        }

        if h == 0 {
            continue;
        }

        for v in edges[p].iter().copied() {
            if ans[v].is_none() || h - 1 > ans[v].unwrap() {
                ans[v] = Some(h - 1);
                pq.push((h - 1, v));
            }
        }
    }

    let mut a = vec![];
    for (i, ans_i) in ans.iter().copied().enumerate() {
        if ans_i.is_some() {
            a.push(i);
        }
    }
    println!("{}", a.len());
    for a_i in a {
        println!("{}", a_i + 1);
    }
}
