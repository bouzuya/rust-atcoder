use std::cmp;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let ans = cmp::min(a, b) + cmp::min(c, d);
    println!("{}", ans);
}
