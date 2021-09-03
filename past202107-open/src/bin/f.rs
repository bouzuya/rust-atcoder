use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        dst: [(usize, usize, usize); n],
    };
    let mut days = BTreeMap::new();
    for (d_i, s_i, t_i) in dst {
        days.entry(d_i).or_insert_with(|| vec![]).push((s_i, t_i));
    }

    for (_, st) in days {
        let mut count = vec![0; 24 + 1];
        for (s_i, t_i) in st {
            count[s_i] += 1;
            count[t_i] -= 1;
        }
        for i in 1..24 {
            count[i] += count[i - 1];
        }
        if count.iter().any(|&c_i| c_i > 1) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
