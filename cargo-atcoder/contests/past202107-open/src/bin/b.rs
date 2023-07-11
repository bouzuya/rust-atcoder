use std::cmp;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let a2 = cmp::min(a, b * c);
    let ans = a2 as f64 / b as f64;
    println!("{}", ans);
}
