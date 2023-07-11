use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn warshall_floyd(n: usize, inf: usize, e: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut d = vec![vec![inf; n]; n];
    for (u, e_u) in e.iter().enumerate() {
        d[u][u] = 0;
        for &v in e_u.iter() {
            d[u][v] = 1;
        }
    }
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                if d[u][k] + d[k][v] < d[u][v] {
                    d[u][v] = d[u][k] + d[k][v];
                }
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    };

    let mut e = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            match s[i][j] {
                '0' => {}
                '1' => e[i].push(j),
                _ => unreachable!(),
            }
        }
    }

    let mut color = vec![None; n];
    let mut deque = VecDeque::new();
    color[0] = Some(true);
    deque.push_back((0, true));
    while let Some((u, c)) = deque.pop_front() {
        if color[u] != Some(c) {
            println!("-1");
            return;
        }
        let nc = !c;
        for v in e[u].iter().copied() {
            match color[v] {
                Some(vc) => {
                    if vc != nc {
                        println!("-1");
                        return;
                    }
                }
                None => {
                    color[v] = Some(nc);
                    deque.push_back((v, nc));
                }
            }
        }
    }

    let inf = n * n + 1;
    let d = warshall_floyd(n, inf, &e);

    let ans = d.iter().map(|d_i| d_i.iter().max().unwrap()).max().unwrap() + 1;
    println!("{}", ans);
}
