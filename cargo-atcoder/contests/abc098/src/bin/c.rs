use std::cmp;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut c = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] += if s[i] == 'E' { 1 } else { 0 } + c[i];
    }
    let mut min_c = n;
    for i in 0..n {
        let re = c[n] - c[i + 1];
        let lw = i - (c[i] - c[0]);
        min_c = cmp::min(min_c, lw + re);
    }
    let ans = min_c;
    println!("{}", ans);
}
