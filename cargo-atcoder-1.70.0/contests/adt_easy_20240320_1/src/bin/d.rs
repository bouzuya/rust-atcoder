use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        xy: [(i64, i64); n],
    };

    let mut people = xy.iter().copied().collect::<HashSet<(i64, i64)>>();
    let mut light = HashSet::new();
    for a_i in a {
        let xy_i = xy[a_i];
        people.remove(&xy_i);
        light.insert(xy_i);
    }

    let p = people.iter().copied().collect::<Vec<(i64, i64)>>();
    let l = light.iter().copied().collect::<Vec<(i64, i64)>>();
    let mut max = 0_f64;
    for p_i in p.iter().copied() {
        let mut min = 1_000_000_000_000_f64;
        for l_j in l.iter().copied() {
            let d = ((p_i.0 - l_j.0).pow(2) + (p_i.1 - l_j.1).pow(2)) as f64;
            if d < min {
                min = d;
            }
        }
        max = max.max(min);
    }
    let ans = max.sqrt();
    println!("{}", ans);
}
