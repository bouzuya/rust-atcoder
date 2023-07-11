use std::{cmp, collections::BTreeMap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut map = BTreeMap::new();
    for (a_i, b_i) in ab {
        *map.entry(cmp::max(a_i, b_i)).or_insert(0) += 1;
    }
    let ans = map.iter().filter(|(_, v)| **v == 1).count();
    println!("{}", ans);
}
