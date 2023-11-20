use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn is_match(s: &[char], t: &[char], i: usize) -> bool {
    for j in 0..t.len() {
        if i + j >= s.len() {
            return false;
        }
        if s[i + j] != '.' && s[i + j] != t[j] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: Chars,
        t: Chars,
    };

    let mut queue = VecDeque::new();
    for i in 0..n {
        if is_match(&s, &t, i) {
            s[i..i + m].fill('.');
            queue.push_back(i);
        }
    }

    let mut used = vec![false; n];
    while let Some(prev) = queue.pop_front() {
        for i in prev.saturating_sub(t.len() - 1)..prev + t.len() - 1 {
            if !used[i] && is_match(&s, &t, i) {
                s[i..i + m].fill('.');
                queue.push_back(i);
                used[i] = true;
            }
        }
    }

    let ans = s.iter().all(|c| c == &'.');
    println!("{}", if ans { "Yes" } else { "No" });
}
