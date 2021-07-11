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
        p: i64,
        k: usize,
        a: [[i64; n]; n],
    };

    let f = |x: i64| {
        let mut d = vec![vec![x; n]; n];
        for i in 0..n {
            for j in 0..n {
                if a[i][j] != -1 {
                    d[i][j] = a[i][j];
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    chmin!(d[i][j], d[i][k] + d[k][j]);
                }
            }
        }

        let mut count = 0;
        for i in 0..n {
            for j in i + 1..n {
                if d[i][j] <= p {
                    count += 1;
                }
            }
        }
        count
    };

    let inf = 1_000_000_000_000;

    let mut ng = 0;
    let mut ok = inf;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        if f(x) <= k {
            ok = x;
        } else {
            ng = x;
        }
    }
    let res_l = ok;

    let mut ng = 0;
    let mut ok = inf;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        if f(x) < k {
            ok = x;
        } else {
            ng = x;
        }
    }

    let res_r = ok;

    let ans = match (res_l == inf, res_r == inf) {
        (true, _) => "0".to_string(),
        (false, true) => "Infinity".to_string(),
        (false, false) => format!("{}", res_r - res_l),
    };

    println!("{}", ans);
}
