use std::cmp::max;

use proconio::input;

fn divisors(n: usize) -> Vec<usize> {
    let mut d = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            d.push(i);
            if i != n / i {
                d.push(n / i);
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let ds = divisors(m);
    let mut max_d = 1;
    for d in ds {
        if d >= n {
            max_d = max(max_d, m / d);
        }
    }
    let ans = max_d;
    println!("{}", ans);
}
