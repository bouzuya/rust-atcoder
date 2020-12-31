use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut deque = VecDeque::new();
    for i in 0..n {
        let c = s[i];
        if deque.is_empty() {
            deque.push_back(c);
            continue;
        }
        let &l = deque.front().unwrap();
        let &r = deque.back().unwrap();
        if c == l {
            deque.pop_front();
            continue;
        }
        if c == r {
            deque.pop_back();
            continue;
        }
        if i + 1 == n {
            deque.push_back(c);
            continue;
        }
        let n = s[i + 1];
        if c == n {
            deque.push_back(s[i]);
            continue;
        }
        if n == l {
            deque.push_back(c);
            continue;
        }
        if n == r {
            deque.push_front(c);
            continue;
        }
        deque.push_back(s[i]);
    }
    let ans = deque.len();
    println!("{}", ans);
}
