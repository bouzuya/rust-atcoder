use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut s = s.into_iter().collect::<VecDeque<char>>();
    while s.len() > 1 && s[0] == 'a' && s[s.len() - 1] == 'a' {
        s.pop_front();
        s.pop_back();
    }
    if s.len() <= 1 {
        println!("Yes");
        return;
    }

    while !s.is_empty() && s[s.len() - 1] == 'a' {
        s.pop_back();
    }

    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - 1 - i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
