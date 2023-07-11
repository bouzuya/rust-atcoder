use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        s: [String; 4],
    };
    let set = s.into_iter().collect::<BTreeSet<String>>();
    let ans = set.len() == 4;
    println!("{}", if ans { "Yes" } else { "No" });
}
