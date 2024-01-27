use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut s = s.into_iter().collect::<VecDeque<char>>();
    let first = s.pop_front().unwrap();
    if !first.is_ascii_uppercase() {
        println!("No");
        return;
    }
    if !s.iter().all(|c| c.is_ascii_lowercase()) {
        println!("No");
        return;
    }
    println!("Yes");
}
