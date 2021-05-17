use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let mut heap = BinaryHeap::new();
    for a_i in a {
        heap.push((a_i, a_i, 0));
    }

    for _ in 0..m {
        let (p, x, y) = heap.pop().unwrap();
        heap.push((p / 2_usize, x, y + 1));
    }

    let ans = heap.iter().map(|(v, _, _)| v).sum::<usize>();
    println!("{}", ans);
}
