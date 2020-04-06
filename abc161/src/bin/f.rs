use proconio::input;
use std::collections::BTreeSet;

fn divisors(n: i64) -> Vec<i64> {
    let mut dv = vec![];
    dv.push(1);
    for i in 2..n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            dv.push(i);
            if n / i != i {
                dv.push(n / i);
            }
        }
    }
    dv.push(n);
    dv
}

fn main() {
    input! {
        n: i64
    };

    let mut set = BTreeSet::new();

    for &k in divisors(n).iter() {
        if k == 1 {
            continue;
        }
        let mut m = n;
        while m % k == 0 {
            m /= k;
        }
        if m % k == 1 {
            set.insert(k);
        }
    }

    for &k in divisors(n - 1).iter() {
        if k == 1 {
            continue;
        }
        set.insert(k);
    }

    println!("{}", set.len());
}
