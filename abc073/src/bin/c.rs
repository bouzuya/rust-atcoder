use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut set = HashSet::new();
    for a_i in a {
        if !set.insert(a_i) {
            set.remove(&a_i);
        }
    }
    let ans = set.len();
    println!("{}", ans);
}
