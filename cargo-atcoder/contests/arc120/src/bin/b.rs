use std::cmp;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let p = 998_244_353;
    let mut ans = 1_usize;
    for d in 0..h + w - 1 {
        let mut b = (false, false);
        for i in 0..=cmp::min(d, h - 1) {
            let j = d - i;
            if i < h && j < w {
                match s[i][j] {
                    'B' => b.0 = true,
                    'R' => b.1 = true,
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }
        match b {
            (true, true) => ans *= 0,
            (true, false) | (false, true) => ans *= 1,
            (false, false) => ans *= 2,
        }
        ans %= p;
    }
    println!("{}", ans);
}
