use std::cmp::min;

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
        m: usize,
        s: Usize1,
        uv: [(Usize1, Usize1); m],
    };

    let e = adjacency_list(n, &uv);

    let mut w = vec![None; n];
    let mut pq = std::collections::BinaryHeap::new();
    w[s] = Some(s + 1);
    pq.push((s + 1, s));
    while let Some((w_u, u)) = pq.pop() {
        if w[u].is_some() && w[u].unwrap() > w_u {
            continue;
        }
        for &v in e[u].iter() {
            let w_v = min(w_u, v + 1);
            match w[v] {
                None => {
                    w[v] = Some(w_v);
                    pq.push((w_v, v));
                }
                Some(x) => {
                    if x < w_v {
                        w[v] = Some(w_v);
                        pq.push((w_v, v));
                    }
                }
            }
        }
    }

    let mut s = String::new();
    for (i, &w_i) in w.iter().enumerate() {
        match w_i {
            None => continue,
            Some(w_i) => {
                if i + 1 <= w_i {
                    s.push_str(&format!("{}\n", i + 1));
                }
            }
        }
    }
    print!("{}", s);
}
