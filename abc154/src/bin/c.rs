use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let ans = a.iter().copied().collect::<HashSet<usize>>().len() == a.len();
    println!("{}", if ans { "YES" } else { "NO" });
}
