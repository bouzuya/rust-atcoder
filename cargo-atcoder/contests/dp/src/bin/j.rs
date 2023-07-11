use proconio::input;

fn f(dp: &mut Vec<Vec<Vec<Option<f64>>>>, n: usize, c: (usize, usize, usize)) -> f64 {
    if let Some(x) = dp[c.0][c.1][c.2] {
        return x;
    }

    let m = n as f64;

    let (c1, c2, c3) = c;
    let c0 = n - (c1 + c2 + c3);

    if c0 == n {
        return 0_f64;
    }

    let p0 = (c0 as f64) / m;
    let p1 = (c1 as f64) / m;
    let p2 = (c2 as f64) / m;
    let p3 = (c3 as f64) / m;

    let f0 = 1_f64;
    let f1 = if c1 > 0 {
        f(dp, n, (c1 - 1, c2, c3)) * p1
    } else {
        0_f64
    };
    let f2 = if c2 > 0 {
        f(dp, n, (c1 + 1, c2 - 1, c3)) * p2
    } else {
        0_f64
    };
    let f3 = if c3 > 0 {
        f(dp, n, (c1, c2 + 1, c3 - 1)) * p3
    } else {
        0_f64
    };

    let res = (f0 + f1 + f2 + f3) / (1_f64 - p0);

    dp[c.0][c.1][c.2] = Some(res);

    res
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut c = (0, 0, 0);
    for a_i in a {
        match a_i {
            1 => c.0 += 1,
            2 => c.1 += 1,
            3 => c.2 += 1,
            _ => unreachable!(),
        }
    }
    let mut dp = vec![vec![vec![None; n + 1]; n + 1]; n + 1];
    let ans = f(&mut dp, n, c);
    println!("{}", ans);
}
