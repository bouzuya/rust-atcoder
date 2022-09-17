use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ks = vec![];
    for k in 0..60 {
        if (n & (1 << k)) != 0 {
            ks.push(k);
        }
    }

    let mut ans = BTreeSet::new();
    for bits in 0..1 << ks.len() {
        let mut a = 0_usize;
        for i in 0..ks.len() {
            if (bits >> i) & 1 == 1 {
                a |= 1 << ks[i];
            }
        }
        ans.insert(a);
    }

    for a in ans {
        println!("{}", a);
    }
}
