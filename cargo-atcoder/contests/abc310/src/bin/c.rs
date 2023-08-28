use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut count = 0_usize;
    let mut set = HashSet::new();
    for s_i in s {
        let s_j = s_i.iter().copied().rev().collect::<Vec<char>>();
        if !set.contains(&s_i) && !set.contains(&s_j) {
            count += 1;
        }
        set.insert(s_i);
        set.insert(s_j);
    }
    let ans = count;
    println!("{}", ans);
}
