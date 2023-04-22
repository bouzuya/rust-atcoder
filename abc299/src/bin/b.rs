use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        t: Usize1,
        c: [Usize1; n],
        r: [usize; n],
    };

    let c1 = c[0];
    let mut max = HashMap::new();
    for (i, (c_i, r_i)) in c.into_iter().zip(r.into_iter()).enumerate() {
        let entry = max.entry(c_i).or_insert((r_i, i));
        if entry.0 < r_i {
            *entry = (r_i, i);
        }
    }

    let ans = if let Some((_, i)) = max.get(&t) {
        *i
    } else {
        max.get(&c1).unwrap().1
    } + 1;
    println!("{}", ans);
}
