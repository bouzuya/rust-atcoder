use std::iter;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();
    let mut pos = vec![vec![]; 26];
    for (i, c) in s.into_iter().enumerate() {
        let index = (c as u8 - b'a') as usize;
        pos[index].push(i);
    }
    let mut count = vec![n; 26];
    for (i, p) in pos.into_iter().enumerate() {
        if p.is_empty() {
            continue;
        }
        let mut max = 0_usize;
        let mut prev = 0_usize;
        for p_j in p.into_iter().chain(iter::once(n)) {
            let p_j = p_j + 1;
            max = max.max(p_j - prev - 1);
            prev = p_j;
        }
        count[i] = max;
    }
    let ans = count.into_iter().min().unwrap();
    println!("{}", ans);
}
