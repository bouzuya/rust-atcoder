use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let len = s.into_iter().collect::<HashSet<char>>().len();
    let ans = match len {
        1 => 1,
        2 => 3,
        3 => 6,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
