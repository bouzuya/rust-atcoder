use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        mut c: [i64; n],
        q: usize,
    };
    let n_even = n / 2 + if n % 2 == 0 { 0 } else { 1 };

    let inf = 1_000_000_000_i64;
    let mut min_all = inf;
    let mut min_odd = inf;
    for (i, &c_i) in c.iter().enumerate() {
        if i % 2 == 0 {
            min_odd = min(min_odd, c_i);
        }
        min_all = min(min_all, c_i);
    }

    let mut count_all = 0;
    let mut count_odd = 0;
    let mut count_sum = 0;

    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! { x: Usize1, a: i64 };
                let cx = c[x] - if x % 2 == 0 { count_odd } else { 0 } - count_all;
                if cx >= a {
                    c[x] -= a;
                    count_sum += a;
                    if x % 2 == 0 {
                        min_odd = min(min_odd, cx - a);
                        min_all = min(min_all, min_odd);
                    } else {
                        min_all = min(min_all, cx - a);
                    }
                };
            }
            2 => {
                input! { a: i64 };
                if min_odd >= a {
                    min_odd -= a;
                    min_all = min(min_all, min_odd);
                    count_odd += a;
                    count_sum += a * n_even as i64;
                }
            }
            3 => {
                input! { a: i64 };
                if min_all >= a {
                    min_all -= a;
                    min_odd -= a;
                    count_all += a;
                    count_sum += a * n as i64;
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = count_sum;
    println!("{}", ans);
}
