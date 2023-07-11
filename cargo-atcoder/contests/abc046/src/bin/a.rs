use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        abc: [usize; 3]
    };
    let ans = abc.into_iter().collect::<HashSet<_>>().len();
    println!("{}", ans);
}
