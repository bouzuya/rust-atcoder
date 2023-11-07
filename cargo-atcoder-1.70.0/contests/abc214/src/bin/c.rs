use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    };

    let inf = 1_000_000_000_usize;
    let mut min = vec![inf; n];
    let mut pq = BinaryHeap::new();

    for (i, t_i) in t.iter().copied().enumerate() {
        pq.push((Reverse(t_i), i));
    }

    while let Some((Reverse(t_i), i)) = pq.pop() {
        if min[i] <= t_i {
            continue;
        }
        min[i] = t_i;
        let j = (i + 1) % n;
        pq.push((Reverse(t_i + s[i]), j));
    }

    for a in min {
        println!("{}", a);
    }
}
