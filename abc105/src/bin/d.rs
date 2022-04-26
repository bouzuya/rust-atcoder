use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            *acc %= m;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let mut c = BTreeMap::new();
    for s_i in s {
        *c.entry(s_i).or_insert(0) += 1;
    }
    let mut ans = 0_usize;
    for (_, c_i) in c {
        if c_i < 2 {
            continue;
        }
        ans += c_i * (c_i - 1) / 2;
    }
    println!("{}", ans);
}
