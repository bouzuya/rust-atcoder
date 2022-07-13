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
        map_r.entry(h_i).or_insert_with(HashSet::new).insert(w_i);
        map_c.entry(w_i).or_insert_with(HashSet::new).insert(h_i);
    }

    let max_c = map_c.iter().map(|(_, s)| s.len()).max().unwrap();
    let mut remove_keys = vec![];
    for (w_i, s) in map_c.iter() {
        if s.len() != max_c {
            remove_keys.push(*w_i);
            continue;
        }
    }
    for k in remove_keys {
        map_c.remove(&k);
    }

    let max_r = map_r.iter().map(|(_, s)| s.len()).max().unwrap();
    for (h_i, set_w) in map_r.iter() {
        if set_w.len() != max_r {
            continue;
        }

        for (w_i, _) in map_c.iter() {
            if !set.contains(&(*h_i, *w_i)) {
                println!("{}", max_c + max_r);
                return;
            }
        }
    }

    let ans = max_c + max_r - 1;
    println!("{}", ans);
}
