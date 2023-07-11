use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };
    let mut s_ = vec![vec![]; w];
    for j in 0..w {
        for i in 0..h {
            s_[j].push(s[i][j]);
        }
    }
    let mut t_ = vec![vec![]; w];
    for j in 0..w {
        for i in 0..h {
            t_[j].push(t[i][j]);
        }
    }

    let mut map_s = HashMap::new();
    for s_i in s_ {
        *map_s.entry(s_i).or_insert(0) += 1_usize;
    }

    let mut map_t = HashMap::new();
    for t_i in t_ {
        *map_t.entry(t_i).or_insert(0) += 1_usize;
    }

    let ans = map_s == map_t;
    println!("{}", if ans { "Yes" } else { "No" });
}
