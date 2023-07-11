use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut map = HashMap::new();
    for c in s {
        *map.entry(c).or_insert(0) += 1_usize;
    }
    println!(
        "{}",
        "ABCDEF"
            .chars()
            .map(|c| (*map.get(&c).unwrap_or(&0)).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
