use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let mut dp = HashMap::new();
    for i in 0..n {
        input! {
            l_i: usize,
            a_i: [usize; l_i]
        }
        if i == 0 {
            for a_ij in a_i {
                *dp.entry(a_ij).or_insert(0) += 1;
            }
            continue;
        }
        let mut next = HashMap::new();
        for a_ij in a_i {
            for (&k, &v) in dp.iter() {
                if a_ij.checked_mul(k).is_none() {
                    continue;
                }
                if a_ij * k > x {
                    continue;
                }
                *next.entry(a_ij * k).or_insert(0) += v;
            }
        }
        dp = next;
    }
    let ans = dp.get(&x).unwrap_or(&0);
    println!("{}", ans);
}
