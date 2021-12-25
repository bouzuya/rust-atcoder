use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    };
    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();
    let mut sum = 0_usize;
    let mut count = HashMap::new();
    for l in (0..=n).rev() {
        let s_l = s[l];
        sum += *count.get(&(s_l + k)).unwrap_or(&0);
        *count.entry(s_l).or_insert(0) += 1_usize;
    }

    let ans = sum;
    println!("{}", ans);
}
