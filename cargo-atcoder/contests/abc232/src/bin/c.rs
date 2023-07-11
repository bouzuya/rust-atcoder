use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); m],
    };

    let mut e1 = vec![BTreeSet::new(); n];
    for (u, v) in ab.iter().copied() {
        e1[u].insert(v);
        e1[v].insert(u);
    }
    let mut is = (0..n).collect::<Vec<usize>>();
    loop {
        let mut e2 = vec![BTreeSet::new(); n];
        for (u, v) in cd.iter().copied() {
            e2[is[u]].insert(is[v]);
            e2[is[v]].insert(is[u]);
        }
        if e1 == e2 {
            println!("Yes");
            return;
        }

        if !is.next_permutation() {
            break;
        }
    }
    println!("No");
}
