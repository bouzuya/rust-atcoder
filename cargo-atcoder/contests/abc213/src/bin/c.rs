use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(Usize1, Usize1); n],
    };
    let mut map_r = BTreeMap::new();
    let mut map_c = BTreeMap::new();
    for (a_i, b_i) in ab.iter().copied() {
        map_r.entry(a_i).or_insert(0);
        map_c.entry(b_i).or_insert(0);
    }
    for (i, (_, v)) in map_r.iter_mut().enumerate() {
        *v = i;
    }
    for (i, (_, v)) in map_c.iter_mut().enumerate() {
        *v = i;
    }
    for (a_i, b_i) in ab {
        println!(
            "{} {}",
            map_r.get(&a_i).unwrap() + 1,
            map_c.get(&b_i).unwrap() + 1
        );
    }
}
