use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};

fn main() {
    input! {
        h: i64,
        w: i64,
        n: usize,
        (s_r, s_c): (i64, i64),
        s: Chars,
        t: Chars,
    };
    let mut p = vec![s_c, s_c, s_r, s_r];
    for i in 0..n {
        match s[i] {
            'L' => p[0] -= 1,
            'R' => p[1] += 1,
            'U' => p[2] -= 1,
            'D' => p[3] += 1,
            _ => unreachable!(),
        }
        if p[0] <= 0 || p[1] > w || p[2] <= 0 || p[3] > h {
            println!("NO");
            return;
        }
        match t[i] {
            'L' => p[1] = max(1, p[1] - 1),
            'R' => p[0] = min(w, p[0] + 1),
            'U' => p[3] = max(1, p[3] - 1),
            'D' => p[2] = min(h, p[2] + 1),
            _ => unreachable!(),
        }
    }
    println!("YES");
}
