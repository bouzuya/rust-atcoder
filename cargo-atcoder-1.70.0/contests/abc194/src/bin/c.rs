use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut count = HashMap::new();
    for a_i in a.iter().copied() {
        *count.entry(a_i).or_insert(0) += 1;
    }
    let mut sum = 0_i64;
    for (a_i, c_i) in count.iter() {
        for (a_j, c_j) in count.iter() {
            sum += (a_i - a_j).pow(2) * c_i * c_j;
        }
    }
    let ans = sum / 2;
    println!("{}", ans);
}
