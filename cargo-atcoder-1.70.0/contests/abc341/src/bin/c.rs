use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        _n: usize,
        t: Chars,
        s: [Chars; h],
    };

    let mut set = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            let mut ok = true;
            let mut cur = (i, j);
            for t_i in t.iter().copied() {
                let (i, j) = cur;
                let (di, dj) = match t_i {
                    'L' => (0, -1),
                    'R' => (0, 1),
                    'U' => (-1, 0),
                    'D' => (1, 0),
                    _ => unreachable!(),
                };
                let (ni, nj) = (i as i64 + di, j as i64 + dj);
                if !(0..h as i64).contains(&ni) || !(0..w as i64).contains(&nj) {
                    ok = false;
                    break;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                if s[ni][nj] == '#' {
                    ok = false;
                    break;
                }
                cur = (ni, nj);
            }

            if ok {
                set.insert(cur);
            }
        }
    }

    let ans = set.len();
    println!("{}", ans);
}
