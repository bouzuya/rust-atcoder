use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut group = vec![];
    let mut joined_groups = vec![vec![]; m];
    for i in 0..n {
        input! {
            a_i: usize,
            s_i: [Usize1; a_i],
        }
        for s_ij in s_i.iter().copied() {
            joined_groups[s_ij].push(i);
        }
        group.push(s_i);
    }

    if joined_groups[0].is_empty() || joined_groups[m - 1].is_empty() {
        println!("-1");
        return;
    }

    let inf = n + 1;
    let mut dist = vec![inf; n];
    let mut used = vec![false; m];
    let mut deque = VecDeque::new();
    for g in joined_groups[0].iter().copied() {
        dist[g] = 0_usize;
        deque.push_back(g);
    }
    used[0] = true;

    while let Some(g_u) = deque.pop_front() {
        for u_i in group[g_u].iter().copied() {
            if used[u_i] {
                continue;
            }
            for g_v in joined_groups[u_i].iter().copied() {
                if dist[g_v] == inf {
                    dist[g_v] = dist[g_u] + 1;
                    deque.push_back(g_v);
                }
            }
            used[u_i] = true;
        }
    }

    let mut d: Option<usize> = None;
    for g in joined_groups[m - 1].iter().copied() {
        if dist[g] != inf {
            d = Some(match d {
                Some(old) => old.min(dist[g]),
                None => dist[g],
            });
        }
    }

    println!("{}", d.map(|d| d as i64).unwrap_or(-1));
}
