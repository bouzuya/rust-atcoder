use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut pq = BinaryHeap::new();
    let mut deque = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                deque.push_back(x);
            }
            2 => {
                let x = if let Some(Reverse(x)) = pq.pop() {
                    x
                } else {
                    deque.pop_front().unwrap()
                };
                println!("{}", x);
            }
            3 => {
                while let Some(x) = deque.pop_front() {
                    pq.push(Reverse(x));
                }
            }
            _ => unreachable!(),
        }
    }
}
