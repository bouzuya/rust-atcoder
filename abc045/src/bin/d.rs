use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(i64, i64); n],
    };

    let mut map = HashMap::new();
    for (a, b) in ab.iter().copied() {
        for y in (a - 1).max(2)..=(a + 1).min(h as i64 - 1) {
            for x in (b - 1).max(2)..=(b + 1).min(w as i64 - 1) {
                *map.entry((y, x)).or_insert(0) += 1;
            }
        }
    }

    let mut ans = vec![0_usize; 10];
    for count in map.values().copied() {
        ans[count] += 1;
    }
    ans[0] = (h - 2) * (w - 2) - map.len();

    for a in ans {
        println!("{}", a);
    }
}
