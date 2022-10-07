use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    };

    let mut ab = vec![];
    for a_i in a {
        for b_i in b.iter().copied() {
            if a_i + b_i <= k {
                ab.push(a_i + b_i);
            }
        }
    }

    let mut cd = HashSet::new();
    for c_i in c {
        for d_i in d.iter().copied() {
            if c_i + d_i <= k {
                cd.insert(c_i + d_i);
            }
        }
    }

    for x in ab {
        if cd.contains(&(k - x)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
