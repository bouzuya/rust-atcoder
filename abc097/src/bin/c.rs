use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };

    let mut set = BTreeSet::new();
    let n = s.len();
    for len in 1..=k {
        for i in 0..n {
            if i + len > n {
                continue;
            }
            set.insert(s[i..i + len].iter().collect::<String>());
        }
    }

    let ans = set.iter().take(k).last().unwrap();
    println!("{}", ans);
}
