use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut map = HashMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1_usize;
    }

    let mut count = 0_usize;
    for (_, v) in map {
        if v < 2 {
            continue;
        }
        count += v * (v - 1) / 2;
    }
    let ans = count;
    println!("{}", ans);
}
