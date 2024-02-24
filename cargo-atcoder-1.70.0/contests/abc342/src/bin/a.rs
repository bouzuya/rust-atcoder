use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut map = HashMap::new();
    for s_i in s.iter().copied() {
        *map.entry(s_i).or_insert(0) += 1;
    }
    let mut min = (1_usize << 60, ' ');
    for (k, v) in map {
        if v < min.0 {
            min = (v, k);
        }
    }
    let ans = s.iter().position(|c| c == &min.1).unwrap() + 1;
    println!("{}", ans);
}
