use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    };

    let sum = a.iter().sum::<usize>();
    let mut pq = BinaryHeap::new();
    for a_i in a {
        pq.push(Reverse(a_i));
    }

    if sum < l {
        pq.push(Reverse(l - sum));
    }

    let mut ans = 0_usize;
    loop {
        match (pq.pop(), pq.pop()) {
            (None, None) => unreachable!(),
            (None, Some(_)) => unreachable!(),
            (Some(Reverse(_)), None) => break,
            (Some(Reverse(x)), Some(Reverse(y))) => {
                ans += x + y;
                pq.push(Reverse(x + y));
            }
        }
    }
    println!("{}", ans);
}
