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
        m: (usize, usize),
        abc: [(usize, usize, usize); n],
    };

    let inf = 1 << 40;
    let mut dp = vec![vec![inf; 400 + 1]; 400 + 1];
    dp[0][0] = 0;
    for (a_i, b_i, c_i) in abc.iter().copied() {
        let mut tmp = vec![vec![inf; 400 + 1]; 400 + 1];
        for i in 0..=400 {
            for j in 0..=400 {
                chmin!(tmp[i][j], dp[i][j]);
                if i + a_i <= 400 && j + b_i <= 400 {
                    chmin!(tmp[i + a_i][j + b_i], dp[i][j] + c_i);
                }
            }
        }
        dp = tmp;
    }
    let mut ans = inf;
    for i in 0..=400 {
        for j in 0..=400 {
            if ((i != 0) || (j != 0)) && (i * m.1 == j * m.0) {
                chmin!(ans, dp[i][j]);
            }
        }
    }
    println!("{}", if ans == inf { -1 } else { ans as i64 });
}
