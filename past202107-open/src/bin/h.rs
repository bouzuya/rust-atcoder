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

fn f(l: usize, r: usize) -> f64 {
    ((1 + (if l > r { l - r } else { r - l }).pow(2)) as f64).sqrt()
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let s = a.iter().sum::<usize>();
    let inf = 1_000_000_000_000_f64;
    let mut dp = vec![vec![inf; 100 + 1]; s + 1];
    dp[0][0] = 0_f64;
    for _ in 1..n {
        let mut next = vec![vec![inf; 100 + 1]; s + 1];
        for sum in 0..=s {
            for last in 0..=100 {
                for j in 0..=100 {
                    if sum >= last {
                        chmin!(next[sum][last], dp[sum - last][j] + f(j, last));
                    }
                }
            }
        }
        dp = next;
    }

    let ans = dp[s][0];
    println!("{}", ans);
}
