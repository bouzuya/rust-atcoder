use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut count = 0_usize;
    let mut used = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if used[i][j] || s[i][j] == '.' {
                continue;
            }
            used[i][j] = true;

            let mut deque = VecDeque::new();
            deque.push_back((i, j));
            while let Some((i, j)) = deque.pop_front() {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                            continue;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        if used[nr][nc] {
                            continue;
                        }
                        if s[nr][nc] == '.' {
                            continue;
                        }
                        used[nr][nc] = true;
                        deque.push_back((nr, nc));
                    }
                }
            }

            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
