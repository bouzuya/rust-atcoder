use std::collections::HashMap;

use proconio::input;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        cy: [(usize, usize); m],
    };
    let mut map = HashMap::new();
    for (c_i, y_i) in cy {
        map.entry(c_i).or_insert(y_i);
    }
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for (i, x_i) in x.iter().copied().enumerate() {
        for j in 0..i + 1 {
            chmax!(
                dp[i + 1][j + 1],
                dp[i][j] + x_i + *map.get(&(j + 1)).unwrap_or(&0)
            );
            chmax!(dp[i + 1][0], dp[i][j]);
        }
    }
    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
