use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut dragon = VecDeque::new();
    for i in 1..=n {
        dragon.push_back((i as i64, 0_i64));
    }

    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            1 => {
                input! {
                    c: char
                }
                let (x, y) = dragon[0];
                match c {
                    'R' => dragon.push_front((x + 1, y)),
                    'L' => dragon.push_front((x - 1, y)),
                    'U' => dragon.push_front((x, y + 1)),
                    'D' => dragon.push_front((x, y - 1)),
                    _ => unreachable!(),
                }
                dragon.pop_back();
            }
            2 => {
                input! {
                    p: Usize1,
                }
                println!("{} {}", dragon[p].0, dragon[p].1);
            }
            _ => unreachable!(),
        }
    }
}
