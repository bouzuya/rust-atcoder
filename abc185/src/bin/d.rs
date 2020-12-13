use proconio::input;
use std::cmp::min;

fn f(x: usize, k: usize) -> usize {
    (x + k - 1) / k
}

fn main() {
    input! {
        n: usize,
        mut m: usize,
        mut a: [usize; m],
    };
    if m == 0 {
        println!("1");
        return;
    }
    if m >= n {
        println!("0");
        return;
    }
    a.sort();
    a.push(n + 1);
    a.push(0);
    a.sort();

    let mut k = n;
    let mut p = a[0];
    for &a_i in a.iter() {
        if a_i - p <= 1 {
            p = a_i;
            continue;
        }
        k = min(k, a_i - p - 1);
        p = a_i;
    }
    let mut count = 0;
    let mut p = a[0];
    for &a_i in a.iter() {
        if a_i - p <= 1 {
            p = a_i;
            continue;
        }
        count += f(a_i - p - 1, k);
        p = a_i;
    }
    let ans = count;
    println!("{}", ans);
}
