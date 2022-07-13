use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: (Usize1, Usize1),
        d: (Usize1, Usize1),
        s: [Chars; h],
    };
    let inf = h * w + 1;
    let mut dist = vec![vec![inf; w]; h];
    let mut deque = VecDeque::new();
    deque.push_back((c.0, c.1, 0));
    while let Some((r, c, d)) = deque.pop_front() {
        if dist[r][c] != inf {
            continue;
        }
        dist[r][c] = d;

        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (r as i64 + dr, c as i64 + dc);
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
            deque.push_front((nr, nc, dist[r][c]));
        }

        for dr in -2..=2 {
            for dc in -2..=2 {
                let (nr, nc) = (r as i64 + dr, c as i64 + dc);
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
                deque.push_back((nr, nc, dist[r][c] + 1));
            }
        }
    }
    let ans = if dist[d.0][d.1] == inf {
        -1
    } else {
        dist[d.0][d.1] as i64
    };
    println!("{}", ans);
}
