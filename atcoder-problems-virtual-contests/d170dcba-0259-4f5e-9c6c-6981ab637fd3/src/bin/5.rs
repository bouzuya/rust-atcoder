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

fn is_ok(abcde: &[(usize, usize, usize, usize, usize)], x: usize) -> bool {
    let inf = abcde.len() + 1;
    let mut dp = vec![inf; 1 << 5];
    dp[0] = 0;
    for (a_i, b_i, c_i, d_i, e_i) in abcde.iter().copied() {
        let mut mask = 0;
        mask |= if a_i > x { 1 << 4 } else { 0 };
        mask |= if b_i > x { 1 << 3 } else { 0 };
        mask |= if c_i > x { 1 << 2 } else { 0 };
        mask |= if d_i > x { 1 << 1 } else { 0 };
        mask |= if e_i > x { 1 << 0 } else { 0 };
        for j in 0..1 << 5 {
            chmin!(dp[j | mask], dp[j] + 1);
        }
    }
    dp[0b11111] <= 3
}

fn main() {
    input! {
        n: usize,
        abcde: [(usize, usize, usize, usize, usize); n],
    }
    let mut ok = 0;
    let mut ng = 1_000_000_001;
    while ng - ok > 1 {
        let x = (ok + ng) / 2;
        if is_ok(&abcde, x) {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok + 1;
    println!("{}", ans);
}
