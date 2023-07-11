use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeMap;

fn main() {
    input! {
        w: Chars,
    };
    let mut map = BTreeMap::new();
    for c in w {
        *map.entry(c).or_insert(0) += 1;
    }
    let ans = map.iter().all(|(_, &v)| v % 2 == 0);
    println!("{}", if ans { "Yes" } else { "No" });
}
