// 解説 AC
use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        ta: [(i64, i64); n],
    };
    let mut t = 1;
    let mut a = 1;
    for (t_i, a_i) in ta {
        let x = max((t + t_i - 1) / t_i, (a + a_i - 1) / a_i);
        t = x * t_i;
        a = x * a_i;
    }
    let ans = t + a;
    println!("{}", ans);
}
