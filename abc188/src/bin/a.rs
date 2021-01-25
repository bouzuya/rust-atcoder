use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        x: i64,
        y: i64,
    };
    let ans = max(x, y) < min(x, y) + 3;
    println!("{}", if ans { "Yes" } else { "No" });
}
