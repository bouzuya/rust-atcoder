use std::collections::BTreeSet;

use proconio::input;

fn f(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn main() {
    input! {
        s: usize,
    };
    let mut n = s;
    let mut set = BTreeSet::new();
    loop {
        if !set.insert(n) {
            let ans = set.len() + 1;
            println!("{}", ans);
            break;
        }
        n = f(n);
    }
}
