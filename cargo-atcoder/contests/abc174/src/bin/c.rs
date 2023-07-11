use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    let mut set = HashSet::new();
    let mut x = 0;
    for ans in 1.. {
        x *= 10;
        x += 7;
        x %= k;
        if x == 0 {
            println!("{}", ans);
            return;
        }
        if !set.insert(x) {
            println!("-1");
            return;
        }
    }
}
