use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [[i64; w];h],
    };
    let mut c_b = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            c_b[i][j] = if (i + j) % 2 == 0 { 0 } else { c[i][j] };
        }
    }
    let mut c_w = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            c_w[i][j] = if (i + j) % 2 != 0 { 0 } else { c[i][j] };
        }
    }
    let mut s_b = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            s_b[i + 1][j + 1] = c_b[i][j] + s_b[i + 1][j] + s_b[i][j + 1] - s_b[i][j];
        }
    }
    let mut s_w = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            s_w[i + 1][j + 1] = c_w[i][j] + s_w[i + 1][j] + s_w[i][j + 1] - s_w[i][j];
        }
    }

    let mut max_count = 0;
    for i_t in 0..=h {
        for i_b in i_t + 1..=h {
            for i_l in 0..=w {
                for i_r in i_l + 1..=w {
                    let b = s_b[i_b][i_r] - s_b[i_b][i_l] - s_b[i_t][i_r] + s_b[i_t][i_l];
                    let w = s_w[i_b][i_r] - s_w[i_b][i_l] - s_w[i_t][i_r] + s_w[i_t][i_l];
                    if b == w {
                        max_count = max(max_count, (i_b - i_t) * (i_r - i_l));
                    }
                }
            }
        }
    }

    let ans = max_count;
    println!("{}", ans);
}
