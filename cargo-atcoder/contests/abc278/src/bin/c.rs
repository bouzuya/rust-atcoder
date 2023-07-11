use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        q: usize,
        tab: [(usize, Usize1, Usize1); q],
    };
    let mut follows = HashMap::new();
    for (t, a, b) in tab {
        match t {
            1 => {
                follows.entry(a).or_insert_with(HashSet::new).insert(b);
            }
            2 => {
                if let Some(fa) = follows.get_mut(&a) {
                    fa.remove(&b);
                }
            }
            3 => {
                let ans = match (follows.get(&a), follows.get(&b)) {
                    (None, None) | (None, Some(_)) | (Some(_), None) => false,
                    (Some(fa), Some(fb)) => fa.contains(&b) && fb.contains(&a),
                };
                println!("{}", if ans { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
