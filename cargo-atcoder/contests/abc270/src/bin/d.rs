use proconio::input;

fn f(dp: &mut Vec<usize>, inf: usize, n: usize, a: &[usize], i: usize) {
    if dp[i] != inf {
        return;
    }

    let mut max = 0_usize;
    for a_j in a.iter().copied() {
        if a_j > i {
            break;
        }

        f(dp, inf, n, a, i - a_j);
        max = max.max(i - dp[i - a_j]);
    }

    dp[i] = max;
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    };
    let inf = 1 << 60;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0;
    f(&mut dp, inf, n, &a, n);
    let ans = dp[n];
    println!("{}", ans);
}
