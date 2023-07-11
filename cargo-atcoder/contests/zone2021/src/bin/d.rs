use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut b = true;
    let mut t = VecDeque::new();
    for s_i in s {
        match s_i {
            'R' => b = !b,
            _ => {
                if b {
                    if let Some(x) = t.pop_back() {
                        if x != s_i {
                            t.push_back(x);
                            t.push_back(s_i);
                        }
                    } else {
                        t.push_back(s_i);
                    }
                } else {
                    if let Some(x) = t.pop_front() {
                        if x != s_i {
                            t.push_front(x);
                            t.push_front(s_i);
                        }
                    } else {
                        t.push_front(s_i);
                    }
                }
            }
        }
    }
    let ans = if b {
        t.iter().collect::<String>()
    } else {
        t.iter().rev().collect::<String>()
    };
    println!("{}", ans);
}
