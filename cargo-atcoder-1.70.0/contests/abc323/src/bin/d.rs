use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        sc: [(usize, usize); n],
    };
    let mut count = HashMap::new();
    let mut pq = BinaryHeap::new();

    for (s, c) in sc {
        pq.push(Reverse(s));
        count.insert(s, c);
    }

    let mut ans = 0_usize;
    while let Some(Reverse(s)) = pq.pop() {
        let c = *count.get(&s).unwrap();
        if c == 0 {
            continue;
        }
        if c % 2 != 0 {
            ans += 1;
        }

        *count.entry(2 * s).or_insert(0) += c / 2;
        count.insert(s, 0);
        pq.push(Reverse(2 * s));
    }

    println!("{}", ans);
}
