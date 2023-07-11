use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut xy: [(usize, usize); n],
    };
    xy.sort_by_key(|&(x_i, _)| x_i);
    let mut pq = BinaryHeap::new();
    let mut sum = 0_usize;
    let mut index = 0;
    for d_i in 1..=d {
        while index < n && xy[index].0 <= d_i {
            pq.push(xy[index].1);
            index += 1;
        }
        if let Some(x) = pq.pop() {
            sum += x;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
