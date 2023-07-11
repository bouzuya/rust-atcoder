use proconio::input;

fn probability(r_p: i64, r_q: i64) -> f64 {
    1_f64 / (1_f64 + 10_f64.powf((r_q - r_p) as f64 / 400_f64))
}

fn main() {
    input! {
        k: usize,
        r: [i64; 1 << k],
    };

    let mut dp = vec![vec![0_f64; r.len()]; k + 1];
    for p_i in dp[0].iter_mut() {
        *p_i = 1_f64;
    }
    for i_r in 0..k {
        let step = 1 << (i_r + 1);
        for o in (0..r.len()).step_by(step) {
            for i in 0..step / 2 {
                for j in step / 2..step {
                    let (i_p, i_q) = (o + i, o + j);
                    let p_w = probability(r[i_p], r[i_q]);
                    let p_m = dp[i_r][i_p] * dp[i_r][i_q];
                    dp[i_r + 1][i_p] += p_w * p_m;
                    dp[i_r + 1][i_q] += (1_f64 - p_w) * p_m;
                }
            }
        }
    }

    for &p in dp[k].iter() {
        println!("{}", p);
    }
}
