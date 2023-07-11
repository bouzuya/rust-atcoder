use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let ans = cmp::min(n * a, b);
    println!("{}", ans);
}
