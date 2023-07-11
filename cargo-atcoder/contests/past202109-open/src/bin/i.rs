use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = 0;
    let mut b = vec![];
    for a_i in a {
        let mut x = a_i;
        while x % 2 == 0 {
            x /= 2;
            count += 1;
        }
        b.push(x);
    }

    let mut heap = b
        .into_iter()
        .map(Reverse)
        .collect::<BinaryHeap<Reverse<usize>>>();
    for _ in 0..count {
        match heap.pop() {
            Some(Reverse(x)) => {
                heap.push(Reverse(x * 3));
            }
            None => unreachable!(),
        }
    }

    let Reverse(ans) = heap.pop().unwrap();
    println!("{}", ans);
}
