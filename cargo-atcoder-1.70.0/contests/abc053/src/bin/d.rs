use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let count = a.iter().copied().collect::<HashSet<_>>().len();
    let ans = if count % 2 == 0 { count - 1 } else { count };
    println!("{}", ans);
}
