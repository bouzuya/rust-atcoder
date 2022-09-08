use std::collections::BTreeSet;

use proconio::input;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
    };
    let mut set = BTreeSet::new();
    let mut x = 6;
    while x <= n {
        set.insert(x);
        x *= 6;
    }
    let mut x = 9;
    while x <= n {
        set.insert(x);
        x *= 9;
    }
    set.insert(1);
    let coins = set.into_iter().collect::<Vec<usize>>();

    let mut dp = vec![n; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        for c in coins.iter().copied() {
            if i < c {
                continue;
            }
            chmin!(dp[i], dp[i - c] + 1);
        }
    }
    let ans = dp[n];
    println!("{}", ans);
}
