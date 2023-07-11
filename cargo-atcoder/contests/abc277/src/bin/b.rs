use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let cs1 = "HDCS".chars().collect::<Vec<char>>();
    let cs2 = "A23456789TJQK".chars().collect::<Vec<char>>();
    let mut set = HashSet::new();
    for s_i in s {
        if !cs1.contains(&s_i[0]) || !cs2.contains(&s_i[1]) || !set.insert(s_i) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
