// 解説 AC <http://logfiles.hatenablog.com/entry/20160523/1464007274>
use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        q: usize,
        xrh: [(i64, i64, i64); n],
        ab: [(i64, i64); q],
    };
    let pi = std::f64::consts::PI;
    let v = xrh
        .iter()
        .map(|&(_, r_i, h_i)| r_i as f64 * r_i as f64 * pi * h_i as f64 / 3_f64)
        .collect::<Vec<f64>>();
    for (a_i, b_i) in ab {
        let mut v_i = 0_f64;
        for (&(x_j, _, h_j), &v_j) in xrh.iter().zip(v.iter()) {
            if a_i <= x_j + h_j {
                let r = (x_j + h_j - max(a_i, x_j)) as f64 / h_j as f64;
                v_i += v_j * r * r * r;
            }
            if b_i <= x_j + h_j {
                let r = (x_j + h_j - max(b_i, x_j)) as f64 / h_j as f64;
                v_i -= v_j * r * r * r;
            }
        }
        println!("{}", v_i);
    }
}
