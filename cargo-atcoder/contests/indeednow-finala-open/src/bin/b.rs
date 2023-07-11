use proconio::{input, marker::Bytes};

fn dijkstra(n: usize, inf: u64, e: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
        if w_u > d[u] {
            continue;
        }
        for &(v, w_v) in e[u].iter() {
            let w = w_u + w_v;
            if w < d[v] {
                d[v] = w;
                pq.push(std::cmp::Reverse((w, v)));
            }
        }
    }
    d
}

fn main() {
    input! {
        r: usize,
        c: usize,
        a: [Bytes; r],
    };
    let mut s = 0;
    let mut t = 0;
    let mut d = vec![vec![0; c]; r];
    for i in 0..r {
        for j in 0..c {
            d[i][j] = match a[i][j] {
                b's' => {
                    s = i * c + j;
                    0
                }
                b't' => {
                    t = i * c + j;
                    0
                }
                c => c - b'0',
            } as u64;
        }
    }

    let pos = |i: usize, j: usize| -> usize { i * c + j };

    let mut e = vec![vec![]; r * c];
    for i in 0..r {
        for j in 0..c {
            let u = pos(i, j);
            if i % 2 == 0 {
                // r
                if j + 1 < c {
                    let v = pos(i, j + 1);
                    e[u].push((v, d[i][j]));
                    e[v].push((u, d[i][j + 1]));
                }
                // br
                if i + 1 < r {
                    let v = pos(i + 1, j);
                    e[u].push((v, d[i][j]));
                    e[v].push((u, d[i + 1][j]));
                }
                // bl
                if i + 1 < r && j > 0 {
                    let v = pos(i + 1, j - 1);
                    e[u].push((v, d[i][j]));
                    e[v].push((u, d[i + 1][j - 1]));
                }
            } else {
                // r
                if j + 1 < c {
                    let v = pos(i, j + 1);
                    e[u].push((v, d[i][j]));
                    e[v].push((u, d[i][j + 1]));
                }
                // br
                if i + 1 < r && j + 1 < c {
                    let v = pos(i + 1, j + 1);
                    e[u].push((v, d[i][j]));
                    e[v].push((u, d[i + 1][j + 1]));
                }
                // bl
                if i + 1 < r {
                    let v = pos(i + 1, j);
                    e[u].push((v, d[i][j]));
                    e[v].push((u, d[i + 1][j]));
                }
            }
        }
    }

    let inf = 1_000_000_000_000;
    let d = dijkstra(r * c, inf, &e, s);

    let ans = d[t];
    println!("{}", ans);
}
