use std::collections::{BTreeMap, BTreeSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        q: usize,
        xy: [(Usize1, Usize1); q]
    };
    let mut val2idx = BTreeMap::new();
    for a_i in a.iter().copied() {
        let len = val2idx.len();
        val2idx.entry(a_i).or_insert(len);
    }
    for b_i in b.iter().copied() {
        let len = val2idx.len();
        val2idx.entry(b_i).or_insert(len);
    }
    let mut cm_a = vec![];
    let mut cm_b = vec![];
    let mut set_a = BTreeSet::new();
    let mut set_b = BTreeSet::new();
    for (a_i, b_i) in a.iter().copied().zip(b.iter().copied()) {
        set_a.insert(*val2idx.get(&a_i).unwrap());
        set_b.insert(*val2idx.get(&b_i).unwrap());
        cm_a.push((set_a.len(), *set_a.iter().rev().next().unwrap()));
        cm_b.push((set_b.len(), *set_b.iter().rev().next().unwrap()));
    }
    let ans = xy
        .into_iter()
        .map(|(x_i, y_i)| cm_a[x_i] == cm_b[y_i])
        .collect::<Vec<bool>>();
    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
