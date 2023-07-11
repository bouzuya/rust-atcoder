use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut t = vec![];
    let mut b = 1_usize;
    for s_i in s.iter().copied().rev() {
        let d = (s_i as u8 - b'0') as usize;
        t.push(d * b);
        b *= 10;
        b %= 2019;
    }
    t.reverse();

    let cs = std::iter::once(0)
        .chain(t.iter().scan(0, |acc, &i| {
            *acc += i;
            *acc %= 2019;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    let mut map = HashMap::new();
    for cs_i in cs {
        *map.entry(cs_i).or_insert(0) += 1;
    }

    let mut ans = 0;
    for &v in map.values() {
        ans += v * (v - 1) / 2;
    }
    println!("{}", ans);
}
