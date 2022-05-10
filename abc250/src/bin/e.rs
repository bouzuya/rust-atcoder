use std::collections::{BTreeMap, BTreeSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    };
    let mut map = BTreeMap::new();
    for c in a.iter().copied().chain(b.iter().copied()) {
        let len = map.len();
        map.entry(c).or_insert(len);
    }
    let mut tup_a = vec![];
    let mut tup_b = vec![];
    let mut set_a = BTreeSet::new();
    let mut set_b = BTreeSet::new();
    for (a_i, b_i) in a.iter().copied().zip(b.iter().copied()) {
        set_a.insert(map[&a_i]);
        set_b.insert(map[&b_i]);
        tup_a.push((set_a.len(), *set_a.iter().rev().next().unwrap()));
        tup_b.push((set_b.len(), *set_b.iter().rev().next().unwrap()));
    }
    for ans in xy.into_iter().map(|(x, y)| tup_a[x] == tup_b[y]) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
