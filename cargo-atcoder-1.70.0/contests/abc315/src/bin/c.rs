use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        fs: [(usize, usize); n],
    };

    let ans = {
        let mut map = HashMap::new();
        for (_, (f, s)) in fs.iter().copied().enumerate() {
            map.entry(f).or_insert_with(Vec::new).push(s);
        }
        let mut max = 0_usize;
        for (_, v) in map.iter_mut() {
            if v.len() <= 1 {
                continue;
            }
            v.sort();
            v.reverse();
            max = max.max(v[0] + v[1] / 2);
        }
        max
    }
    .max({
        let mut map = HashMap::new();
        for (i, (f, s)) in fs.iter().copied().enumerate() {
            let entry = map.entry(f).or_insert(s);
            *entry = (*entry).max(s);
        }
        if map.len() <= 1 {
            0
        } else {
            let mut values = map.values().copied().collect::<Vec<usize>>();
            values.sort();
            values.reverse();
            values[0] + values[1]
        }
    });
    println!("{}", ans);
}
