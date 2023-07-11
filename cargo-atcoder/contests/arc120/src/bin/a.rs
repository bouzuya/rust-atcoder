use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let s = a
        .iter()
        .scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        })
        .collect::<Vec<u64>>();
    let m = a
        .iter()
        .scan(0, |acc, &i| {
            *acc = cmp::max(*acc, i);
            Some(*acc)
        })
        .collect::<Vec<u64>>();
    let mut sum = 0;
    for (i, (s, m)) in s.into_iter().zip(m.into_iter()).enumerate() {
        println!("{}", (i as u64 + 1) * m + sum + s);
        sum += s;
    }
}
