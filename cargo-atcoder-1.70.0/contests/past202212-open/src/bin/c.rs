use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut set = HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                set.insert(a[i] * a[j] * a[k]);
            }
        }
    }
    let ans = set.len();
    println!("{}", ans);
}
