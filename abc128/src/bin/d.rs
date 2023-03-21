use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        v: [i64; n],
    };

    let mut max = 0_i64;
    for left in 0..=k.min(n) {
        for right in 0..=k.min(n) - left {
            let mut sum = 0_i64;
            let mut bag = BinaryHeap::new();
            for i in 0..left {
                sum += v[i];
                bag.push(Reverse(v[i]));
            }
            for i in 0..right {
                sum += v[n - 1 - i];
                bag.push(Reverse(v[n - 1 - i]));
            }
            for _ in 0..k.saturating_sub(left + right) {
                if let Some(Reverse(x)) = bag.pop() {
                    if x < 0_i64 {
                        sum -= x;
                    }
                }
            }
            max = max.max(sum);
        }
    }

    let ans = max;
    println!("{}", ans);
}
