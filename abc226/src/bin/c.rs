use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    };
    let mut tech = vec![];
    for _ in 0..n {
        input! {
            t: usize,
            k: usize,
            a: [Usize1; k],
        }
        tech.push((t, a));
    }

    let mut sum = 0_usize;
    let mut h = vec![false; n];
    let mut q = VecDeque::new();
    h[n - 1] = true;
    sum += tech[n - 1].0;
    for v in tech[n - 1].1.iter().copied() {
        q.push_back(v);
    }
    while let Some(x) = q.pop_front() {
        if h[x] {
            continue;
        }
        h[x] = true;
        sum += tech[x].0;

        for v in tech[x].1.iter().copied() {
            if !h[v] {
                q.push_back(v);
            }
        }
    }

    let ans = sum;
    println!("{}", ans);
}
