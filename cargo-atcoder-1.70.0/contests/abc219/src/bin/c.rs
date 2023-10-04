use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [String; n],
    };
    let mut map = BTreeMap::new();
    for s_i in s {
        let t_i = s_i
            .chars()
            .map(|c| x.iter().position(|c_j| c_j == &c).unwrap())
            .collect::<Vec<usize>>();
        map.insert(t_i, s_i);
    }

    for (_, v) in map {
        println!("{}", v);
    }
}
