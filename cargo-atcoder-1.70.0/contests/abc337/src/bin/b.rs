use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut s = s.into_iter().collect::<VecDeque<char>>();
    while let Some(c) = s.pop_front() {
        if c == 'A' {
            continue;
        }
        s.push_front(c);
        break;
    }
    while let Some(c) = s.pop_back() {
        if c == 'C' {
            continue;
        }
        s.push_front(c);
        break;
    }
    let ans = s.iter().all(|c| *c == 'B');
    println!("{}", if ans { "Yes" } else { "No" });
}
