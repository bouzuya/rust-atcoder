use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        bc: [(usize, usize); m],
    };
    let mut pq = BinaryHeap::new();
    for a_i in a {
        pq.push((a_i, 1));
    }
    for (b, c) in bc {
        pq.push((c, b));
    }
    let mut count = 0_usize;
    let mut sum = 0_usize;
    while let Some((v, c)) = pq.pop() {
        if count + c <= n {
            sum += v * c;
            count += c;
        } else {
            sum += v * (n - count);
            count += n - count;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
