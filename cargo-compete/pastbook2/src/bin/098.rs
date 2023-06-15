use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }
    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; n]; 1 << n];
    dp[1 << 0][0] = 0_usize;
    let mut deque = VecDeque::new();
    deque.push_back((1 << 0, 0_usize));
    while let Some((set, u)) = deque.pop_front() {
        for v in 0..n {
            if (set & (1 << v)) != 0 {
                continue;
            }
            let new_set = set | (1 << v);
            let new_cost = dp[set][u] + a[u][v];
            if new_cost < dp[new_set][v] {
                dp[new_set][v] = new_cost;
                deque.push_back((new_set, v));
            }
        }
    }
    let mut min = inf;
    for u in 1..n {
        min = min.min(dp[(1 << n) - 1][u] + a[u][0]);
    }
    let ans = min;
    println!("{}", ans);
}
