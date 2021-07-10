use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
    };
    let set_a = a.into_iter().collect::<BTreeSet<i64>>();
    let set_b = b.into_iter().collect::<BTreeSet<i64>>();
    let duplicated = set_a
        .intersection(&set_b)
        .copied()
        .collect::<BTreeSet<i64>>();
    for x in set_a.union(&set_b).copied() {
        if duplicated.contains(&x) {
            continue;
        }
        println!("{}", x);
    }
}
