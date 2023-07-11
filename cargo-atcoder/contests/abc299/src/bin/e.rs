use std::collections::VecDeque;

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
        uv: [(Usize1, Usize1); m],
        k: usize,
        pd: [(Usize1, usize); k],
    };

    let edges = adjacency_list(n, &uv);

    let inf = 2 * n;
    let mut dist = vec![vec![inf; n]; n];
    for p in 0..n {
        let mut deque = VecDeque::new();
        dist[p][p] = 0;
        deque.push_back(p);
        while let Some(u) = deque.pop_front() {
            for v in edges[u].iter().copied() {
                if dist[p][v] == inf {
                    dist[p][v] = dist[p][u] + 1;
                    deque.push_back(v);
                }
            }
        }
    }

    // 0: white, 1: black, 2: none
    let mut colors = vec![2; n];
    for (p, d) in pd.iter().copied() {
        for v in 0..n {
            if dist[p][v] < d {
                colors[v] = 0;
            }
        }
    }
    for i in 0..n {
        colors[i] = match colors[i] {
            0 => 0,
            2 => 1,
            _ => unreachable!(),
        };
    }

    for (p, d) in pd.iter().copied() {
        let mut any_one = false;
        for v in 0..n {
            if colors[v] == 1 {
                if dist[p][v] < d {
                    println!("No");
                    return;
                }
                if dist[p][v] == d {
                    any_one = true;
                }
            }
        }
        if !any_one {
            println!("No");
            return;
        }
    }

    println!("Yes");
    for c in colors.iter().copied() {
        print!("{}", c);
    }
}
