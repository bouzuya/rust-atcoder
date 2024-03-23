use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let s1 = (1 + k) * k / 2;
    let s2 = a
        .iter()
        .copied()
        .collect::<HashSet<usize>>()
        .into_iter()
        .filter(|a_i| a_i <= &k)
        .sum::<usize>();
    let ans = s1 - s2;
    println!("{}", ans);
}
