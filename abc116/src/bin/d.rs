use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut td: [(usize, usize); n],
    };
    td.sort_by_key(|&(t, d)| (Reverse(d), t));
    let mut pq = BinaryHeap::new();
    let mut set = BTreeSet::new();
    let mut sum = 0_usize;
    for (t_i, d_i) in td.iter().copied().take(k) {
        if !set.insert(t_i) {
            pq.push(Reverse(d_i));
        }
        sum += d_i;
    }
    let mut ans = sum + set.len() * set.len();
    for (t_i, d_i) in td.iter().copied().skip(k) {
        if !set.insert(t_i) {
            continue;
        }
        match pq.pop() {
            Some(Reverse(x)) => sum -= x,
            None => break,
        }
        sum += d_i;
        ans = ans.max(sum + set.len() * set.len());
    }
    println!("{}", ans);
}
