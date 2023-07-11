use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = !(s.iter().find(|c| c.is_uppercase()).is_none()
        || s.iter().find(|c| c.is_lowercase()).is_none()
        || s.len() != s.iter().collect::<BTreeSet<_>>().len());
    println!("{}", if ans { "Yes" } else { "No" });
}
