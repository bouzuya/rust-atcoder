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

    let ans = a
        .into_iter()
        .map(|a_i| a_i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
