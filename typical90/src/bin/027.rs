use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let mut output = vec![];
    let mut set = BTreeSet::new();
    for (i, s_i) in s.iter().enumerate() {
        if set.insert(s_i) {
            output.push(format!("{}", i + 1));
        }
    }
    let ans = output.join("\n");
    println!("{}", ans);
}
