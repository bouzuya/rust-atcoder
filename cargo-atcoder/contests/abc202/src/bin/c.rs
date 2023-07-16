use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
        c: [Usize1; n],
    };

    let mut js = vec![vec![]; n];
    for (i, b_i) in b.iter().copied().enumerate() {
        js[b_i].push(i);
    }

    let f = |s: &[usize]| -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        for s_i in s.iter().copied() {
            *map.entry(s_i).or_insert(0) += 1;
        }
        map
    };
    let map_a = f(&a);
    let map_c = f(&c);

    let mut ans = 0_usize;
    for (a_i, c_a) in map_a {
        let mut count = 0_usize;
        for j in js[a_i].iter().copied() {
            count += map_c.get(&j).unwrap_or(&0);
        }
        ans += c_a * count
    }
    println!("{}", ans);
}
