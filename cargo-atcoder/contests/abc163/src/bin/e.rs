use proconio::input;

macro_rules! chmax {
    ($l: expr, $r: expr) => {
        $l = std::cmp::max($l, $r);
    };
}

fn main() {
    input! {
        n: usize,
        av: [i64; n],
    };

    let mut pv: Vec<(i64, i64)> = av.iter().enumerate().map(|(i, &a)| (a, i as i64)).collect();
    pv.sort_by_key(|(a, i)| (-a, -i));
    // dp[i][l] ... 降順で i 番目まで見て l 個を左側に配置したときの最大値
    let mut dp = vec![vec![0_i64; n + 1]; n + 1];
    for i in 0..n {
        let (a, i_a) = pv[i];
        for l in 0..i + 1 {
            let r = n - (i - l) - 1; // (i - l) 個が既に右側に配置されている
            chmax!(dp[i + 1][l + 1], dp[i][l] + a * (i_a - l as i64).abs());
            chmax!(dp[i + 1][l], dp[i][l] + a * (i_a - r as i64).abs());
        }
    }

    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
