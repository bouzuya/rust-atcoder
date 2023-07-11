use std::cmp::min;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: [Chars; n],
    };
    let mut count_h = vec![vec![vec![0; 10]; m + 1]; n];
    for i in 0..n {
        for j in 0..m {
            let c = (s[i][j] as u8 - '0' as u8) as usize;
            for k in 0..10 {
                count_h[i][j + 1][k] = count_h[i][j][k] + if k == c { 1 } else { 0 };
            }
        }
    }
    for x in (1..=min(m, n)).rev() {
        for i in 0..n - x + 1 {
            for j in 0..m - x + 1 {
                let mut min_count = x * x;
                for k in 0..10 {
                    let mut count = 0;
                    for r in i..i + x {
                        count += count_h[r][j + x][k] - count_h[r][j][k];
                    }
                    min_count = min(min_count, x * x - count);
                }
                if min_count <= k {
                    println!("{}", x);
                    return;
                }
            }
        }
    }
    println!("0");
}
