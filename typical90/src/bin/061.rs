use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    };
    let mut deque = VecDeque::new();
    for (t_i, x_i) in tx {
        match t_i {
            1 => deque.push_front(x_i),
            2 => deque.push_back(x_i),
            3 => println!("{}", deque[x_i - 1]),
            _ => unreachable!(),
        }
    }
}
