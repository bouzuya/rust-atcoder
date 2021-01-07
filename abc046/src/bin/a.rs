use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let set = [a, b, c].iter().map(|&x| x).collect::<BTreeSet<usize>>();
    let ans = set.len();
    println!("{}", ans);
}
