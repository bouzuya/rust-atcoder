use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    };
    let mut dp = HashSet::new();
    dp.insert(0);
    for a_i in a.iter().copied() {
        let mut next = dp.clone();
        for d in dp {
            if d + a_i <= 10_000 {
                next.insert(d + a_i);
            }
        }
        dp = next;
    }
    let ans = dp.contains(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}
