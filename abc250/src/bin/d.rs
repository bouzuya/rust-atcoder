use proconio::input;
use superslice::Ext;

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut p = vec![];
    let mut b = vec![true; n + 1];
    for i in 2.. {
        if i * i > n {
            for j in i..=n {
                if b[j] {
                    p.push(j);
                }
            }
            break;
        }
        if b[i] {
            p.push(i);
            for j in (i + i..=n).step_by(i) {
                b[j] = false;
            }
        }
    }
    p
}

fn main() {
    input! {
        n: usize,
    };
    let x = 1_000_000;
    let ps = sieve_of_eratosthenes(x);
    let mut count = 0_usize;
    for q in ps.iter().copied() {
        let q3 = q * q * q;
        if q3 > n {
            break;
        }
        let i = ps.upper_bound(&((n + q3 - 1) / q3));
        let i = if i + 1 < ps.len() {
            if ps[i + 1] * q3 <= n {
                i + 1
            } else {
                i
            }
        } else {
            i
        };
        let i = if i > 0 {
            if ps[i - 1] * q3 > n {
                i - 1
            } else {
                i
            }
        } else {
            i
        };
        count += i.min(ps.upper_bound(&(q - 1)));
    }
    let ans = count;
    println!("{}", ans);
}
