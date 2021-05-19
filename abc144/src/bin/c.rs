use std::cmp;

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
    };
    let mut min_v = 1_000_000_000_000_000_usize;
    let ds = divisors(n);
    for i in ds {
        let j = n / i;
        min_v = cmp::min(min_v, i + j - 2);
    }
    let ans = min_v;
    println!("{}", ans);
}
