use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut same = false;
    let mut sum = 0_usize;
    let mut map = HashMap::new();
    for (i, s_i) in s.iter().copied().rev().enumerate() {
        if map.contains_key(&s_i) {
            same = true;
        }
        let count = i - map.get(&s_i).unwrap_or(&0);
        sum += count;
        *map.entry(s_i).or_insert(0) += 1_usize;
    }
    let ans = sum + if same { 1 } else { 0 };
    println!("{}", ans);
}
