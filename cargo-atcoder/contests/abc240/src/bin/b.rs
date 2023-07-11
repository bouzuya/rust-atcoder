use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let set = a.into_iter().collect::<HashSet<usize>>();
    let ans = set.len();
    println!("{}", ans);
}
