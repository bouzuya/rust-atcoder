// use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n - 1],
//         b: [usize; n - 2],
//     };
//     let inf = 1 << 60;
//     let mut dp = vec![inf; n];
//     dp[0] = 0;
//     for i in 1..n {
//         dp[i] = (dp[i - 1] + a[i - 1]).min(if i > 1 { dp[i - 2] + b[i - 2] } else { inf })
//     }
//     let ans = dp[n - 1];
//     println!("{}", ans);
// }

// B22
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
        a: [usize; n - 1],
        b: [usize; n - 2],
    };
    let inf = 1 << 60;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..n {
        if i + 1 < n {
            chmin!(dp[i + 1], dp[i] + a[i]);
        }
        if i + 2 < n {
            chmin!(dp[i + 2], dp[i] + b[i]);
        }
    }
    let ans = dp[n - 1];
    println!("{}", ans);
}
