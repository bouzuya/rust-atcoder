use std::collections::VecDeque;

use proconio::input;

fn f(n: usize, xy: &[(i64, i64, i64)], s: i64) -> bool {
    let mut edges = vec![vec![]; n];
    for (i, (x_i, y_i, p_i)) in xy.iter().copied().enumerate() {
        for (j, (x_j, y_j, _)) in xy.iter().copied().enumerate() {
            if i == j {
                continue;
            }
            if p_i * s >= (x_i - x_j).abs() + (y_i - y_j).abs() {
                edges[i].push(j);
            }
        }
    }

    let mut ok = false;
    for s in 0..n {
        let mut used = vec![false; n];
        let mut deque = VecDeque::new();
        deque.push_back(s);
        used[s] = true;
        while let Some(u) = deque.pop_front() {
            for v in edges[u].iter().copied() {
                if used[v] {
                    continue;
                }
                deque.push_back(v);
                used[v] = true;
            }
        }
        if used.iter().copied().all(|b| b) {
            ok = true;
            break;
        }
    }
    ok
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64, i64); n],
    };

    let mut ng = 0_i64;
    let mut ok = 10_000_000_000_i64;
    while ok - ng > 1 {
        let s = ng + (ok - ng) / 2;
        if f(n, &xy, s) {
            ok = s;
        } else {
            ng = s;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
