use std::cmp::max;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
        b: Chars,
    };

    let x_a = {
        let mut a = a.clone();
        for i in 0..3 {
            if a[i] != '9' {
                a[i] = '9';
                break;
            }
        }
        let a = a.iter().collect::<String>().parse::<i64>().unwrap();
        let b = b.iter().collect::<String>().parse::<i64>().unwrap();
        a - b
    };
    let x_b = {
        let mut b = b.clone();
        for i in 0..3 {
            if i == 0 && b[i] != '1' {
                b[i] = '1';
                break;
            } else if i != 0 && b[i] != '0' {
                b[i] = '0';
                break;
            }
        }
        let a = a.iter().collect::<String>().parse::<i64>().unwrap();
        let b = b.iter().collect::<String>().parse::<i64>().unwrap();
        a - b
    };
    let ans = max(x_a, x_b);
    println!("{}", ans);
}
