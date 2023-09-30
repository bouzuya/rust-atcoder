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
        k: usize,
        p: usize,
        ca: [(usize, [usize; k]); n],
    };
    let ca = {
        let mut v = vec![(0, vec![p; 5]); n];
        for (i, (c, a)) in ca.iter().enumerate() {
            v[i].0 = *c;
            for (j, a) in a.iter().copied().enumerate() {
                v[i].1[j] = a;
            }
        }
        v
    };

    let mut sum = vec![0_usize; 5];
    for (_, a) in ca.iter() {
        for j in 0..5 {
            sum[j] += a[j];
        }
    }
    if sum.iter().any(|sum| sum < &p) {
        println!("-1");
        return;
    }
    let inf = 1_usize << 60;
    let mut dp = vec![vec![vec![vec![vec![vec![inf; p + 1]; p + 1]; p + 1]; p + 1]; p + 1]; n + 1];
    dp[0][0][0][0][0][0] = 0_usize;
    for (i, (c, a)) in ca.iter().enumerate() {
        for i0 in 0..=p {
            for i1 in 0..=p {
                for i2 in 0..=p {
                    for i3 in 0..=p {
                        for i4 in 0..=p {
                            let v = dp[i][i0][i1][i2][i3][i4];
                            let ni0 = (i0 + a[0]).min(p);
                            let ni1 = (i1 + a[1]).min(p);
                            let ni2 = (i2 + a[2]).min(p);
                            let ni3 = (i3 + a[3]).min(p);
                            let ni4 = (i4 + a[4]).min(p);
                            chmin!(dp[i + 1][ni0][ni1][ni2][ni3][ni4], v + c);
                            chmin!(dp[i + 1][i0][i1][i2][i3][i4], v);
                        }
                    }
                }
            }
        }
    }
    let ans = dp[n][p][p][p][p][p];
    println!("{}", ans);
}
