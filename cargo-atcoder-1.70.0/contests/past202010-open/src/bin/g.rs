use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn ok(n: usize, m: usize, s: &[Vec<char>]) -> bool {
    let mut count = 0_usize;
    let mut visited = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '#' || visited[i][j] {
                continue;
            }

            count += 1;
            let mut deque = VecDeque::new();
            visited[i][j] = true;
            deque.push_back((i, j));
            while let Some((r, c)) = deque.pop_front() {
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (dr, dc) in dir {
                    let (nr, nc) = (r as i64 + dr, c as i64 + dc);
                    if !(0..n as i64).contains(&nr) || !(0..m as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if s[nr][nc] == '#' {
                        continue;
                    }
                    if visited[nr][nc] {
                        continue;
                    }
                    visited[nr][nc] = true;
                    deque.push_back((nr, nc));
                }
            }
        }
    }
    count == 1
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n],
    };

    let mut count = 0_usize;
    for i in 0..n {
        for j in 0..m {
            if s[i][j] != '#' {
                continue;
            }
            s[i][j] = '.';
            if ok(n, m, &s) {
                count += 1;
            }
            s[i][j] = '#';
        }
    }
    let ans = count;
    println!("{}", ans);
}
