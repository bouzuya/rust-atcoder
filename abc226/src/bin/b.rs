use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            a_i: [usize; l]
        }
        set.insert(a_i);
    }
    let ans = set.len();
    println!("{}", ans);
}
