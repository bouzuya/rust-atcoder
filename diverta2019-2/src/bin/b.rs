use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let mut map = HashMap::new();
    for (x1, y1) in xy.iter().copied() {
        for (x2, y2) in xy.iter().copied() {
            let p = x1 - x2;
            let q = y1 - y2;
            if p != 0 || q != 0 {
                *map.entry((p, q)).or_insert(0_usize) += 1;
            }
        }
    }

    let ans = n - *map.values().max().unwrap_or(&0);
    println!("{}", ans);
}
