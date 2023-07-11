use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let ans = n - a.len();
    println!("{}", ans);
}
