use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut count = 0_usize;
    let mut set = HashSet::new();
    for a_i in a {
        if !set.insert(a_i) {
            set.remove(&a_i);
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
