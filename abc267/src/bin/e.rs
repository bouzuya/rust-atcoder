use std::collections::BTreeSet;

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
        a: [usize; n],
        uv: [(Usize1, Usize1); m],
    };

    let edges = adjacency_list(n, &uv);
    let weights = (0..n)
        .map(|u| edges[u].iter().copied().map(|v| a[v]).sum::<usize>())
        .collect::<Vec<usize>>();

    let mut ok = weights.iter().sum::<usize>();
    let mut ng = 0_usize;
    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;

        let mut w = weights.clone();
        let mut removed = vec![false; n];
        let mut set = BTreeSet::new();
        for (i, w_i) in weights.iter().copied().enumerate() {
            if w_i <= mid {
                set.insert(i);
            }
        }
        while !set.is_empty() {
            let u = *set.iter().next().unwrap();
            if !removed[u] {
                removed[u] = true;
                for v in edges[u].iter().copied() {
                    if !removed[v] {
                        w[v] -= a[u];
                        if w[v] <= mid {
                            set.insert(v);
                        }
                    }
                }
            }
            set.remove(&u);
        }
        if removed.into_iter().all(|b| b) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let ans = ok;
    println!("{}", ans);
}
