use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut a = a.into_iter().collect::<VecDeque<_>>();
    for _ in 0..k {
        a.pop_front();
        a.push_back(0);
    }
    for x in a {
        println!("{}", x);
    }
}
