use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut win = vec![0_usize; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                win[i] += 1;
            }
        }
    }
    let mut v = win
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<Vec<(usize, usize)>>();
    v.sort_by_key(|&(c, i)| (Reverse(c), i));
    for (_, rank) in v {
        println!("{}", rank + 1);
    }
}
