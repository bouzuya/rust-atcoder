use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        av: [usize; n]
    };
    let set: HashSet<usize> = av.into_iter().collect();
    let ans = if set.len() == n { "YES" } else { "NO" };
    println!("{}", ans);
}
