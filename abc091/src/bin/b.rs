use std::{cmp, collections::BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m],
    };
    let mut map = BTreeMap::new();
    for s_i in s {
        *map.entry(s_i).or_insert(0) += 1;
    }
    for t_i in t {
        *map.entry(t_i).or_insert(0) -= 1;
    }

    let mut max_v = 0;
    for (_, v) in map {
        max_v = cmp::max(max_v, v);
    }

    let ans = max_v;
    println!("{}", ans);
}
