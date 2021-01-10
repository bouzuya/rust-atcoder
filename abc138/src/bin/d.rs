use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        px: [(Usize1, u64); q],
    };

    let mut xs = vec![0_u64; n];
    for (p_i, x_i) in px {
        xs[p_i] += x_i;
    }

    let e = adjacency_list(n, &ab);

    let mut vs = vec![0_u64; n];
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    while let Some((u, p)) = q.pop_front() {
        vs[u] = xs[u] + vs[p];
        for &v in e[u].iter() {
            if v == p {
                continue;
            }
            q.push_back((v, u));
        }
    }

    for v_i in vs {
        println!("{}", v_i);
    }
}
