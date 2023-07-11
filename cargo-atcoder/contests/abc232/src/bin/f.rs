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
        x: usize,
        y: usize,
        a: [usize; n],
        b: [usize; n],
    };
    let inf = 1_000_000_000_000_000_000;
    let mut dp = vec![inf; 1 << n];
    dp[0] = 0;
    for s in 0_usize..1 << n {
        let j = s.count_ones() as usize;
        for i in 0..n {
            if (s & (1 << i)) != 0 {
                continue;
            }
            chmin!(
                dp[s | 1 << i],
                dp[s]
                    + (if a[i] > b[j] {
                        a[i] - b[j]
                    } else {
                        b[j] - a[i]
                    }) * x
                    + (s >> i).count_ones() as usize * y
            );
        }
    }
    let ans = dp[(1 << n) - 1];
    println!("{}", ans);
}
