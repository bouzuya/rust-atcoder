use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut map = BTreeMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1;
    }

    let set = map
        .iter()
        .filter(|&(_, count)| *count >= 2)
        .map(|(a_i, _)| *a_i)
        .collect::<BTreeSet<_>>();
    let mut iter = set.into_iter().rev();
    let ans2 = match (iter.next(), iter.next()) {
        (Some(h), Some(w)) => h * w,
        (_, _) => 0,
    };
    let set = map
        .iter()
        .filter(|&(_, count)| *count >= 4)
        .map(|(a_i, _)| *a_i)
        .collect::<BTreeSet<_>>();
    let ans4 = set
        .into_iter()
        .rev()
        .next()
        .map(|s| s * s)
        .unwrap_or_default();
    let ans = ans2.max(ans4);
    println!("{}", ans);
}
