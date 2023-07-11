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
        w: usize,
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    };
    let mut dp = vec![vec![0_usize; k + 1]; w + 1];
    for (a, b) in ab.iter().copied() {
        let mut next = vec![vec![0_usize; k + 1]; w + 1];
        for j in 0..=k {
            for l in 0..=w {
                chmax!(next[l][j], dp[l][j]);
                if j + 1 <= k && l + a <= w {
                    chmax!(next[l + a][j + 1], dp[l][j] + b);
                }
            }
        }
        dp = next;
    }
    let ans = dp
        .iter()
        .map(|dp_i| dp_i.iter().max().unwrap())
        .max()
        .unwrap();
    println!("{}", ans);
}
