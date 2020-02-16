use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        sv: [String; n]
    };

    let mut map = HashMap::new();
    for s in sv {
        *map.entry(s).or_insert(0) += 1;
    }
    let max = *map.values().max().unwrap();
    let mut maxs = Vec::new();
    for (s, count) in map {
        if count == max {
            maxs.push(s);
        }
    }
    maxs.sort();
    for s in maxs {
        println!("{}", s);
    }
}
