use std::f64::consts::PI;

use proconio::input;

fn f(a: f64, b: f64, theta: f64) -> f64 {
    let eps = 0.0000000001_f64;
    if theta > PI / 2_f64 - eps {
        return 0_f64;
    }
    let t = theta.tan();
    if t * a <= b {
        a * a * b - a * a * a * t / 2_f64
    } else {
        b * b / t * a / 2_f64
    }
}

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    };
    let mut ok = PI / 2_f64;
    let mut ng = 0_f64;
    for _ in 1..=1_000_000 {
        let theta = (ok + ng) / 2_f64;
        if f(a, b, theta) < x {
            ok = theta;
        } else {
            ng = theta;
        }
    }
    let ans = ok * 180_f64 / PI;
    println!("{:.10}", ans);
}
