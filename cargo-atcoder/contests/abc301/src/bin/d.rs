use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        n: usize,
    };
    let mut t = format!("{:0b}", n).chars().collect::<VecDeque<char>>();
    let len = t.len().max(s.len());
    let mut s = s.into_iter().collect::<VecDeque<char>>();
    for _ in 0..len - s.len() {
        s.push_front('0');
    }
    for _ in 0..len - t.len() {
        t.push_front('0');
    }

    let f = |ans: &[char]| -> i64 {
        let mut b = 1_i64;
        let mut v = 0_i64;
        for a in ans.iter().copied().rev() {
            if a == '1' {
                v += b;
            }
            b <<= 1;
        }
        v
    };

    let mut max = -1;
    for i in 0..=len {
        let mut ans = vec![];
        for (j, s_j) in s.iter().copied().enumerate() {
            if j < i {
                match s_j {
                    '0' => ans.push('0'),
                    '1' => ans.push('1'),
                    '?' => ans.push(t[j]),
                    _ => unreachable!(),
                }
            } else if j == i {
                match s_j {
                    '0' => ans.push('0'),
                    '1' => ans.push('1'),
                    '?' => ans.push('0'),
                    _ => unreachable!(),
                }
            } else {
                match s_j {
                    '0' => ans.push('0'),
                    '1' => ans.push('1'),
                    '?' => ans.push('1'),
                    _ => unreachable!(),
                }
            }
        }
        let v = f(&ans);
        if v <= n as i64 {
            max = max.max(v);
        }
    }
    let ans = max;
    println!("{}", ans);
}
