use num::rational::Ratio;
use proconio::input;

fn f(x_i: i64, y_i: i64, x_j: i64, y_j: i64, x_a: i64, y_a: i64, x_b: i64, y_b: i64) -> bool {
    let denominator = x_j - x_i;
    if denominator == 0 {
        let s_a = x_a > x_i;
        let s_b = x_b > x_i;
        s_a != s_b
    } else {
        let a = Ratio::new(y_j - y_i, denominator);
        let b = Ratio::from_integer(y_i) - a * x_i;
        let s_a = (Ratio::from_integer(y_a) - a * x_a - b) > Ratio::from_integer(0);
        let s_b = (Ratio::from_integer(y_b) - a * x_b - b) > Ratio::from_integer(0);
        s_a != s_b
    }
}

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut count = 0;
    for i in 0..n {
        let j = if i == 0 { n - 1 } else { i - 1 };
        let (x_i, y_i) = xy[i];
        let (x_j, y_j) = xy[j];
        let (x_a, y_a) = a;
        let (x_b, y_b) = b;
        // 線分の交差判定
        if f(x_i, y_i, x_j, y_j, x_a, y_a, x_b, y_b) && f(x_a, y_a, x_b, y_b, x_i, y_i, x_j, y_j) {
            count += 1;
        }
    }
    let ans = count / 2 + 1;
    println!("{}", ans);
}
