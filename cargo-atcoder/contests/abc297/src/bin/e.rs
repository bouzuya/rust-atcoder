use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut set = a.iter().copied().collect::<BTreeSet<_>>();
    for _ in 0..k - 1 {
        let v = *set.iter().next().unwrap();
        set.remove(&v);

        for a_i in a.iter().copied() {
            set.insert(v + a_i);
        }
        let l = set.len();
        if l > k {
            let mut vs = vec![];
            for v in set.iter().copied().rev().take(l - k) {
                vs.push(v);
            }
            for v in vs {
                set.remove(&v);
            }
        }
    }

    let ans = *set.iter().next().unwrap();
    println!("{}", ans);
}
