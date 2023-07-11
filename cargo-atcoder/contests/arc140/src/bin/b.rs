use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut rs = vec![];
    for (i, s_i) in s.iter().copied().enumerate() {
        if s_i == 'R' {
            rs.push(i);
        }
    }
    let mut counts = vec![];
    for i in rs {
        let mut count_a = 0_usize;
        for j in 1.. {
            if i < j {
                break;
            }
            if s[i - j] == 'A' {
                count_a += 1;
            } else {
                break;
            }
        }
        let mut count_c = 0_usize;
        for j in 1.. {
            if i + j >= n {
                break;
            }
            if s[i + j] == 'C' {
                count_c += 1;
            } else {
                break;
            }
        }
        let c = count_a.min(count_c);
        if c > 0 {
            counts.push(c);
        }
    }
    let mut deque = VecDeque::new();
    for c in counts {
        if c == 1 {
            deque.push_front(c);
        } else {
            deque.push_back(c);
        }
    }

    let mut count = 0_usize;
    loop {
        if count % 2 == 0 {
            // 奇数回目
            if let Some(c) = deque.pop_back() {
                let c = c - 1;
                if c == 0 {
                    // do nothing
                } else if c == 1 {
                    deque.push_front(c);
                } else {
                    deque.push_back(c);
                }
                count += 1;
            } else {
                break;
            }
        } else {
            // 偶数回目
            if deque.pop_front().is_some() {
                count += 1;
            } else {
                break;
            }
        }
    }

    let ans = count;
    println!("{}", ans);
}
