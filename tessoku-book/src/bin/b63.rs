use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        c: [Chars; h],
    };

    let inf = h * w + 1;
    let mut dist = vec![vec![inf; w]; h];
    let mut deque = VecDeque::new();
    deque.push_back(s);
    dist[s.0][s.1] = 0;
    while let Some((i, j)) = deque.pop_front() {
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if c[nr][nc] == '#' {
                continue;
            }
            if dist[nr][nc] != inf {
                continue;
            }
            deque.push_back((nr, nc));
            dist[nr][nc] = dist[i][j] + 1;
        }
    }

    let ans = dist[g.0][g.1];
    println!("{}", ans);
}
