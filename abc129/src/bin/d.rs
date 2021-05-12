use std::cmp;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let sl = {
        let mut sl = vec![vec![0; w]; h];
        for i in 0..h {
            let mut c = 0;
            for j in 0..w {
                match s[i][j] {
                    '.' => {
                        c += 1;
                        sl[i][j] = c;
                    }
                    '#' => {
                        c = 0;
                    }
                    _ => unreachable!(),
                }
            }
        }
        sl
    };

    let sr = {
        let mut sr = vec![vec![0; w]; h];
        for i in 0..h {
            let mut c = 0;
            for j in (0..w).rev() {
                match s[i][j] {
                    '.' => {
                        c += 1;
                        sr[i][j] = c;
                    }
                    '#' => {
                        c = 0;
                    }
                    _ => unreachable!(),
                }
            }
        }
        sr
    };

    let st = {
        let mut st = vec![vec![0; w]; h];
        for j in 0..w {
            let mut c = 0;
            for i in 0..h {
                match s[i][j] {
                    '.' => {
                        c += 1;
                        st[i][j] = c;
                    }
                    '#' => {
                        c = 0;
                    }
                    _ => unreachable!(),
                }
            }
        }
        st
    };

    let sb = {
        let mut sb = vec![vec![0; w]; h];
        for j in 0..w {
            let mut c = 0;
            for i in (0..h).rev() {
                match s[i][j] {
                    '.' => {
                        c += 1;
                        sb[i][j] = c;
                    }
                    '#' => {
                        c = 0;
                    }
                    _ => unreachable!(),
                }
            }
        }
        sb
    };

    let mut max_v = None;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let v = sl[i][j] + sr[i][j] + st[i][j] + sb[i][j] + 1 - 4;
            max_v = Some(match max_v {
                None => v,
                Some(pv) => cmp::max(pv, v),
            });
        }
    }
    let ans = max_v.unwrap();
    println!("{}", ans);
}
