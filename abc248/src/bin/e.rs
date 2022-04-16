// TLE
use std::collections::BTreeSet;

use num::rational::Rational64;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n],
    };
    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut set = BTreeSet::new();
    let mut ans = 0_usize;
    for i in 0..n {
        for j in i + 1..n {
            let (x_i, y_i) = xy[i];
            let (x_j, y_j) = xy[j];
            let count = if x_j - x_i == 0 {
                if !set.insert((
                    0,
                    Rational64::from_integer(x_i),
                    Rational64::from_integer(0),
                )) {
                    continue;
                }
                let mut count = 0_usize;
                for k in 0..n {
                    let (x_k, _) = xy[k];
                    if x_k == x_i {
                        count += 1;
                    }
                }
                count
            } else {
                let a = Rational64::new(y_j - y_i, x_j - x_i);
                let b = Rational64::from_integer(y_i) - a * Rational64::from_integer(x_i);
                if !set.insert((1, a, b)) {
                    continue;
                }
                let mut count = 0_usize;
                for k in 0..n {
                    let (x_k, y_k) = xy[k];
                    if a * x_k + b == Rational64::from_integer(y_k) {
                        count += 1;
                    }
                }
                count
            };
            if count >= k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
