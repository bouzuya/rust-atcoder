use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let b = a
        .into_iter()
        .enumerate()
        .map(|(i, a_i)| a_i * if i % 2 == 0 { 1 } else { -1 })
        .collect::<Vec<i64>>();
    let c = std::iter::once(0)
        .chain(b.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();
    let mut map = HashMap::new();
    for c_i in c {
        *map.entry(c_i).or_insert(0) += 1;
    }
    let mut ans = 0_i64;
    for (_, v) in map {
        ans += v * (v - 1) / 2;
    }
    println!("{}", ans);
}
