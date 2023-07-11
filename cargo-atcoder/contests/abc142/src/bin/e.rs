use proconio::{input, marker::Usize1};

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
        m: usize,
    };
    let mut abc = vec![];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: [Usize1; b]
        }
        let mut mask = 0_usize;
        for c_i in c {
            mask |= 1 << c_i;
        }
        abc.push((a, b, mask));
    }

    let inf = 1_000_000_000;
    let mut dp = vec![vec![inf; 1 << 12 + 1]; m + 1];
    dp[0][0] = 0;
    for (i, &(a_i, _, c_i)) in abc.iter().enumerate() {
        for j in 0..1 << n {
            chmin!(dp[i + 1][j], dp[i][j]);
            chmin!(dp[i + 1][j | c_i], dp[i][j] + a_i);
        }
    }

    let ans = dp[m][(0..n).fold(0, |acc, x| acc | 1 << x)];
    println!("{}", if ans == inf { -1 } else { ans as i64 });
}
