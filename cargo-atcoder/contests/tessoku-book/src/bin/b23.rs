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
        xy: [(f64, f64); n],
    };
    let inf = 1e18;
    let mut min = inf;
    for start in 0..n {
        let mut dp = vec![vec![inf; n]; 1 << n];
        dp[1 << start][start] = 0_f64;
        for k in 0..1 << n {
            for i in 0..n {
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    let (x_i, y_i) = xy[i];
                    let (x_j, y_j) = xy[j];
                    let dist = ((x_i - x_j).powf(2_f64) + (y_i - y_j).powf(2_f64)).sqrt();
                    chmin!(dp[k | (1 << i)][j], dp[k][i] + dist);
                }
            }
        }

        let mut a = inf;
        for (i, d) in dp[(1 << n) - 1].iter().copied().enumerate() {
            let (x_i, y_i) = xy[i];
            let (x_j, y_j) = xy[start];
            let dist = ((x_i - x_j).powf(2_f64) + (y_i - y_j).powf(2_f64)).sqrt();
            chmin!(a, d + dist);
        }
        chmin!(min, a);
    }

    let ans = min;
    println!("{}", ans);
}
