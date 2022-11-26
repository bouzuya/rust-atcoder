use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };

    let mut a = vec![vec![' '; h]; w];
    for j in 0..w {
        for i in 0..h {
            a[j][i] = s[i][j];
        }
    }
    let mut b = vec![vec![' '; h]; w];
    for j in 0..w {
        for i in 0..h {
            b[j][i] = t[i][j];
        }
    }

    let mut map_a = HashMap::new();
    for a_i in a {
        *map_a.entry(a_i.iter().collect::<String>()).or_insert(0) += 1;
    }
    let mut map_b = HashMap::new();
    for b_i in b {
        *map_b.entry(b_i.iter().collect::<String>()).or_insert(0) += 1;
    }

    let ans = map_a == map_b;
    println!("{}", if ans { "Yes" } else { "No" });
}
