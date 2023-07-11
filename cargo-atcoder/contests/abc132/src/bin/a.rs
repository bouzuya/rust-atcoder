use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut map = HashMap::new();
    for s_i in s {
        *map.entry(s_i).or_insert(0) += 1;
    }
    let ans = map.len() == 2 && map.iter().next().unwrap().1 == &2;
    println!("{}", if ans { "Yes" } else { "No" });
}
