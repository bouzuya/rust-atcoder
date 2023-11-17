use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        p: [[f64; 6]; 3],
    };
    let mut map = HashMap::new();
    map.insert(0, 1_f64);
    for i in 0..3 {
        let mut next = HashMap::new();
        for j in 0..6 {
            for (k, v) in &map {
                *next.entry(k + j + 1).or_insert(0_f64) += v * p[i][j] / 100_f64;
            }
        }
        map = next;
    }
    for k in 1..=18 {
        let ans = *map.get(&k).unwrap_or(&0_f64);
        println!("{}", ans);
    }
}
