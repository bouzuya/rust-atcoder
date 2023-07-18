use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let f = |start: (usize, usize)| -> Vec<Vec<usize>> {
        let mut dist = vec![vec![h * w; w]; h];
        let mut deque = VecDeque::new();
        dist[start.0][start.1] = 0;
        deque.push_back(start);
        while let Some((i, j)) = deque.pop_front() {
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if s[nr][nc] == '#' {
                    continue;
                }
                let nd = dist[i][j] + 1;
                if dist[nr][nc] <= nd {
                    continue;
                }
                dist[nr][nc] = nd;
                deque.push_back((nr, nc));
            }
        }
        dist
    };

    let g = |dist: &[Vec<usize>]| -> ((usize, usize), usize) {
        let mut pos = (0_usize, 0_usize);
        let mut max = 0_usize;
        for i in (0..h).rev() {
            for j in (0..w).rev() {
                if dist[i][j] == h * w {
                    continue;
                }
                if dist[i][j] > max {
                    max = dist[i][j];
                    pos = (i, j);
                }
            }
        }
        (pos, max)
    };

    let mut start = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                start = (i, j);
                break;
            }
        }
    }
    let dist1 = f(start);
    let (start, _) = g(&dist1);
    let dist2 = f(start);
    let (_, ans) = g(&dist2);

    println!("{}", ans);
}
