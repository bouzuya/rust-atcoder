use std::{cmp, collections::BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    };
    let mut max_len = 0;
    let mut map = BTreeMap::new();
    let mut r = 0;
    for l in 0..n {
        while r < n && (map.len() < k || map.contains_key(&a[r])) {
            *map.entry(a[r]).or_insert(0) += 1;
            r += 1;
        }
        if map.len() > k {
            break;
        }

        max_len = cmp::max(max_len, r - l);

        if r == l {
            r += 1;
        } else {
            let entry = map.entry(a[l]).or_insert(0);
            *entry -= 1;
            if *entry == 0 {
                map.remove(&a[l]);
            }
        }
    }
    let ans = max_len;
    println!("{}", ans);
}
