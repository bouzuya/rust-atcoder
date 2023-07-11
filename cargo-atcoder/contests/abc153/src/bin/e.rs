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
        h: usize,
        n: usize,
        ab: [(usize, usize); n],
    };
    let inf = 1_usize << 60;
    let mut dp = vec![inf; h + 1];
    dp[0] = 0_usize;
    for i in 0..h {
        for (a, b) in ab.iter().copied() {
            chmin!(dp[(i + a).min(h)], dp[i] + b);
        }
    }
    let ans = dp[h];
    println!("{}", ans);
}
