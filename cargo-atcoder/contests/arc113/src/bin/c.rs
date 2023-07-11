use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    s.reverse();

    let mut count = 0_usize;
    let mut map = BTreeMap::new();
    let mut p = 0;
    let mut s_2 = ' ';
    let mut s_1 = ' ';
    for (i, &s_i) in s.iter().enumerate() {
        if i - p >= 2 && s_2 != s_1 && s_1 == s_i {
            count += i - 1 - (map.get(&s_i).unwrap() - 1);
            map.clear();
            map.entry(s_i).or_insert(i + 1);
            s_2 = ' ';
            s_1 = ' ';
            p = i;
        } else {
            *map.entry(s_i).or_insert(0) += 1_usize;
            s_2 = s_1;
            s_1 = s_i;
        }
    }

    let ans = count;
    println!("{}", ans);
}
