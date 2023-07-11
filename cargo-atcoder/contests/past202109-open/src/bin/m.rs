use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn dfs(
    inf: i64,
    edges: &[Vec<(usize, i64, usize)>],
    res: &mut Vec<i64>,
    u: usize,
    p: usize,
    x: i64,
) {
    res[u] = x;
    for (v, w, _) in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        if res[v] != inf {
            continue;
        }

        dfs(inf, edges, res, v, u, w - x);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    };

    let mut edges = vec![vec![]; n];
    for (i, (a_i, b_i, c_i)) in abc.iter().copied().enumerate() {
        edges[a_i].push((b_i, c_i, i));
        edges[b_i].push((a_i, c_i, i));
    }

    let mut p = vec![None; n]; // p_i + x_0
    let mut q = vec![None; n]; // q_i - x_0
    let mut used_edges = vec![false; m];
    let mut deque = VecDeque::new();
    let x_0 = 0;
    deque.push_back((0, x_0, true));
    p[0] = Some(x_0);
    q[0] = Some(x_0);
    while let Some((u, x_u, u_is_p)) = deque.pop_front() {
        for (v, w, i) in edges[u].iter().copied() {
            if used_edges[i] {
                continue;
            }
            used_edges[i] = true;

            let nv = w - x_u;
            if let Some(cv) = if u_is_p { q[v] } else { p[v] } {
                if cv != nv {
                    println!("-1");
                    return;
                }
            }
            if u_is_p {
                q[v] = Some(nv);
            } else {
                p[v] = Some(nv);
            }
            deque.push_back((v, nv, !u_is_p));
        }
    }

    let mut bounds = (
        0,
        edges[0].iter().copied().map(|(_, w, _)| w).min().unwrap(),
    );
    let mut ans0 = None;
    for (p_i, q_i) in p.iter().copied().zip(q.iter().copied()).skip(1) {
        match (p_i, q_i) {
            (None, None) => unreachable!(),
            (None, Some(q_i)) => bounds = (bounds.0, bounds.1.min(q_i)),
            (Some(p_i), None) => bounds = (bounds.0.max(-p_i), bounds.1),
            (Some(p_i), Some(q_i)) => {
                if (q_i - p_i) % 2 != 0 {
                    println!("-1");
                    return;
                }
                if let Some(ans_0) = ans0 {
                    if ans_0 != (q_i - p_i) / 2 {
                        println!("-1");
                        return;
                    }
                }
                ans0 = Some((q_i - p_i) / 2);
            }
        }
    }

    let (l, u) = bounds;
    if l > u {
        println!("-1");
        return;
    }
    let ans_0 = ans0.unwrap_or(l);
    if !(l..=u).contains(&ans_0) {
        println!("-1");
        return;
    }

    let inf = 1_000_000_000_000_000_i64;
    let mut ans = vec![inf; n];
    dfs(inf, &edges, &mut ans, 0, 0, ans_0);

    for (a, b, c) in abc.iter().copied() {
        if ans[a] + ans[b] != c {
            unreachable!();
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
