use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let inf = 1_usize << 60;
    let f = |dist: &mut Vec<Vec<usize>>, i: usize, j: usize| {
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), (i, j)));
        dist[i][j] = 0;
        while let Some((Reverse(d), (i, j))) = pq.pop() {
            if dist[i][j] < d {
                continue;
            }
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if dist[nr][nc] > dist[i][j] + a[nr][nc] {
                    dist[nr][nc] = dist[i][j] + a[nr][nc];
                    pq.push((Reverse(dist[nr][nc]), (nr, nc)));
                }
            }
        }
    };

    let mut dist_h1 = vec![vec![inf; w]; h];
    f(&mut dist_h1, h - 1, 0);

    let mut dist_hw = vec![vec![inf; w]; h];
    f(&mut dist_hw, h - 1, w - 1);

    let mut dist_1w = vec![vec![inf; w]; h];
    f(&mut dist_1w, 0, w - 1);

    let mut min = inf;
    for i in 0..h {
        for j in 0..w {
            min = min.min(dist_h1[i][j] + dist_hw[i][j] + dist_1w[i][j] - 2 * a[i][j]);
        }
    }
    let ans = min;
    println!("{}", ans);
}
