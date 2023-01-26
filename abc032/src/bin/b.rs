use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };

    let n = s.len();
    let mut set = HashSet::new();
    for i in 0..n.saturating_sub(k - 1) {
        let t = s[i..i + k].iter().collect::<String>();
        set.insert(t);
    }
    let ans = set.len();
    println!("{}", ans);
}
