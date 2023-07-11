use proconio::input;
use std::cmp;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    println!("{} {}", cmp::max(a, b), a + b);
}
