use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    };
    let mut t = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {
            t_i: usize,
            k_i: usize,
            a_i: [Usize1; k_i]
        };
        t.push(t_i);
        a.push(a_i);
    }

    let mut pq = BinaryHeap::new();
    let mut used = vec![false; n];
    let mut sum = t[n - 1];
    used[n - 1] = true;
    pq.push(n - 1);
    while let Some(i) = pq.pop() {
        for j in a[i].iter().copied() {
            if used[j] {
                continue;
            }
            sum += t[j];
            used[j] = true;
            pq.push(j);
        }
    }

    let ans = sum;
    println!("{}", ans);
}
