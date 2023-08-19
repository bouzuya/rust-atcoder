use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };

    let inf = 1e18;
    let mut dp = vec![HashMap::new(); 30 + 1];
    let mut prev = VecDeque::new();
    prev.push_back(0);
    dp[0] = {
        let mut map = HashMap::new();
        map.insert(0, 0_f64);
        map
    };
    for (i, (x_i, y_i)) in xy.iter().copied().enumerate().skip(1) {
        let mut next = vec![HashMap::new(); 30 + 1];
        for j in 0..=30 {
            for (&k, &v) in dp[j].iter() {
                // 直前のチェックポイントが k
                let (x_j, y_j) = xy[k];

                // i を省略しない
                let entry = next[j].entry(i).or_insert(inf);
                let nv = v + (((x_j - x_i).pow(2) + (y_j - y_i).pow(2)) as f64).sqrt();
                if nv < *entry {
                    *entry = nv;
                }

                // i を省略する
                if j < 30 {
                    let entry = next[j + 1].entry(k).or_insert(inf);
                    if v < *entry {
                        *entry = v;
                    }
                }
            }
        }
        prev.push_back(i);
        if prev.len() > 30 {
            prev.pop_front();
        }
        dp = next;
    }

    let mut min = inf;
    for j in 0..=30 {
        let p = if j == 0 {
            0_f64
        } else {
            2_f64.powf((j - 1) as f64)
        };
        for (&k, &v) in dp[j].iter() {
            if k == n - 1 && v + p < min {
                min = v + p;
            }
        }
    }
    let ans = min;
    println!("{}", ans);
}
