use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn adjacency_list(n: usize, uvw: &Vec<(usize, usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut e = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(Usize1, Usize1, Usize1); m],
    };

    let e = adjacency_list(n, &uvc);
    let inf = n + 1;
    let mut c = vec![inf; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    c[0] = 0;
    while let Some(u) = q.pop_front() {
        for &(v, c_v) in e[u].iter() {
            if c[v] != inf {
                continue;
            }
            if c[u] == c_v {
                c[v] = (c_v + 1) % n;
            } else {
                c[v] = c_v;
            }
            q.push_back(v);
        }
    }
    if c.iter().any(|&c_i| c_i == inf) {
        println!("No");
        return;
    }

    for c_i in c {
        println!("{}", c_i + 1);
    }
}
