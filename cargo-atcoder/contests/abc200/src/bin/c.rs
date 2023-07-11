use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut map = HashMap::new();
    for a_i in a {
        *map.entry(a_i % 200).or_insert(0) += 1;
    }

    let mut ans = 0_usize;
    for (_, c) in map {
        if c < 2 {
            continue;
        }
        ans += c * (c - 1) / 2;
    }

    println!("{}", ans);
}
