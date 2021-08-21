use std::collections::BTreeSet;

use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    let n = s.len();
    let mut is = (0..n).collect::<Vec<usize>>();
    let mut set = BTreeSet::new();
    loop {
        let mut t = vec![];
        for i in is.clone() {
            t.push(s[i]);
        }
        set.insert(t.iter().collect::<String>());
        if !is.next_permutation() {
            break;
        }
    }
    println!("{}", set.iter().nth(k - 1).unwrap());
}
