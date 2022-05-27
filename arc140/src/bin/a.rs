use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let mut ans = n;
    for x in 1..=n {
        if n % x != 0 {
            continue;
        }
        let mut sum = 0_usize;
        for o in 0..x {
            let mut map = HashMap::new();
            for t in 0..n / x {
                *map.entry(s[t * x + o]).or_insert(0_usize) += 1;
            }
            let v = *map.values().max().unwrap();
            sum += n / x - v;
        }
        if sum <= k {
            ans = ans.min(x);
        }
    }
    println!("{}", ans);
}
