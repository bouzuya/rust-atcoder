use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn dfs(ans: &mut Vec<usize>, level: usize, s: Vec<(usize, VecDeque<char>)>) {
    if s.is_empty() {
        return;
    }
    if s.len() == 1 {
        return;
    }

    let mut count = vec![0_usize; 26];
    let mut t = vec![vec![]; 26];
    for (i, mut s_i) in s {
        ans[i] = level;
        if let Some(c) = s_i.pop_front() {
            let index = (c as u8 - b'a') as usize;
            t[index].push((i, s_i));
            count[index] += 1;
        }
    }
    for t_i in t {
        if t_i.is_empty() {
            continue;
        }
        dfs(ans, level + 1, t_i);
    }
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let s = s
        .into_iter()
        .enumerate()
        .map(|(i, s_i)| (i, s_i.into_iter().collect::<VecDeque<_>>()))
        .collect::<Vec<(usize, VecDeque<char>)>>();

    let mut ans = vec![0_usize; n];
    dfs(&mut ans, 0, s);

    for a in ans {
        println!("{}", a);
    }
}
