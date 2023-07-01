use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut t = vec![vec![h * w; w]; h];
    for i in 0..h {
        for j in 0..w {
            t[i][j] = match s[i][j] {
                's' => 0,
                'n' => 1,
                'u' => 2,
                'k' => 3,
                'e' => 4,
                _ => 5,
            };
        }
    }

    if t[0][0] != 0 {
        println!("No");
        return;
    }

    let mut used = vec![vec![false; w]; h];
    let mut deque = VecDeque::new();
    deque.push_back((0, 0, 0));
    used[0][0] = true;

    while let Some((i, j, k)) = deque.pop_front() {
        if i == h - 1 && j == w - 1 {
            println!("Yes");
            return;
        }

        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let nk = (k + 1) % 5;
            if t[nr][nc] != nk {
                continue;
            }
            if used[nr][nc] {
                continue;
            }
            used[nr][nc] = true;
            deque.push_back((nr, nc, nk));
        }
    }

    println!("No");
}
