use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: Usize1,
        c: Usize1,
        s: [Chars; h],
    };
    let inf = h * w + 1;
    let mut d = vec![vec![inf; w]; h];
    let mut deque = VecDeque::new();
    d[r][c] = 0;
    deque.push_back((r, c));
    while let Some((r, c)) = deque.pop_front() {
        let nd = d[r][c] + 1;
        let cs = vec!['v', '>', '<', '^'];
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (i, (dr, dc)) in dir.iter().enumerate() {
            let (nr, nc) = (r as i64 + dr, c as i64 + dc);
            if !(0..h as i64).contains(&nr) {
                continue;
            }
            if !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if (s[nr][nc] == '.' || s[nr][nc] == cs[i]) && d[nr][nc] > nd {
                d[nr][nc] = nd;
                deque.push_back((nr, nc));
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!(
                "{}",
                if s[i][j] == '#' {
                    '#'
                } else if d[i][j] == inf {
                    'x'
                } else {
                    'o'
                }
            );
        }
        println!();
    }
}
