use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        mut c: [i64; n],
        q: usize,
    };
    let mut min_set = 1_000_000_000_000;
    let mut min_all = 1_000_000_000_000;
    for (i, &c_i) in c.iter().enumerate() {
        if i % 2 == 0 {
            min_set = std::cmp::min(min_set, c_i);
        }
        min_all = std::cmp::min(min_all, c_i);
    }
    let mut count = 0;
    let mut count_set = 0;
    let mut count_all = 0;
    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! {
                    x: Usize1,
                    a: i64,
                };
                if c[x] - if x % 2 == 0 { count_set } else { 0 } - count_all >= a {
                    count += a;
                    c[x] -= a;
                    let c_new = c[x] - if x % 2 == 0 { count_set } else { 0 } - count_all;
                    if x % 2 == 0 {
                        min_set = std::cmp::min(min_set, c_new);
                    }
                    min_all = std::cmp::min(min_all, c_new);
                }
            }
            2 => {
                input! { a: i64 };
                if min_set >= a {
                    count += a * ((n as i64 + 1) / 2);
                    count_set += a;
                    min_set -= a;
                    min_all = std::cmp::min(min_all, min_set);
                }
            }
            3 => {
                input! { a: i64 };
                if min_all >= a {
                    count += a * n as i64;
                    count_all += a;
                    min_set -= a;
                    min_all -= a;
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = count;
    println!("{}", ans);
}
