use proconio::input;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
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
        a: [i64; n],
    };
    let inf = 1_i64 << 60;
    let mut dp = vec![-inf; m + 1];
    for i in 0..n {
        let mut next = dp.clone();
        for j in 2..=m.min(i + 1) {
            chmax!(next[j], dp[j - 1] + j as i64 * a[i]);
        }
        chmax!(next[1], a[i]);
        dp = next;
    }
    let ans = dp[m];
    println!("{}", ans);
}
