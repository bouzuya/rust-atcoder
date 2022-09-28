use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let mut x = a;
    let mut set = HashSet::new();
    while set.insert(x % b) {
        x += a;
        x %= b;
    }
    let ans = set.contains(&c);
    println!("{}", if ans { "YES" } else { "NO" });
}
