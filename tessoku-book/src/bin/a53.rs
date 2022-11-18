use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut pq = BinaryHeap::new();
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    x: usize
                }
                pq.push(Reverse(x));
            }
            2 => {
                let Reverse(x) = pq.peek().unwrap();
                println!("{}", x);
            }
            3 => {
                pq.pop();
            }
            _ => unreachable!(),
        }
    }
}
