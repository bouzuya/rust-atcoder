use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut pal = 0_usize;
    let mut set = HashSet::new();
    for s_i in s.iter() {
        let mut s_j = s_i.clone();
        s_j.reverse();

        if set.contains(s_i) || set.contains(&s_j) {
            continue;
        }

        if s_i == &s_j {
            pal += 1;
        }

        set.insert(s_i.clone());
        set.insert(s_j);
    }

    let ans = (set.len() + pal) / 2;
    println!("{}", ans);
}
