use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut map = BTreeMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1;
    }
    let mut count = 0;
    for (k, v) in map {
        count += if v == k {
            0
        } else if v > k {
            v - k
        } else if v < k {
            v
        } else {
            unreachable!()
        };
    }
    let ans = count;
    println!("{}", ans);
}
