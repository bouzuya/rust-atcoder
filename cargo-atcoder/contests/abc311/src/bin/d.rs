use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };
    let mut used = vec![vec![vec![false; 4]; m]; n];
    let mut deque = VecDeque::new();
    used[1][1][0] = true;
    used[1][1][1] = true;
    used[1][1][2] = true;
    used[1][1][3] = true;
    deque.push_back((1, 1));
    while let Some(cur) = deque.pop_front() {
        // 0:u 1:r 2:d 3:l
        let dir = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (diri, (dr, dc)) in dir.iter().copied().enumerate() {
            let (mut i, mut j) = (cur.0, cur.1);
            loop {
                let (nr, nc) = ((i as i64 + dr) as usize, (j as i64 + dc) as usize);
                if used[nr][nc][diri] {
                    break;
                }
                used[nr][nc][diri] = true;
                if s[nr][nc] == '#' {
                    deque.push_back((i, j));
                    break;
                }
                i += dr;
                j += dc;
            }
        }
    }

    let mut count = 0_usize;
    for i in 0..n {
        for j in 0..m {
            let mut ok = false;
            for k in 0..4 {
                if s[i][j] == '.' && used[i][j][k] {
                    ok = true;
                    break;
                }
            }
            if ok {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
