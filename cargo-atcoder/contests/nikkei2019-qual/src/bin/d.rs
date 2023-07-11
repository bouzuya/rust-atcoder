use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
    }
    e
}

fn adjacency_list_rev(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[v].push(u);
    }
    e
}

fn topological_sort(e: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = e.len();
    let mut c_in = vec![0; n];
    for e_u in e.iter() {
        for &v in e_u.iter() {
            c_in[v] += 1;
        }
    }
    let mut q = VecDeque::new();
    for u in 0..n {
        if c_in[u] == 0 {
            q.push_back(u);
        }
    }
    let mut res = vec![];
    while let Some(u) = q.pop_front() {
        res.push(u);
        for &v in e[u].iter() {
            c_in[v] -= 1;
            if c_in[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if res.len() == n {
        Some(res)
    } else {
        None
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n + m - 1],
    };
    let e = adjacency_list(n, &ab);
    let mut sorted = topological_sort(&e)
        .unwrap()
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    sorted.sort_by_key(|&(_, v)| v);
    let e = adjacency_list_rev(n, &ab);
    let mut ans = vec![n; n];
    for (_, v) in sorted.iter().copied() {
        ans[v] = e[v]
            .iter()
            .copied()
            .max_by_key(|&u| sorted[u].0)
            .map(|u| u + 1)
            .unwrap_or(0);
    }
    for a_i in ans {
        println!("{}", a_i);
    }
}
