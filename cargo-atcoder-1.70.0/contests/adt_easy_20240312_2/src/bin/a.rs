use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        abcde: [usize; 5],
    };
    let ans = abcde.into_iter().collect::<HashSet<usize>>().len();
    println!("{}", ans);
}
