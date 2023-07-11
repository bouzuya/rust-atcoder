use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let map = a
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });
    let ans = a
        .iter()
        .map(|a_i| *map.get(&a_i).unwrap())
        .collect::<Vec<usize>>();
    for a in ans {
        println!("{}", a + 1);
    }
}
