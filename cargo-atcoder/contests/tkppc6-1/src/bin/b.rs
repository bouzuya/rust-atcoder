use std::{cmp, collections::HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    if m >= n {
        println!("{}", n);
        return;
    }

    let mut set = HashSet::new();
    for a_i in a {
        set.insert(a_i);
    }

    let ans = cmp::min(n, set.len() + m);
    println!("{}", ans);
}
