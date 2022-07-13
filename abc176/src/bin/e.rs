use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _h: usize,
        _w: usize,
        m: usize,
        hw: [(Usize1, Usize1); m],
    };

    let mut set = HashSet::new();
    let mut map_r = HashMap::new();
    let mut map_c = HashMap::new();
    for (h_i, w_i) in hw {
        set.insert((h_i, w_i));
        map_r.entry(h_i).or_insert_with(Vec::new).push(w_i);
        map_c.entry(w_i).or_insert_with(Vec::new).push(h_i);
    }

    let max_r = map_r.iter().map(|(_, v)| v.len()).max().unwrap();
    let map_r = map_r
        .into_iter()
        .filter(|(_, v)| v.len() == max_r)
        .map(|(k, _)| k)
        .collect::<Vec<usize>>();
    let max_c = map_c.iter().map(|(_, v)| v.len()).max().unwrap();
    let map_c = map_c
        .into_iter()
        .filter(|(_, v)| v.len() == max_c)
        .map(|(k, _)| k)
        .collect::<Vec<usize>>();

    for h_i in map_r {
        for w_i in map_c.iter().copied() {
            if !set.contains(&(h_i, w_i)) {
                let ans = max_r + max_c;
                println!("{}", ans);
                return;
            }
        }
    }

    let ans = max_r + max_c - 1;
    println!("{}", ans);
}
