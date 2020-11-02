use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        l_oa: i64,
        l_ab: i64,
        l_bc: i64,
    };
    let r1 = l_oa + l_ab + l_bc;
    let a1 = (r1 * r1) as f64 * std::f64::consts::PI;
    let r2 = if l_oa + l_ab > l_bc && l_oa + l_bc > l_ab && l_ab + l_bc > l_oa {
        0
    } else {
        min(
            (l_oa + l_ab - l_bc).abs(),
            min((l_oa - l_ab + l_bc).abs(), (l_oa - l_ab - l_bc).abs()),
        )
    };
    let a2 = (r2 * r2) as f64 * std::f64::consts::PI;
    let ans = a1 - a2;
    println!("{}", ans);
}
