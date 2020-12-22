use proconio::input;
use std::cmp::min;

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
    d.sort();
    d
}

fn main() {
    input! {
        n: usize,
    };
    let mut min_x = n;
    let d = divisors(n);
    for a in d {
        let b = n / a;
        assert_eq!(a * b, n);
        min_x = min(min_x, a.saturating_sub(1) + b.saturating_sub(1));
    }
    let ans = min_x;
    println!("{}", ans);
}
