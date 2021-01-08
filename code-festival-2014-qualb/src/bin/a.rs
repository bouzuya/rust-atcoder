use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = max(a, b);
    println!("{}", ans);
}
