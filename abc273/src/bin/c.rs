use std::collections::{BTreeSet, HashMap};

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let set = a
        .iter()
        .copied()
        .collect::<BTreeSet<usize>>()
        .into_iter()
        .collect::<Vec<usize>>();
    let mut map = HashMap::new();
    for a_i in a {
        let count = set.len() - 1 - set.lower_bound(&a_i);
        *map.entry(count).or_insert(0) += 1;
    }
    for i in 0..n {
        println!("{}", map.get(&i).unwrap_or(&0));
    }
}
