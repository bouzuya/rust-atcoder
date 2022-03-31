use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        a: Usize1,
        b: Usize1,
        s: [Chars; 3]
    };
    let mut dirs = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            if s[(1 + i) as usize][(1 + j) as usize] == '#' {
                dirs.push((i, j));
            }
        }
    }
    let mut used = vec![vec![false; 9]; 9];
    let mut deque = VecDeque::new();
    deque.push_back((a, b));
    while let Some((i, j)) = deque.pop_front() {
        if used[i][j] {
            continue;
        }
        used[i][j] = true;
        for (di, dj) in dirs.iter().copied() {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if (0..9).contains(&ni) && (0..9).contains(&nj) {
                let (ni, nj) = (ni as usize, nj as usize);
                if !used[ni][nj] {
                    deque.push_back((ni, nj));
                }
            }
        }
    }
    let mut count = 0;
    for i in 0..9 {
        for j in 0..9 {
            if used[i][j] {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
