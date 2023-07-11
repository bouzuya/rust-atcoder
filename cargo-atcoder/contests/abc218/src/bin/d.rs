use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    };

    xy.sort();

    let mut map_x = BTreeMap::new();
    let mut map_y = BTreeMap::new();
    for (x_i, y_i) in xy.iter().copied() {
        map_x.entry(x_i).or_insert(BTreeSet::new()).insert(y_i);
        map_y.entry(y_i).or_insert(BTreeSet::new()).insert(x_i);
    }

    let mut count = 0_usize;
    for i in 0..n {
        let (x_i, y_i) = xy[i];
        for j in i + 1..n {
            let (x_j, y_j) = xy[j];
            if x_i == x_j || y_i == y_j {
                continue;
            }

            if let Some(set_x) = map_x.get_mut(&x_i) {
                if let Some(set_y) = map_y.get_mut(&y_i) {
                    if set_x.contains(&y_j) && set_y.contains(&x_j) {
                        set_x.remove(&y_i);
                        set_y.remove(&x_i);
                        count += 1;
                    }
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
