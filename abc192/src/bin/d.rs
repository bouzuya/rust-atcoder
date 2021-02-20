use std::cmp::min;

use proconio::{input, marker::Chars};

fn f(x: &Vec<char>, y: u128) -> Option<u128> {
    let mut sum = 0_u128;
    for i in 0..x.len() {
        let d = (x[i] as u8 - '0' as u8) as u128;
        let e = x.len() - 1 - i;
        sum = match y.checked_pow(e as u32) {
            None => return None,
            Some(z) => match z.checked_mul(d) {
                None => return None,
                Some(z) => match sum.checked_add(z) {
                    None => return None,
                    Some(z) => z,
                },
            },
        };
    }
    Some(sum)
}

fn main() {
    input! {
        x: Chars,
        m: u128,
    };

    let d = (*x.iter().max().unwrap() as u8 - '0' as u8) as u128;

    let r = f(&x, d + 1);
    if r.is_none() || r.unwrap() > m {
        println!("0");
        return;
    }

    let mut ok = d + 1;
    let mut ng = m + 1;
    while ng - ok > 1 {
        let n = ok + (ng - ok) / 2;
        let r = f(&x, n);
        if r.is_some() && r.unwrap() <= m {
            ok = n;
        } else {
            ng = n;
        }
    }

    if x.len() == 1 {
        println!("1");
    } else {
        let ans = ok - d;
        println!("{}", ans);
    }
}
