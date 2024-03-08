use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut set = HashSet::new();
    for (i, s_i) in s.iter().copied().enumerate() {
        set.insert(s_i);
        if set.len() == 3 {
            println!("{}", i + 1);
            return;
        }
    }
}
