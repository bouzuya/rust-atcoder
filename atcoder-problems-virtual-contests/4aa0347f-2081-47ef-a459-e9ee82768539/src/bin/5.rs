use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _: usize,
        m: usize,
        ab: [(Usize1, Usize1); m]
    }
    let mut map = BTreeMap::new();
    for (a_i, b_i) in ab {
        *map.entry(a_i).or_insert(0) += 1;
        *map.entry(b_i).or_insert(0) += 1;
    }
    for (_, v) in map {
        if v % 2 != 0 {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
