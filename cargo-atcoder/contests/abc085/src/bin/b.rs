use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    };
    let ans = d.into_iter().collect::<HashSet<_>>().len();
    println!("{}", ans);
}
