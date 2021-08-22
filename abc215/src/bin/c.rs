use std::collections::BTreeSet;

use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    let all_words = {
        let mut set = BTreeSet::new();
        let mut is = (0..s.len()).collect::<Vec<usize>>();
        loop {
            let t = is.iter().copied().map(|i| s[i]).collect::<String>();
            set.insert(t);
            if !is.next_permutation() {
                break;
            }
        }
        set
    };
    println!("{}", all_words.iter().nth(k - 1).unwrap());
}
