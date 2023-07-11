use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut count = 0;
    let mut set = BTreeSet::new();
    for a_i in a {
        if !set.insert(a_i) {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
