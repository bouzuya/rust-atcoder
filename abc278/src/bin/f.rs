// WIP
use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let edges = {
        let mut map = HashMap::new();
        for (i, s_i) in s.iter().enumerate() {
            map.entry(s_i[0]).or_insert_with(Vec::new).push(i);
        }
        let mut edges = vec![];
        for s_i in s.iter() {
            edges.push(
                map.get(&s_i.last().unwrap())
                    .cloned()
                    .unwrap_or_else(Vec::new),
            );
        }
        edges
    };
    for start in 0..n {
        //
    }
    // let ans = n - a.len();
    // println!("{}", ans);
}
