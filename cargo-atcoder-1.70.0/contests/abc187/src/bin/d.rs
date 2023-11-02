use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    };
    ab.sort_by_key(|(a, b)| Reverse(2 * a + b));
    let sum_a = ab.iter().map(|(a, _)| a).copied().sum::<i64>();
    let mut s = -sum_a;
    for (i, (a, b)) in ab.iter().copied().enumerate() {
        s += 2 * a + b;
        if s > 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
