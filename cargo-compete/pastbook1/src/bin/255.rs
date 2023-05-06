use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut ab: [(Usize1, usize); n],
    }

    ab.sort();

    let mut sum = 0_usize;
    let mut index = 0_usize;
    let mut pq = BinaryHeap::new();
    for k in 0..n {
        while index < n {
            let (a, b) = ab[index];
            if a <= k {
                pq.push(b);
                index += 1;
            } else {
                break;
            }
        }

        sum += pq.pop().unwrap();
        println!("{}", sum);
    }
}
