use std::{cmp, collections::BinaryHeap};

use proconio::input;

#[derive(Clone, Debug)]
enum Op {
    Op1(usize),
    Op2(usize),
    Op3,
}

fn main() {
    input! {
        q: usize,
    };
    let mut op = vec![];
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize
                }
                op.push(Op::Op1(x));
            }
            2 => {
                input! {
                    x: usize
                }
                op.push(Op::Op2(x));
            }
            3 => {
                op.push(Op::Op3);
            }
            _ => unreachable!(),
        }
    }

    let mut op2 = vec![];
    let mut plus = 0_usize;
    for op in op.clone().into_iter().rev() {
        match op {
            Op::Op1(x) => op2.push(Op::Op1(x + plus)),
            Op::Op2(x) => {
                plus += x;
                op2.push(Op::Op2(x));
            }
            Op::Op3 => op2.push(Op::Op3),
        }
    }
    op2.reverse();
    let sum = plus;
    let mut heap = BinaryHeap::new();
    let mut plus = 0_usize;
    for op in op2 {
        match op {
            Op::Op1(x) => {
                heap.push(cmp::Reverse(x));
            }
            Op::Op2(x) => {
                plus += x;
            }
            Op::Op3 => {
                if let Some(cmp::Reverse(x)) = heap.pop() {
                    println!("{}", x - (sum - plus));
                }
            }
        }
    }
}
