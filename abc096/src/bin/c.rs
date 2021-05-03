use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Bytes; h],
    };
    let mut painted = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'#' && !painted[i][j] {
                let mut count = 0;
                let mut q = VecDeque::new();
                count += 1;
                painted[i][j] = true;
                q.push_back((i, j));
                while let Some((i, j)) = q.pop_front() {
                    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                    for (dr, dc) in dir {
                        let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                            continue;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        if s[nr][nc] == b'#' && !painted[nr][nc] {
                            painted[nr][nc] = true;
                            count += 1;
                            q.push_back((nr, nc));
                        }
                    }
                }
                if count == 1 {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
