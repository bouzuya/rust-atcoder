use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut a = VecDeque::new();
    for (i, s_i) in s.iter().enumerate() {
        match s_i {
            'L' => a.push_front(i + 1),
            'R' => a.push_back(i + 1),
            'A' => {
                if a.is_empty() {
                    println!("ERROR");
                } else {
                    let a1 = a.pop_front().unwrap();
                    println!("{}", a1);
                }
            }
            'B' => {
                if a.len() <= 1 {
                    println!("ERROR");
                } else {
                    let a1 = a.pop_front().unwrap();
                    let a2 = a.pop_front().unwrap();
                    println!("{}", a2);
                    a.push_front(a1);
                }
            }
            'C' => {
                if a.len() <= 2 {
                    println!("ERROR");
                } else {
                    let a1 = a.pop_front().unwrap();
                    let a2 = a.pop_front().unwrap();
                    let a3 = a.pop_front().unwrap();
                    println!("{}", a3);
                    a.push_front(a2);
                    a.push_front(a1);
                }
            }
            'D' => {
                if a.is_empty() {
                    println!("ERROR");
                } else {
                    let a1 = a.pop_back().unwrap();
                    println!("{}", a1);
                }
            }
            'E' => {
                if a.len() <= 1 {
                    println!("ERROR");
                } else {
                    let a1 = a.pop_back().unwrap();
                    let a2 = a.pop_back().unwrap();
                    println!("{}", a2);
                    a.push_back(a1);
                }
            }
            'F' => {
                if a.len() <= 2 {
                    println!("ERROR");
                } else {
                    let a1 = a.pop_back().unwrap();
                    let a2 = a.pop_back().unwrap();
                    let a3 = a.pop_back().unwrap();
                    println!("{}", a3);
                    a.push_back(a2);
                    a.push_back(a1);
                }
            }
            _ => unreachable!(),
        }
    }
}
