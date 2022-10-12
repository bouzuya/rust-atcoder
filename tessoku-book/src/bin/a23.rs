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
        m: usize,
        a: [[usize; n]; m],
    };
    let inf = 1_usize << 60;
    let mut dp = vec![inf; 1 << n];
    dp[0] = 0;
    for a_i in a {
        let mut bits = 0_usize;
        for (j, a_ij) in a_i.iter().copied().enumerate() {
            if a_ij == 1 {
                bits |= 1 << (n - 1 - j);
            }
        }
        for j in 0..1 << n {
            chmin!(dp[j | bits], dp[j] + 1);
        }
    }
    let ans = if dp[(1 << n) - 1] == inf {
        -1
    } else {
        dp[(1 << n) - 1] as i64
    };
    println!("{}", ans);
}
