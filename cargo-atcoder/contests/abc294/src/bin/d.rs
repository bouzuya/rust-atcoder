use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut ans = vec![];
    let mut next = 0;
    let mut called = BinaryHeap::new();
    let mut done = HashSet::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                called.push(Reverse(next));
                next += 1;
            }
            2 => {
                input! {
                    x: Usize1,
                }
                done.insert(x);
            }
            3 => {
                while let Some(Reverse(x)) = called.pop() {
                    if done.contains(&x) {
                        continue;
                    }
                    called.push(Reverse(x));
                    ans.push(x);
                    break;
                }
            }
            _ => unreachable!(),
        }
    }
    for a in ans {
        println!("{}", a + 1);
    }
}
