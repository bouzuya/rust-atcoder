use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        tab: [(usize, usize, usize); q],
    };
    let mut map = BTreeMap::new();
    for (t, a, b) in tab {
        match t {
            1 => {
                map.entry(a).or_insert_with(BTreeSet::new).insert(b);
            }
            2 => {
                if let Some(set) = map.get_mut(&a) {
                    set.remove(&b);
                }
            }
            3 => {
                let ans = match (map.get(&a), map.get(&b)) {
                    (None, None) | (None, Some(_)) | (Some(_), None) => false,
                    (Some(sa), Some(sb)) => sa.contains(&b) && sb.contains(&a),
                };
                println!("{}", if ans { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
