use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        apx: [(i64, i64, i64); n],
    };
    let inf = 1_000_000_000_000;
    let mut min_p = inf;
    for (a_i, p_i, x_i) in apx {
        if a_i >= x_i {
            continue;
        }
        min_p = min(min_p, p_i);
    }
    let ans = if min_p == inf { -1 } else { min_p };
    println!("{}", ans);
}
