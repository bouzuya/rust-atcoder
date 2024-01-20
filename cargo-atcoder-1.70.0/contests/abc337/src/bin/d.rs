use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn count(line: &[char], k: usize, w: usize) -> Option<usize> {
    if k > w {
        return None;
    }
    let inf = 1_usize << 60;
    let mut min = inf;
    let mut count_d = 0_usize;
    let mut deque = VecDeque::new();
    for j in 0..w {
        match line[j] {
            'x' => {
                count_d = 0;
                deque.clear();
                continue;
            }
            '.' => {
                count_d += 1;
                deque.push_back(line[j]);
            }
            'o' => {
                count_d += 0;
                deque.push_back(line[j]);
            }
            _ => unreachable!(),
        }
        if deque.len() > k {
            if let Some(u) = deque.pop_front() {
                match u {
                    '.' => count_d -= 1,
                    'o' => {}
                    _ => unreachable!(),
                }
            }
        }
        if deque.len() == k {
            min = min.min(count_d);
        }
    }
    if min == inf {
        None
    } else {
        Some(min)
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };

    let inf = 1_usize << 60;
    let mut min = inf;

    for i in 0..h {
        if let Some(c) = count(&s[i], k, w) {
            min = min.min(c);
        }
    }
    for j in 0..w {
        let mut line = vec![];
        for i in 0..h {
            line.push(s[i][j]);
        }
        if let Some(c) = count(&line, k, h) {
            min = min.min(c);
        }
    }

    let ans = if min == inf { -1 } else { min as i64 };
    println!("{}", ans);
}
