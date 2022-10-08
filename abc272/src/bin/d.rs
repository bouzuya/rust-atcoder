use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut dij = vec![];
    for i in -(n as i64)..=(n as i64) {
        for j in -(n as i64)..=(n as i64) {
            if (i.pow(2) + j.pow(2)) as usize == m {
                dij.push((i, j));
            }
        }
    }

    let inf = 1_usize << 60;
    let mut dist = vec![vec![inf; n]; n];
    dist[0][0] = 0_usize;
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0_usize), (0_usize, 0_usize)));
    while let Some((Reverse(d), (r, c))) = pq.pop() {
        if d != dist[r][c] {
            continue;
        }
        let nd = d + 1;
        for (di, dj) in dij.iter().copied() {
            let (nr, nc) = (r as i64 + di, c as i64 + dj);
            if !(0..n as i64).contains(&nr) || !(0..n as i64).contains(&nc) {
                continue;
            }
            if ((r as i64 - nr).pow(2) + (c as i64 - nc).pow(2)) as usize != m {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if nd < dist[nr][nc] {
                dist[nr][nc] = nd;
                pq.push((Reverse(nd), (nr, nc)));
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!(
                "{}{}",
                if dist[i][j] == inf {
                    -1
                } else {
                    dist[i][j] as i64
                },
                if j == n - 1 { '\n' } else { ' ' }
            );
        }
    }
}
