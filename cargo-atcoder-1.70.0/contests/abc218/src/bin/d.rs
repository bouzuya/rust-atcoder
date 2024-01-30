use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut ans = HashSet::new();
    let set = xy.iter().collect::<std::collections::HashSet<_>>();
    for i in 0..n {
        let (x_i, y_i) = xy[i];
        for j in i + 1..n {
            let (x_j, y_j) = xy[j];
            if x_i == x_j || y_i == y_j {
                continue;
            }
            if set.contains(&(x_i, y_j)) && set.contains(&(x_j, y_i)) {
                ans.insert((x_i.min(x_j), y_i.min(y_j), x_i.max(x_j), y_i.max(y_j)));
            }
        }
    }
    println!("{}", ans.len());
}
