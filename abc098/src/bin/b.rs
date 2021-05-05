use std::{cmp, collections::BTreeSet};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut max_count = 0;
    for i in 1..n - 1 {
        let (x, y) = s.split_at(i);
        let set_x = x.iter().cloned().collect::<BTreeSet<char>>();
        let set_y = y.iter().cloned().collect::<BTreeSet<char>>();
        max_count = cmp::max(max_count, set_x.intersection(&set_y).into_iter().count());
    }
    let ans = max_count;
    println!("{}", ans);
}
