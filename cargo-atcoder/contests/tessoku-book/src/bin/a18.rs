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
    for a_i in a {
        let mut next = dp.clone();
        for v in dp {
            if v + a_i <= s {
                next.insert(v + a_i);
            }
        }
        dp = next;
    }
    let ans = dp.contains(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}
