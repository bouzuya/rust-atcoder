use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let ans = min(n * a, b);
    println!("{}", ans);
}
