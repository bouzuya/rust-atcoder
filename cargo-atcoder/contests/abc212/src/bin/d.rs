use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut heap = BinaryHeap::new();
    let mut s = 0_i64;
    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! { x: i64 };
                heap.push(Reverse(x - s));
            }
            2 => {
                input! { x: i64 };
                s += x;
            }
            3 => {
                if let Some(Reverse(x)) = heap.pop() {
                    println!("{}", x + s);
                }
            }
            _ => unreachable!(),
        }
    }
}
