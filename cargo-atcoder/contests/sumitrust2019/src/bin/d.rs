use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut set1 = BTreeSet::new();
    let mut set2 = BTreeSet::new();
    let mut set3 = BTreeSet::new();
    for c in s {
        for (c1, c2) in set2.iter() {
            set3.insert((*c1, *c2, c));
        }

        for c1 in set1.iter() {
            set2.insert((*c1, c));
        }

        set1.insert(c);
    }
    let ans = set3.len();
    println!("{}", ans);
}
