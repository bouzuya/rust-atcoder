use std::collections::BTreeMap;

use proconio::input;

fn cumsum(a: &[i64]) -> Vec<i64> {
    std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect()
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let s = cumsum(&a);
    let mut map = BTreeMap::new();
    for &s_i in s.iter() {
        *map.entry(s_i).or_insert(0) += 1;
    }
    let mut count = 0_usize;
    for (_, v) in map {
        count += v * (v - 1) / 2;
    }
    let ans = count;
    println!("{}", ans);
}
