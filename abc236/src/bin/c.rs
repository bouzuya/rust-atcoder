use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };
    let set = t.into_iter().collect::<HashSet<_>>();
    for s_i in s {
        let ans = set.contains(&s_i);
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
