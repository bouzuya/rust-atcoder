use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    };

    let mut map = BTreeMap::new();
    for (t, d) in td {
        map.entry(t).or_insert_with(Vec::new).push(d);
    }

    let mut pq = BinaryHeap::new();
    let mut count = 0_usize;
    let mut cur = 0_usize;
    for (t, ds) in map {
        while let Some(Reverse(end_inclusive)) = pq.pop() {
            if cur >= t {
                pq.push(Reverse(end_inclusive));
                break;
            }
            if end_inclusive < cur {
                continue;
            }
            count += 1;
            cur += 1;
        }

        for d in ds {
            pq.push(Reverse(t + d));
        }
        cur = cur.max(t);
    }
    while let Some(Reverse(end_inclusive)) = pq.pop() {
        if end_inclusive < cur {
            continue;
        }
        count += 1;
        cur += 1;
    }

    let ans = count;
    println!("{}", ans);
}
