// WA
use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n + 1],
    };
    if a[0] > 1 {
        println!("-1");
        return;
    }
    let mut c = 1_usize - a[0];
    let mut xs = vec![c];
    for i in 1..=n {
        if c == 0 {
            println!("-1");
            return;
        }
        let x = c.checked_mul(2);
        if x.is_none() {
            println!("-1");
            return;
        }
        let x = x.unwrap();
        if a[i] > x {
            println!("-1");
            return;
        }
        xs.push(x);
        c = x - a[i];
    }

    let mut c = a[n];
    let mut sum = c;
    for (i, &x_i) in xs.iter().enumerate().rev().skip(1) {
        c = min(c + a[i], x_i);
        sum += c;
    }
    let ans = sum;
    println!("{}", ans);
    todo!()
}
