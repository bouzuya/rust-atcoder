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
        h: [i64; n],
    };
    // h[i]: 足場 i - 1 の高さ
    // dp[i]: 足場 i - 1 辿り着くまでに支払うコストの総和の最小値
    // 足場 1 は最初から居るのでコスト 0 、それ以外は最小値を求めるため大きな値で初期化する
    let inf = 1_000_000_000_000_i64; // 10^4 * 10^5 が最大。それを超える値。
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..n {
        if i + 1 < n {
            chmin!(dp[i + 1], dp[i] + (h[i + 1] - h[i]).abs());
        }
        if i + 2 < n {
            chmin!(dp[i + 2], dp[i] + (h[i + 2] - h[i]).abs());
        }
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
