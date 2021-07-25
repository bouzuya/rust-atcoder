use proconio::{input, marker::Chars};

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
        h: usize,
        w: usize,
        c: [Chars; h],
    };
    let inf = h * w + 1;
    let mut dp = vec![vec![inf; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h - 1 && j == w - 1 {
                dp[h - 1][w - 1] = if c[h - 1][w - 1] == 'E' { 0 } else { 1 };
            } else {
                if i + 1 < h {
                    chmin!(dp[i][j], dp[i + 1][j] + if c[i][j] == 'E' { 1 } else { 0 });
                }
                if j + 1 < w {
                    chmin!(dp[i][j], dp[i][j + 1] + if c[i][j] == 'E' { 0 } else { 1 });
                }
            }
        }
    }
    let ans = dp[0][0];
    println!("{}", ans);
}
