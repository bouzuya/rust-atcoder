use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    };
    let mut ss = s.split('#').collect::<VecDeque<&str>>();
    let min_x = ss.pop_front().unwrap().len();
    let min_y = ss.pop_back().unwrap().len();
    let max = ss.iter().fold(min_x.max(min_y), |acc, x| {
        acc.max(x.len().saturating_sub(min_x.min(min_y)))
    });
    let (x, y) = if min_x > min_y {
        (max, min_y)
    } else {
        (min_x, max)
    };
    println!("{} {}", x, y);
}
