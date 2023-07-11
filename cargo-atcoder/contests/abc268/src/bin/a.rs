use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        abcde: [usize; 5],
    };
    let set = abcde.into_iter().collect::<HashSet<_>>();
    let ans = set.len();
    println!("{}", ans);
}
