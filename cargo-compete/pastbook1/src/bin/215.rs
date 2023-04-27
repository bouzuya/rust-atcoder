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
        n: usize,
        m: usize,
        sc: [(Chars, usize); m],
    }
    let mut tc = vec![];
    for (s, c) in sc {
        let mut t = 0_usize;
        for i in 0..n {
            if s[i] == 'Y' {
                t |= 1 << (n - i - 1);
            }
        }
        tc.push((t, c));
    }

    let inf = 1_usize << 60;
    let mut dp = vec![vec![inf; 1 << n]; m + 1];
    dp[0][0] = 0_usize;
    for (i, (t, c)) in tc.iter().copied().enumerate() {
        for bits in 0..1 << n {
            chmin!(dp[i + 1][bits], dp[i][bits]);
            chmin!(dp[i + 1][bits | t], dp[i][bits] + c);
        }
    }
    let ans = dp[m][(1 << n) - 1];
    println!("{}", if ans == inf { -1 } else { ans as i64 });
}
