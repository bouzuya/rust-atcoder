use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    };

    let mut set = HashSet::new();
    for a_i in a.iter().copied() {
        for b_i in b.iter().copied() {
            for c_i in c.iter().copied() {
                set.insert(a_i + b_i + c_i);
            }
        }
    }
    for x_i in x {
        let ans = set.contains(&x_i);
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
