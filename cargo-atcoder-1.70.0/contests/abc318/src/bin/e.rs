use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map = HashMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        map.entry(a_i).or_insert_with(Vec::new).push(i);
    }

    let mut count = 0_usize;
    for (_, b) in map.iter_mut() {
        let m = b.len();
        if m < 2 {
            continue;
        }
        for i in 0..m - 1 {
            if b[i + 1] - b[i] == 1 {
                continue;
            }
            count += (b[i + 1] - b[i] - 1) * (i + 1) * (m - (i + 1));
        }
    }

    let ans = count;
    println!("{}", ans);
}
