use std::collections::{BTreeSet, VecDeque};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut t = VecDeque::new();
    let mut set = BTreeSet::new();
    for &c in s.iter().rev() {
        if set.insert(c) {
            t.push_front(c);
        }
    }
    let ans = t.iter().collect::<String>();
    println!("{}", ans);
}
