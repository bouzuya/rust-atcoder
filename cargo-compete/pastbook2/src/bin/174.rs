use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    };

    let mut edges = vec![vec![]; n];
    for (a_i, b_i, c_i) in abc.iter().copied() {
        edges[a_i].push((b_i, c_i));
        edges[b_i].push((a_i, c_i));
    }

    let mut pos = vec![0_i64; n]; // p_i + x_0
    let mut neg = vec![0_i64; n]; // q_i - x_0
    let mut exists_pos = vec![false; n];
    let mut exists_neg = vec![false; n];

    let mut lbound = 0_i64;
    let mut ubound = 1_i64 << 60;

    let mut deque = VecDeque::new();
    deque.push_back(0);
    exists_pos[0] = true;
    while let Some(u) = deque.pop_front() {
        for (v, c) in edges[u].iter().copied() {
            let mut updated = false;

            if exists_pos[u] && !exists_neg[v] {
                exists_neg[v] = true;
                neg[v] = c - pos[u];
                ubound = ubound.min(c - pos[u]);
                updated = true;
            }

            if exists_neg[u] && !exists_pos[v] {
                exists_pos[v] = true;
                pos[v] = c - neg[u];
                lbound = lbound.max(neg[u] - c);
                updated = true;
            }

            if updated {
                deque.push_back(v);
            }
        }
    }

    if lbound > ubound {
        println!("-1");
        return;
    }

    let mut ans = vec![0_i64; n];

    let mut x = ubound;
    for i in 0..n {
        if exists_pos[i] && exists_neg[i] {
            if (neg[i] - pos[i]) % 2 != 0 {
                println!("-1");
                return;
            }
            x = (neg[i] - pos[i]) / 2;
        }
    }

    for i in 0..n {
        if exists_pos[i] {
            ans[i] = pos[i] + x;
        } else {
            ans[i] = neg[i] - x;
        }

        if ans[i] < 0 {
            println!("-1");
            return;
        }
    }

    for (a_i, b_i, c_i) in abc.iter().copied() {
        if ans[a_i] + ans[b_i] != c_i {
            println!("-1");
            return;
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
