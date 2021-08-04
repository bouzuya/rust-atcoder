use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        mut b: [u64; n],
    };
    b.sort();

    let mut set = BTreeSet::new();
    let a_0 = a[0];
    for b_j in b.clone() {
        let x = a_0 ^ b_j;
        set.insert(x);
    }

    let mut ans = vec![];
    for x in set {
        let mut c = a.iter().copied().map(|a_i| a_i ^ x).collect::<Vec<u64>>();
        c.sort();
        if b == c {
            ans.push(x);
        }
    }

    println!("{}", ans.len());
    for x in ans {
        println!("{}", x);
    }
}
