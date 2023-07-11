// 解説 AC <https://www.slideshare.net/hama_du/wupc2012>
use proconio::input;
use std::{cmp, collections::HashSet};

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    };
    let max_x = 1_000;
    let max_y = 1_000;
    let mut c = vec![vec![0; max_y + 1 + 1]; max_x + 1 + 1];
    let mut s = HashSet::new();
    for &(x_i, y_i) in xy.iter() {
        c[y_i + 1][x_i + 1] += 1;
        s.insert((y_i, x_i));
    }
    for i in 0..=max_y + 1 {
        for j in 0..=max_x {
            c[i][j + 1] += c[i][j];
        }
    }
    for j in 0..=max_x + 1 {
        for i in 0..=max_y {
            c[i + 1][j] += c[i][j];
        }
    }

    let mut max_a = 0;
    for &(x_i, y_i) in xy.iter() {
        for &(x_j, y_j) in xy.iter() {
            if (x_i >= x_j) || (y_i >= y_j) {
                continue;
            }
            if !s.contains(&(y_j, x_i)) || !s.contains(&(y_i, x_j)) {
                continue;
            }
            let p = c[y_j][x_j] + c[y_i + 1][x_i + 1] - c[y_j][x_i + 1] - c[y_i + 1][x_j];
            if p == 0 {
                max_a = cmp::max(max_a, (y_j - y_i) * (x_j - x_i));
            }
        }
    }
    let ans = max_a;
    println!("{}", ans);
}
