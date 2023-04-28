use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; 1 << n]; n];
    dp[0][1] = 0_usize;
    let mut deque = VecDeque::new();
    deque.push_back((1, 0));
    while let Some((set, u)) = deque.pop_front() {
        for v in 0..n {
            if (set & (1 << v)) != 0 {
                continue;
            }
            let new_set = set | (1 << v);
            let new_cost = dp[u][set] + a[u][v];
            if new_cost < dp[v][new_set] {
                dp[v][new_set] = new_cost;
                deque.push_back((new_set, v));
            }
        }
    }
    let ans = (0..n).map(|i| dp[i][(1 << n) - 1] + a[i][0]).min().unwrap();
    println!("{}", ans);
}
