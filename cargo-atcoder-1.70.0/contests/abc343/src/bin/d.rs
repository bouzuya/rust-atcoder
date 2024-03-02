use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(Usize1, usize); t],
    };
    let mut cur = vec![0; n];
    let mut map = HashMap::new();
    map.insert(0, n);
    for (i, (a, b)) in ab.into_iter().enumerate() {
        let before = cur[a];
        let after = before + b;

        let entry = map.get_mut(&before).unwrap();
        *entry -= 1;
        if *entry == 0 {
            map.remove(&before);
        }

        *map.entry(after).or_insert(0) += 1;
        cur[a] = after;

        let ans = map.len();
        println!("{}", ans);
    }
}
