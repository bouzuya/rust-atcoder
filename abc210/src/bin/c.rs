use std::{cmp, collections::BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    };
    let mut map = BTreeMap::new();
    for i in 0..k {
        *map.entry(c[i]).or_insert(0) += 1;
    }
    let mut count = map.len();
    for i in k..n {
        let c_i = c[i];
        *map.entry(c_i).or_insert(0) += 1;
        let entry = map.entry(c[i - k]).or_insert(0);
        *entry -= 1;
        if *entry <= 0 {
            map.remove(&c[i - k]);
        }
        count = cmp::max(count, map.len());
    }
    let ans = count;
    println!("{}", ans);
}
