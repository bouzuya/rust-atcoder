use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut p_a = (0, 0);
    let mut p_g = (0, 0);
    let mut p_s = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match s[i][j] {
                'a' => p_a = (i, j),
                'g' => p_g = (i, j),
                's' => p_s = (i, j),
                _ => {}
            }
        }
    }

    let inf = 1_usize << 60;
    let mut dist = vec![vec![vec![vec![inf; w]; h]; w]; h];
    let mut deque = VecDeque::new();
    dist[p_a.0][p_a.1][p_s.0][p_s.1] = 0_usize;
    deque.push_back((p_a.0, p_a.1, p_s.0, p_s.1));
    while let Some((r1, c1, r2, c2)) = deque.pop_front() {
        let d = dist[r1][c1][r2][c2];
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (r2 as i64 + dr, c2 as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr2, nc2) = (nr as usize, nc as usize);
            if nr2 == r1 && nc2 == c1 {
                let (nr1, nc1) = (nr2 as i64 + dr, nc2 as i64 + dc);
                if !(0..h as i64).contains(&nr1) || !(0..w as i64).contains(&nc1) {
                    continue;
                }
                let (nr1, nc1) = (nr1 as usize, nc1 as usize);
                if dist[nr1][nc1][nr2][nc2] == inf && s[nr1][nc1] != '#' {
                    dist[nr1][nc1][nr2][nc2] = d + 1;
                    deque.push_back((nr1, nc1, nr2, nc2));
                }
            } else if dist[r1][c1][nr2][nc2] == inf && s[nr2][nc2] != '#' {
                dist[r1][c1][nr2][nc2] = d + 1;
                deque.push_back((r1, c1, nr2, nc2));
            }
        }
    }

    let mut min = inf;
    for i in 0..h {
        for j in 0..w {
            min = min.min(dist[p_g.0][p_g.1][i][j]);
        }
    }

    let ans = min;
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
