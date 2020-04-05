use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        k: usize
    };

    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 1..=9 {
        q.push_back(i);
    }
    for _ in 1..=k - 1 {
        match q.pop_front() {
            Some(n) => {
                if n % 10 != 0 {
                    q.push_back(10 * n + n % 10 - 1);
                }
                q.push_back(10 * n + n % 10 + 0);
                if n % 10 != 9 {
                    q.push_back(10 * n + n % 10 + 1);
                }
            }
            None => unreachable!(),
        }
    }
    let ans = q.pop_front().unwrap();
    println!("{}", ans);
}
