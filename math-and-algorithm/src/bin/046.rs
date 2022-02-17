use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        large_r: usize,
        large_c: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        c: [Chars; large_r],
    };

    let inf = large_r * large_c + 1;
    let mut dist = vec![vec![inf; large_c]; large_r];
    let mut deque = VecDeque::new();
    deque.push_back((s.0, s.1, 0));
    while let Some((y, x, d)) = deque.pop_front() {
        let nd = d + 1;
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (y as i64 + dr, x as i64 + dc);
            if !(0..large_r as i64).contains(&nr) || !(0..large_c as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if c[nr][nc] != '.' {
                continue;
            }
            if dist[nr][nc] != inf {
                continue;
            }
            dist[nr][nc] = nd;
            deque.push_back((nr, nc, nd));
        }
    }
    let ans = dist[g.0][g.1];
    println!("{}", ans);
}
