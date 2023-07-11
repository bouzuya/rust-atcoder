use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [char; n],
    };
    let ans = s.into_iter().collect::<BTreeSet<_>>().len() == 4;
    println!("{}", if ans { "Four" } else { "Three" });
}
