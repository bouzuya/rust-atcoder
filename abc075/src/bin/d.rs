use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n],
    };
    let map = xy
        .iter()
        .copied()
        .map(|(x, _)| x)
        .chain(xy.iter().copied().map(|(_, y)| y))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });
    let m = map.len();
    let mut rev = vec![0_i64; m];
    for (&k, &i) in map.iter() {
        rev[i] = k;
    }
    let mut count = vec![vec![0; m + 1]; m + 1];
    for (x, y) in xy.iter().copied() {
        count[map[&y] + 1][map[&x] + 1] += 1;
    }
    let mut sum = vec![vec![0; m + 1]; m + 1];
    sum[0][0] = 0;
    for i in 1..=m {
        for j in 1..=m {
            sum[i][j] = count[i][j] + sum[i][j - 1] + sum[i - 1][j] - sum[i - 1][j - 1];
        }
    }

    let mut min = i64::max_value();
    for t in 0..m {
        for b in t + 1..m {
            for l in 0..m {
                for r in l + 1..m {
                    if sum[b + 1][r + 1] + sum[t][l] - sum[b + 1][l] - sum[t][r + 1] < k {
                        continue;
                    }

                    let (t, b, l, r) = (rev[t], rev[b], rev[l], rev[r]);
                    min = min.min((t - b).abs() * (r - l).abs());
                }
            }
        }
    }

    let ans = min;
    println!("{}", ans);
}
