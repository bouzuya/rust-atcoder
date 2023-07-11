use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };

    let mut map = HashMap::new();
    for mut s_i in s {
        s_i.sort();
        *map.entry(s_i).or_insert(0) += 1;
    }

    let mut count = 0_usize;
    for (_, v) in map {
        if v >= 2 {
            count += v * (v - 1) / 2;
        }
    }

    let ans = count;
    println!("{}", ans);
}
