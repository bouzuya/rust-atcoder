use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = max(0, a - 2 * b);
    println!("{}", ans);
}
