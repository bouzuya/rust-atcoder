use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    };
    let mut map = BTreeMap::new();
    for (a, b) in ab {
        *map.entry(a).or_insert(0) += b;
    }

    let mut count = 0_usize;
    for (a, b) in map {
        count += b;
        if count >= k {
            println!("{}", a);
            return;
        }
    }
}
