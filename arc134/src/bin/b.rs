use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut pq = BinaryHeap::new();
    for (i, s_i) in s.iter().copied().enumerate() {
        pq.push((Reverse(s_i), i));
    }

    let mut r = n;
    let mut ans = s.clone();
    for (l, s_l) in s.iter().copied().enumerate() {
        while let Some((Reverse(s_j), j)) = pq.pop() {
            if j < l || r <= j {
                continue;
            }
            if s_j < s_l {
                r = j;
                ans.swap(l, r);
            } else {
                pq.push((Reverse(s_j), j));
            }
            break;
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
