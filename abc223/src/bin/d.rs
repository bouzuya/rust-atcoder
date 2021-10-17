use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
    }
    for e_i in e.iter_mut() {
        e_i.sort();
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
    let mut q = BinaryHeap::new();
    for u in (0..n).rev() {
        if c_in[u] == 0 {
            q.push(Reverse(u));
        }
    }
    let mut res = vec![];
    while let Some(Reverse(u)) = q.pop() {
        res.push(u);
        for &v in e[u].iter() {
            c_in[v] -= 1;
            if c_in[v] == 0 {
                q.push(Reverse(v));
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
        ab: [(Usize1, Usize1); m],
    };

    let e = adjacency_list(n, &ab);
    match topological_sort(&e) {
        None => {
            println!("-1");
        }
        Some(ans) => {
            for x in ans {
                println!("{}", x + 1);
            }
        }
    }
}
