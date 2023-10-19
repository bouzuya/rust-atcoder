use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut pq = BinaryHeap::new();
    let mut s = 0_i64;
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: i64,
                }
                pq.push(Reverse(x - s));
            }
            2 => {
                input! {
                    x: i64,
                }
                s += x;
            }
            3 => {
                let Reverse(ans) = pq.pop().unwrap();
                println!("{}", ans + s);
            }
            _ => unreachable!(),
        }
    }
}
