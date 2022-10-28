use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = HashMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1;
    }

    let mut ans = 0_usize;
    for (_, v) in map {
        if v < 3 {
            continue;
        }
        ans += v * (v - 1) * (v - 2) / 6;
    }

    println!("{}", ans);
}
