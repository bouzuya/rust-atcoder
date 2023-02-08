use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let inf = h * w + 1;
    let mut dist = vec![vec![inf; w]; h];
    let mut deque = VecDeque::new();
    dist[0][0] = 0;
    deque.push_back((0, 0));
    while let Some((i, j)) = deque.pop_front() {
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if s[nr][nc] != '.' {
                continue;
            }
            if dist[nr][nc] != inf {
                continue;
            }
            dist[nr][nc] = dist[i][j] + 1;
            deque.push_back((nr, nc));
        }
    }
    let d = dist[h - 1][w - 1];
    if d == inf {
        println!("-1");
        return;
    }
    let d = d + 1;

    let mut count = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                count += 1;
            }
        }
    }

    let ans = count - d;
    println!("{}", ans);
}
