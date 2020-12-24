use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    };
    let min_s = a * (n - 1) + b;
    let max_s = a + b * (n - 1);
    let ans = max(0, max_s - min_s + 1);
    println!("{}", ans);
}
