use std::collections::BTreeMap;

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
    let mut it = map.iter().rev();
    for _ in 0..n {
        let ans = *it.next().map(|(_, c)| c).unwrap_or(&0);
        println!("{}", ans);
    }
}
