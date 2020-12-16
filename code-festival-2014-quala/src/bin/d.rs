use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        a: usize,
        k: usize,
    };
    if k == 10 || a.to_string().chars().collect::<BTreeSet<char>>().len() <= k {
        println!("0");
        return;
    }
    todo!();
    let ans = n - a.len();

    println!("{}", ans);
}
