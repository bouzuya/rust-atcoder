use std::collections::VecDeque;

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        mut n: Bytes,
    };
    n.sort();
    n.reverse();
    let mut n = n
        .into_iter()
        .map(|b| (b - b'0') as usize)
        .collect::<VecDeque<usize>>();
    let mut n1 = n.pop_front().unwrap();
    let mut n2 = n.pop_front().unwrap();
    while let Some(x) = n.pop_front() {
        let p1 = (n1 * 10 + x) * n2;
        let p2 = n1 * (n2 * 10 + x);

        if p1 > p2 {
            n1 = n1 * 10 + x as usize;
        } else {
            n2 = n2 * 10 + x as usize;
        }
    }
    let ans = n1 * n2;
    println!("{}", ans);
}
