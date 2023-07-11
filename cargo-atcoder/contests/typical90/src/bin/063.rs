use std::cmp;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[Usize1; w]; h],
    };
    let mut max = 0;
    for bits in 0..1 << h {
        let is: Vec<usize> = (0..h).filter(|i| ((bits >> i) & 1) == 1).collect();
        if is.is_empty() {
            continue;
        }
        let mut count = vec![0; h * w];
        for j in 0..w {
            let mut v = Some(p[is[0]][j]);
            for &i in is.iter() {
                if v != Some(p[i][j]) {
                    v = None;
                }
            }
            if let Some(v) = v {
                count[v] += 1;
            }
        }
        max = cmp::max(max, is.len() * count.iter().max().unwrap());
    }

    let ans = max;
    println!("{}", ans);
}
