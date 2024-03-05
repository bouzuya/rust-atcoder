use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    };
    let mut ok = 0_usize;
    let mut ng = 1 << 60;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut sum = 0_usize;
        for a_i in a.iter().copied() {
            if a_i < mid {
                sum += (mid - a_i + k - 1) / k;
            }
        }
        if sum <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let min = ok;
    let mut count = 0_usize;
    let mut b = a.clone();
    for b_i in b.iter_mut() {
        if *b_i < min {
            let c = (min - *b_i + k - 1) / k;
            count += c;
            *b_i += c * k;
        }
    }

    let mut pq = BinaryHeap::new();
    for (i, b_i) in b.iter().copied().enumerate() {
        pq.push(Reverse((b_i, i)));
    }

    for _ in 0..m - count {
        let Reverse((b_i, i)) = pq.pop().unwrap();
        pq.push(Reverse((b_i + k, i)));
    }

    let mut ans = vec![0; n];
    for Reverse((b_i, i)) in pq {
        ans[i] = b_i;
    }
    for a in ans {
        println!("{}", a);
    }
}
