use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        sc: [(usize, usize); n],
    };

    let mut map = BTreeMap::new();
    for (s, c) in sc {
        map.insert(s, c);
    }

    let mut ans = 0_usize;
    while let Some((s, c)) = map.pop_first() {
        if c == 0 {
            continue;
        }
        ans += c % 2;
        map.remove(&s);
        *map.entry(s * 2).or_insert(0) += c / 2;
    }

    println!("{}", ans);
}
