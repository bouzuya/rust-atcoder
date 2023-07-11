use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, i64); n],
    };
    let mut ks = vec![vec![]; n];
    for &(a_i, b_i) in ab.iter() {
        ks[a_i].push(b_i);
    }
    let mut pq = std::collections::BinaryHeap::new();
    let mut sum = 0;
    for k in 0..n {
        for &b_i in ks[k].iter() {
            pq.push(b_i);
        }
        sum += pq.pop().unwrap();
        println!("{}", sum);
    }
}
