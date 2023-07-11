use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let mut map = BTreeMap::new();
    for s_i in s {
        *map.entry(s_i).or_insert(0) += 1;
    }
    let mut ans = "".to_string();
    let mut max_v = 0;
    for (k, v) in map {
        if v > max_v {
            ans = k;
            max_v = v;
        }
    }
    println!("{}", ans);
}
