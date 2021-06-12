use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        b: [Usize1; n],
    };
    let mut res = VecDeque::new();
    let mut b = b.into_iter().collect::<VecDeque<_>>();
    for _ in 0..n {
        match b.iter().enumerate().rposition(|(j, &b_j)| j == b_j) {
            Some(j) => res.push_front(b.remove(j).unwrap()),
            None => return println!("-1"),
        }
    }
    for r in res {
        println!("{}", r + 1);
    }
}
