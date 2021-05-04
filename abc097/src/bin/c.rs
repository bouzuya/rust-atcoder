use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    let mut t = BTreeSet::new();
    for l in 1..=5 {
        if s.len() < l {
            continue;
        }
        for i in 0..s.len() - l + 1 {
            t.insert(s[i..i + l].iter().cloned().collect::<String>());
        }
    }
    println!("{}", t.iter().skip(k - 1).next().unwrap());
}
