use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let mut count = vec![vec![0_usize; n + 1]; 10];
    for (i, s_i) in s.iter().copied().enumerate() {
        let d = (s_i as u8 - b'0') as usize;
        count[d][i + 1] += 1;
        for j in 0..10 {
            count[j][i + 1] += count[j][i];
        }
    }

    let mut map = HashMap::new();
    for i in 0..=n {
        let mut col = vec![];
        for j in 0..10 {
            col.push(count[j][i] % 2);
        }
        *map.entry(col).or_insert(0) += 1;
    }

    let mut ans = 0_usize;
    for (_, v) in map {
        if v <= 1 {
            continue;
        }
        ans += v * (v - 1) / 2;
    }

    println!("{}", ans);
}
