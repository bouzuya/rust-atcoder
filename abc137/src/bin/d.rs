use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); n],
    };

    ab.sort_by_key(|&(a, _)| a);

    let mut sum = 0_usize;
    let mut pq = BinaryHeap::new();
    let mut index = 0_usize;
    for d in 1..=m {
        while index < n {
            let (a, b) = ab[index];
            if a <= d {
                pq.push(b);
            } else {
                break;
            }
            index += 1;
        }

        if let Some(x) = pq.pop() {
            sum += x;
        }
    }

    let ans = sum;
    println!("{}", ans);
}
