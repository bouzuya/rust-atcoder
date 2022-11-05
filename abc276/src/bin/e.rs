use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let s = {
        let mut s = (0, 0);
        for i in 0..h {
            for j in 0..w {
                if c[i][j] == 'S' {
                    s = (i, j);
                }
            }
        }
        s
    };

    let mut used = vec![vec![0; w]; h];
    let mut deques = vec![];
    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (dr, dc) in dir {
        let (nr, nc) = (s.0 as i64 + dr, s.1 as i64 + dc);
        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
            continue;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if c[nr][nc] == '#' {
            continue;
        }
        let mut deque = VecDeque::new();
        deque.push_back((nr, nc));
        deques.push(deque);
        used[nr][nc] = deques.len();
    }

    let mut ans = false;
    loop {
        let mut count = 0_usize;
        for index in 0..deques.len() {
            if let Some((i, j)) = deques[index].pop_front() {
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (dr, dc) in dir {
                    let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                    if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if (nr, nc) == s {
                        continue;
                    }
                    if c[nr][nc] == '#' {
                        continue;
                    }
                    if used[nr][nc] == index + 1 {
                        continue;
                    }
                    if used[nr][nc] != 0 {
                        ans = true;
                        break;
                    }
                    used[nr][nc] = index + 1;
                    deques[index].push_back((nr, nc));
                }
            } else {
                count += 1;
            }
        }
        if count == deques.len() {
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
