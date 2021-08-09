use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let inf = h * w + 1;
    let mut dist = vec![vec![inf; w]; h];
    let mut pq = BinaryHeap::new();
    dist[0][0] = 0;
    pq.push(Reverse((0, 0, 0)));
    while let Some(Reverse((d, i, j))) = pq.pop() {
        if dist[i][j] != d {
            continue;
        }
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let nd = d;
            if s[nr][nc] == '.' && nd < dist[nr][nc] {
                dist[nr][nc] = nd;
                pq.push(Reverse((nd, nr, nc)));
            }
        }
        // .xxx.
        // xxxxx
        // xx@xx
        // xxxxx
        // .xxx.
        for dr in -2..=2_i64 {
            for dc in -2..=2_i64 {
                if dr.abs() + dc.abs() > 3_i64 {
                    continue;
                }
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                let nd = d + 1;
                if nd < dist[nr][nc] {
                    dist[nr][nc] = nd;
                    pq.push(Reverse((nd, nr, nc)));
                }
            }
        }
    }
    let ans = dist[h - 1][w - 1];
    println!("{}", ans);
}
