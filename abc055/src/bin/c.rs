use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        s: usize,
        c: usize,
    };
    let x = min(s, c / 2);
    let c = c - x * 2;
    let ans = x + if c < 2 { 0 } else { c / 4 };
    println!("{}", ans);
}
