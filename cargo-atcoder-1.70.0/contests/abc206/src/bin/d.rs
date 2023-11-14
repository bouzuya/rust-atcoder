use std::collections::{BTreeMap, BTreeSet};

use ac_library::Dsu;
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
    let mut dsu = Dsu::new(map.len());
    for i in 0..n / 2 {
        let (a_i, a_j) = (a[i], a[n - 1 - i]);
        dsu.merge(map[&a_i], map[&a_j]);
    }
    let ans = map.len() - dsu.groups().len();
    println!("{}", ans);
}
