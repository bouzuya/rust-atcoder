use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = HashMap::new();
    for a_i in a.iter().copied() {
        *map.entry(a_i).or_insert(0) += 1;
    }

    let mut sum = 0_usize;
    for (_, &count) in map.iter() {
        if count <= 1 {
            continue;
        }
        sum += count * (count - 1) / 2;
    }

    for k in a {
        let count = map.get(&k).unwrap();
        let x = count * count.saturating_sub(1) / 2;
        let y = count.saturating_sub(1) * count.saturating_sub(2) / 2;
        let ans = sum - x + y;
        println!("{}", ans);
    }
}
