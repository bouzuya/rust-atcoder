#![deny(unused_imports)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = 15;
    let k = s.len();
    let ans = ((n - k) + s.into_iter().filter(|&c| c == 'o').count()) >= 8;
    println!("{}", if ans { "YES" } else { "NO" });
}
