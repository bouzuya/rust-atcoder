use proconio::input;

fn mul(a: &[Vec<usize>], b: &[Vec<usize>], modp: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut c = vec![vec![0_usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
                c[i][j] %= modp;
            }
        }
    }
    c
}

fn main() {
    input! {
        mut h: usize,
        r: usize,
        g: [[usize; r]; r],
    }

    let modp = 1_000_000_007_usize;
    let unit = {
        let mut unit = vec![vec![0_usize; r]; r];
        for i in 0..r {
            unit[i][i] = 1_usize;
        }
        unit
    };

    let mut m = vec![vec![0_usize; r]; r];
    for s in 0..r {
        let mut dp = vec![vec![0_usize; r]; 1 << r];
        dp[1 << s][s] = 1_usize;
        for bit in 0..1 << r {
            for i in 0..r {
                if (bit & (1 << i)) == 0 {
                    continue;
                }
                for j in 0..r {
                    if i == j || (bit & (1 << j)) == 0 {
                        continue;
                    }
                    if g[j][i] == 0 {
                        continue;
                    }
                    dp[bit][i] += dp[bit ^ (1 << i)][j];
                    dp[bit][i] %= modp;
                }
            }
        }

        for t in 0..r {
            for bit in 0..1 << r {
                m[s][t] += dp[bit][t];
                m[s][t] %= modp;
            }
        }
    }

    let mut res = unit.clone();
    let mut t = m;
    while h != 0 {
        if (h & 1) == 1 {
            res = mul(&res, &t, modp);
        }
        t = mul(&t, &t, modp);
        h >>= 1;
    }

    let ans = res[0][0];
    println!("{}", ans);
}
