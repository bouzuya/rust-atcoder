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

fn shortest_path(n: usize, e: &Vec<Vec<usize>>, s: usize) -> Vec<Option<usize>> {
    let mut d = vec![None; n];
    let mut q = VecDeque::new();
    d[s] = Some(0);
    q.push_back((s, s));
    while let Some((u, p)) = q.pop_front() {
        for &v in e[u].iter() {
            if v == p {
                continue;
            }
            if d[v].is_none() {
                let d_v = d[u].unwrap() + 1;
                d[v] = Some(d_v);
                q.push_back((v, u));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };
    let e = adjacency_list(n, &ab);
    let d = shortest_path(n, &e, 0);
    let max_d = d.iter().max_by_key(|d_i| d_i.unwrap()).unwrap().unwrap();
    let s = d.iter().position(|d_i| d_i.unwrap() == max_d).unwrap();
    let d = shortest_path(n, &e, s);
    let max_d = d.iter().max_by_key(|d_i| d_i.unwrap()).unwrap().unwrap();
    let ans = max_d + 1;
    println!("{}", ans);
}
