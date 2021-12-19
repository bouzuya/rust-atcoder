use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };
    let mut dp = vec![vec![0; w]; h];
    let mut deque = VecDeque::new();
    dp[0][0] = 1;
    deque.push_back((0, 0, 1));
    while let Some((i, j, d)) = deque.pop_front() {
        let dir = vec![(0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc, nd) = (i as i64 + dr, j as i64 + dc, d + 1);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if c[nr][nc] != '.' {
                continue;
            }
            if dp[nr][nc] >= nd {
                continue;
            }
            dp[nr][nc] = nd;
            deque.push_back((nr, nc, nd));
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(dp[i][j]);
        }
    }
    println!("{}", ans);
}
