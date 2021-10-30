use std::collections::HashSet;

use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        s: Chars,
    };
    let mut set = HashSet::new();
    let mut is = (0..3).collect::<Vec<usize>>();
    loop {
        let t = is.iter().map(|&i| s[i]).collect::<String>();
        set.insert(t);
        if !is.next_permutation() {
            break;
        }
    }
    let ans = set.len();
    println!("{}", ans);
}
