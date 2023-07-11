use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };

    let mut pq = BinaryHeap::new();
    for i in 0..h {
        for j in 0..w {
            pq.push((a[i][j], i, j));
        }
    }

    let mut dp = vec![vec![1_usize; w]; h];
    while let Some((_, i, j)) = pq.pop() {
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if a[nr][nc] >= a[i][j] {
                continue;
            }
            dp[nr][nc] += dp[i][j];
            dp[nr][nc] %= 1_000_000_007;
        }
    }

    let mut ans = 0_usize;
    for i in 0..h {
        for j in 0..w {
            ans += dp[i][j];
            ans %= 1_000_000_007;
        }
    }
    println!("{}", ans);
}
