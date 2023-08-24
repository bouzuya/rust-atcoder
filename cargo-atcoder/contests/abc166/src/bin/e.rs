use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let b = a
        .iter()
        .copied()
        .enumerate()
        .map(|(j, a_j)| -a_j + j as i64)
        .collect::<Vec<i64>>();
    let mut map = HashMap::new();
    for b_j in b {
        *map.entry(b_j).or_insert(0) += 1;
    }

    let mut count = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        if let Some(c) = map.get(&(a_i + i as i64)) {
            count += c;
        }
    }

    let ans = count;
    println!("{}", ans);
}
