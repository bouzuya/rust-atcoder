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
        h: usize,
        x: [usize; n],
        pf: [(usize, usize); n - 1],
    };

    let x = vec![0].into_iter().chain(x.into_iter()).collect::<Vec<_>>();
    let pf = pf
        .into_iter()
        .chain(vec![(0, 0)].into_iter())
        .collect::<Vec<_>>();

    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; h + 1]; h + 1];
    for i in 0..=h {
        dp[h][i] = 0_usize;
    }

    for (i, (p, f)) in pf.iter().copied().enumerate() {
        let mut next = vec![vec![inf; h + 1]; h + 1];
        let dx = x[i + 1] - x[i];
        for j in 0..=h {
            for k in 0..=h {
                if j < dx || k + dx > h {
                    continue;
                }
                let nj = j - dx;
                let nk = k + dx;
                chmin!(next[nj][nk], dp[j][k]);
                chmin!(next[(nj + f).min(h)][nk], dp[j][k] + p);
                if nk >= f {
                    chmin!(next[nj][nk - f], dp[j][k] + p);
                }
                if nk == h {
                    for l in 0..f {
                        chmin!(next[nj][nk - l], dp[j][k] + p);
                    }
                }
            }
        }
        dp = next;
    }

    let mut ans = inf;
    for i in 0..=h {
        ans = ans.min(dp[i][i]);
    }
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
