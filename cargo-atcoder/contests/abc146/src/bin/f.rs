use std::cmp;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        m: usize,
        mut s: Chars,
    };
    s.reverse();
    let mut steps = vec![];
    let mut i = 0;
    'outer: loop {
        for j in (i + 1..cmp::min(i + 1 + m, s.len())).rev() {
            if s[j] == '0' {
                steps.push(j - i);
                i = j;
                if j == s.len() - 1 {
                    break 'outer;
                } else {
                    continue 'outer;
                }
            }
        }
        println!("-1");
        return;
    }
    steps.reverse();
    for (i, &s_i) in steps.iter().enumerate() {
        print!("{}{}", s_i, if i == steps.len() - 1 { "\n" } else { " " });
    }
}
