use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        capital_r: usize,
        capital_c: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        c: [Chars; capital_r],
    }

    let inf = capital_r * capital_c + 1;
    let mut dist = vec![vec![inf; capital_c]; capital_r];
    let mut deque = VecDeque::new();
    dist[s.0][s.1] = 0_usize;
    deque.push_back((s.0, s.1));
    while let Some((i, j)) = deque.pop_front() {
        if (i, j) == g {
            println!("{}", dist[i][j]);
            return;
        }

        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..capital_r as i64).contains(&nr) || !(0..capital_c as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if c[nr][nc] == '#' {
                continue;
            }
            if dist[nr][nc] != inf {
                continue;
            }
            dist[nr][nc] = dist[i][j] + 1;
            deque.push_back((nr, nc));
        }
    }
}
