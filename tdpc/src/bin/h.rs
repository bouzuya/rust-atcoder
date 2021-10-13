use proconio::{input, marker::Usize1};

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        w: usize,
        c: usize,
        wvc: [(usize, usize, Usize1); n],
    };
    let nc = 50;

    let mut wv = vec![vec![]; nc + 1];
    for (w_i, v_i, c_i) in wvc {
        wv[c_i].push((w_i, v_i));
    }

    let mut dp = vec![vec![0; w + 1]; c + 1];
    for wv_i in wv {
        let mut d = dp.clone();
        for d_j in d.iter_mut() {
            for (w_ik, v_ij) in wv_i.iter().copied() {
                for l in (w_ik..=w).rev() {
                    chmax!(d_j[l], d_j[l - w_ik] + v_ij);
                }
            }
        }
        for j in 0..c {
            for k in 0..=w {
                chmax!(dp[j + 1][k], d[j][k]);
            }
        }
    }
    let ans = dp[c][w];
    println!("{}", ans);
}
