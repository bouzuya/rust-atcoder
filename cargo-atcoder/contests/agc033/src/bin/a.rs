use std::{cmp::max, cmp::Reverse, collections::BinaryHeap};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };
    let inf = h * w + 1_usize;
    let mut d = vec![vec![inf; w]; h];
    let mut pq = BinaryHeap::new();
    for r in 0_usize..h {
        for c in 0_usize..w {
            if a[r][c] == '#' {
                d[r][c] = 0;
                pq.push(Reverse((0_usize, r, c)));
            }
        }
    }
    let mut max_d = 0;
    while let Some(Reverse((d_i, r_i, c_i))) = pq.pop() {
        if d_i > d[r_i][c_i] {
            continue;
        }
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            if dr == 0 && dc == 0 {
                continue;
            }
            let (nr, nc) = (r_i as i64 + dr, c_i as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let nd = d_i + 1;
            if nd < d[nr][nc] {
                d[nr][nc] = nd;
                pq.push(Reverse((nd, nr, nc)));
                max_d = max(max_d, nd);
            }
        }
    }
    let ans = max_d;
    println!("{}", ans);
}
