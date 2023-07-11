use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut min_cost = 100_000_000_000;
    for x in -100..=100 {
        let cost = a.iter().map(|&a_i| (a_i - x).pow(2)).sum::<i64>();
        min_cost = min(min_cost, cost);
    }
    let ans = min_cost;
    println!("{}", ans);
}
