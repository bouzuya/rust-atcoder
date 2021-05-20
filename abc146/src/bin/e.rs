use proconio::input;
use std::{collections::BTreeMap, iter};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let s: Vec<usize> = iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect();
    let mut sum = 0_i64;
    let mut map = BTreeMap::new();
    for (r, s_i) in s.iter().enumerate() {
        let b_i = (s_i - r) % k;
        sum += *map.entry(b_i).or_insert(0);
        *map.entry(b_i).or_insert(0) += 1;
        if r + 1 >= k {
            let l = r + 1 - k;
            *map.get_mut(&((s[l] - l) % k)).unwrap() -= 1;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
