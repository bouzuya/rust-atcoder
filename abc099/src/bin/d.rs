use std::cmp;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        large_n: usize,
        large_c: usize,
        d: [[u64; large_c]; large_c],
        c: [[Usize1; large_n]; large_n],
    };

    let mut count = vec![vec![0; large_c]; 3];
    for i in 0..large_n {
        for j in 0..large_n {
            count[(i + j) % 3][c[i][j]] += 1;
        }
    }

    let mut min_v = 1_000_000_000;
    for c1 in 0..large_c {
        for c2 in 0..large_c {
            for c3 in 0..large_c {
                if c1 == c2 || c1 == c3 || c2 == c3 {
                    continue;
                }
                let colors = vec![c1, c2, c3];
                let mut v = 0;
                let cs = colors.iter().take(3).cloned().collect::<Vec<usize>>();
                for (i, &c_i) in cs.iter().enumerate() {
                    for p in 0..large_c {
                        v += count[i][p] * d[p][c_i];
                    }
                }
                min_v = cmp::min(min_v, v);
            }
        }
    }
    let ans = min_v;
    println!("{}", ans);
}
