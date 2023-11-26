use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        s: [Chars; n]
    };

    let mut ans = 0_usize;
    for bits in 0..1 << l {
        let wildcard = {
            let selected = (0..l)
                .filter(|i| ((bits >> i) & 1) == 1)
                .collect::<Vec<usize>>();
            if selected.len() != k {
                continue;
            }
            let mut wildcard = vec![false; l];
            for i in selected {
                wildcard[i] = true;
            }
            wildcard
        };

        let mut count = HashMap::new();
        for s_i in s.iter() {
            let x = s_i
                .iter()
                .copied()
                .enumerate()
                .map(|(j, s_ij)| if wildcard[j] { '?' } else { s_ij })
                .collect::<Vec<char>>();
            *count.entry(x).or_insert(0) += 1;
        }
        ans = ans.max(*count.values().max().unwrap());
    }

    println!("{}", ans);
}
