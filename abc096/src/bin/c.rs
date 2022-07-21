use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    };
    let mut used = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' && !used[i][j] {
                let mut deque = VecDeque::new();
                deque.push_back((i, j));
                while let Some((r, c)) = deque.pop_front() {
                    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                    for (dr, dc) in dir {
                        let (nr, nc) = (r as i64 + dr, c as i64 + dc);
                        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                            continue;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        if s[nr][nc] == '#' && !used[nr][nc] {
                            used[nr][nc] = true;
                            deque.push_back((nr, nc));
                        }
                    }
                }
            }
        }
    }

    let mut ok = true;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' && !used[i][j] {
                ok = false;
            }
        }
    }
    let ans = ok;
    println!("{}", if ans { "Yes" } else { "No" });
}
