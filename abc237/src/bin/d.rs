use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut deque = VecDeque::new();
    deque.push_back(n);
    for (i, s_i) in s.iter().copied().enumerate().rev() {
        match s_i {
            'L' => {
                deque.push_back(i);
            }
            'R' => {
                deque.push_front(i);
            }
            _ => unreachable!(),
        }
    }
    for (i, a_i) in deque.iter().copied().enumerate() {
        print!("{}{}", a_i, if i == n - 1 { '\n' } else { ' ' });
    }
}
