use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut t = VecDeque::new();
    t.push_back(n);
    for (i, s_i) in s.iter().copied().enumerate().rev() {
        match s_i {
            'L' => t.push_back(i),
            'R' => t.push_front(i),
            _ => unreachable!(),
        }
    }
    for (i, t_i) in t.iter().copied().enumerate() {
        print!("{}{}", t_i, if i == t.len() - 1 { '\n' } else { ' ' });
    }
}
