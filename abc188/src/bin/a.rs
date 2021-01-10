use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let ans = max(x, y) < min(x, y) + 3;
    println!("{}", if ans { "Yes" } else { "No" });
}
