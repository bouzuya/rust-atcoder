use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };

    let mut map = HashMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1_usize;
    }
    if map.len() == m {
        println!("0");
        return;
    }
    let min_count = if map.len() < m {
        0
    } else {
        *map.iter().map(|(_, v)| v).min().unwrap()
    };
    let mut xs = vec![];
    let mut sum = 0_usize;
    for (k, v) in map.iter() {
        sum += (*v - min_count) * *k;
        xs.push(*k);
    }
    xs.sort();

    let mut group_sum = vec![0];
    for x in xs.iter().copied() {
        let prev = (m + x - 1) % m;
        if !map.contains_key(&prev) {
            group_sum.push(0);
        }
        let index = group_sum.len() - 1;
        group_sum[index] += map.get(&x).unwrap() * x;
    }
    if xs[0] == 0 && xs[xs.len() - 1] == m - 1 && group_sum.len() > 1 {
        group_sum[0] += group_sum[group_sum.len() - 1];
    }

    let ans = sum - group_sum.iter().max().unwrap();
    println!("{}", ans);
}
