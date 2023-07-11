use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    for _ in 0..q {
        input! {
            t: char,
        }

        match t {
            'A' => {
                input! {
                    x: String,
                }
                right.push_back(x);
                if left.len() < right.len() {
                    left.push_back(right.pop_front().unwrap());
                }
            }
            'B' => {
                input! {
                    x: String,
                }
                left.push_back(x);
                if left.len() > right.len() + 1 {
                    right.push_front(left.pop_back().unwrap());
                }
            }
            'C' => {
                left.pop_front();
                if left.len() < right.len() {
                    left.push_back(right.pop_front().unwrap());
                }
            }
            'D' => {
                println!("{}", left.front().unwrap());
            }
            _ => unreachable!(),
        }
    }
}
