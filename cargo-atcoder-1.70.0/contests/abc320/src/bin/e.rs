use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(usize, usize, usize); m],
    };

    let mut events = BinaryHeap::new();
    for (t, w, s) in tws {
        events.push(Reverse((t, 1, w, s)));
    }
    let mut people = BinaryHeap::new();
    for i in 0..n {
        people.push(Reverse(i));
    }
    let mut sum = vec![0_usize; n];
    while let Some(Reverse((t, typ, w, s))) = events.pop() {
        match typ {
            0 => {
                let p = w;
                people.push(Reverse(p));
            }
            1 => {
                if let Some(Reverse(p)) = people.pop() {
                    sum[p] += w;
                    events.push(Reverse((t + s, 0, p, 0)));
                }
            }
            _ => unreachable!(),
        }
    }
    for sum_i in sum {
        println!("{}", sum_i);
    }
}
