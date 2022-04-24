use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    };
    let t = s
        .into_iter()
        .map(|s_i| s_i.into_iter().collect::<BTreeSet<_>>())
        .collect::<Vec<_>>();
    let mut ans = 0_usize;
    for bits in 0..1 << n {
        let mut count = vec![0; 26];
        for (_, t_i) in t.iter().enumerate().filter(|(i, _)| ((bits >> i) & 1) == 1) {
            for c in t_i.iter().copied() {
                count[(c as u8 - b'a') as usize] += 1;
            }
        }
        ans = ans.max(count.into_iter().filter(|&c| c == k).count());
    }
    println!("{}", ans);
}
