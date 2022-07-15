use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [String; n],
    };
    let ans = a.into_iter().collect::<HashSet<_>>().len();
    println!("{}", ans);
}
