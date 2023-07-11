use std::{cmp::max, collections::BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut max_len = 0;
    let mut set = BTreeSet::new();
    let mut r = 0;
    for l in 0..n {
        while r < n {
            if !set.insert(a[r]) {
                break;
            }
            r += 1;
        }
        max_len = max(max_len, set.len());
        set.remove(&a[l]);
    }
    let ans = max_len;
    println!("{}", ans);
}
