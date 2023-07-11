use proconio::input;
use proconio::marker::Chars;
use std::{cmp::max, collections::VecDeque};

fn f(h: usize, w: usize, s: &Vec<Vec<char>>, start: (usize, usize)) -> (usize, (usize, usize)) {
    let inf = h * w + 1;
    let mut d = vec![vec![inf; w]; h];
    let mut q = VecDeque::new();
    d[start.0][start.1] = 0;
    q.push_back(start);
    while let Some((r, c)) = q.pop_front() {
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (r as i64 + dr, c as i64 + dc);
            if (0..h as i64).contains(&nr) && (0..w as i64).contains(&nc) {
                let (nr, nc) = (nr as usize, nc as usize);
                if s[nr][nc] == '.' && d[nr][nc] == inf {
                    d[nr][nc] = d[r][c] + 1;
                    q.push_back((nr, nc));
                }
            }
        }
    }
    let mut max_d = (0, start);
    for r in 0..h {
        for c in 0..w {
            if d[r][c] != inf && d[r][c] > max_d.0 {
                max_d = (d[r][c], (r, c));
            }
        }
    }
    max_d
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut max_d = 0;
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == '#' {
                continue;
            }
            let (d, _) = f(h, w, &s, (r, c));
            max_d = max(max_d, d);
        }
    }
    let ans = max_d;
    println!("{}", ans);
}
